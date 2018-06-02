// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the APACHE2.0/MIT licence that can be
// found in the LICENCE-{APACHE/MIT} file.

use std::borrow::Cow;
use std::collections::{btree_map, BTreeMap};
use std::fs;
use std::iter::IntoIterator;
use std::mem;
use std::ops::{AddAssign, Deref, DerefMut};

use encoding;
use encoding::all::UTF_8;
use encoding::DecoderTrap::Replace;
use log::Level::Trace;
use rayon::prelude::*;

#[cfg(feature = "io")] use serde;

use stats::Stats;
use super::LanguageType::*;
use super::{Language, LanguageType};
use utils;

fn count_files((name, ref mut language): (&LanguageType, &mut Language)) {

    let files = mem::replace(&mut language.files, Vec::new());
    let is_blank = language.is_blank();
    let is_fortran = name == &FortranModern || name == &FortranLegacy;
    let nested_is_empty = language.nested_comments.is_empty();

    let stats: Vec<_> = files.into_par_iter().filter_map(|file| {
        let mut stack: Vec<&'static str> = Vec::with_capacity(1);
        let mut quote: Option<&'static str> = None;

        let contents = rs_ret_error!(fs::read(&file));

        let text = match encoding::decode(&contents, Replace, UTF_8) {
            (Ok(string), _) => Cow::Owned(string),
            (Err(cow), _) => cow,
        };

        let lines = text.lines();
        let mut stats = Stats::new(file);

        if is_blank {
            let count = lines.count();
            stats.lines += count;
            stats.code += count;
            return Some(stats);
        }

        for line in lines {

            stats.lines += 1;
            if line.chars().all(char::is_whitespace) {
                stats.blanks += 1;
                trace!("Blank line. So far: {}", stats.blanks);
                continue;
            }

            // FORTRAN has a rule where it only counts as a comment if it's the
            // first character in the column, so removing starting whitespace
            // could cause a miscount.
            let line = if !is_fortran { line.trim() } else { line };
            let mut ended_with_comments = false;
            let mut had_code = stack.is_empty();
            let mut skip = 0;
            macro_rules! skip {
                ($skip:expr) => {{
                    skip = $skip - 1;
                }}
            }

            if quote.is_none() && stack.is_empty() {

                let mut iter = language.quotes.iter()
                                          .chain(language.multi_line)
                                          .chain(language.nested_comments);

                if !iter.any(|(s, _)| line.contains(s)) {
                    trace!(r#"Determined to be skippable"#);
                    if language.line_comment.iter()
                                            .any(|s| line.starts_with(s))
                    {
                        stats.comments += 1;
                        trace!("Determined to be comment. So far: {} lines", stats.comments);
                    } else {
                        stats.code += 1;
                        trace!("Determined to be code. So far: {} lines", stats.code);
                    }
                    trace!("{}", line);
                    continue;
                }
            }

            'window: for i in 0..line.len() {
                if skip != 0 {
                    skip -= 1;
                    continue;
                }

                ended_with_comments = false;
                let line = line.as_bytes();
                let window = &line[i..];

                if let Some(quote_str) = quote {
                    if window.starts_with(br"\") {
                        skip = 1;
                    } else if window.starts_with(quote_str.as_bytes()) {
                        quote = None;
                        trace!(r#"End of "{}"."#, quote_str);
                        skip!(quote_str.len());
                    }
                    continue;
                }

                if let Some(true) = stack.last()
                    .and_then(|l| Some(window.starts_with(l.as_bytes())))
                {
                    let last = stack.pop().unwrap();
                    ended_with_comments = true;

                    if log_enabled!(Trace) && stack.is_empty() {
                        trace!(r#"End of "{}"."#, last);
                    } else {
                        trace!(r#"End of "{}". Still in comments."#, last);
                    }

                    skip!(last.len());
                    continue;
                }

                if stack.is_empty() {
                    for comment in language.line_comment {
                        if window.starts_with(comment.as_bytes()) {
                            trace!(r#"Start of "{}"."#, comment);
                            break 'window;
                        }
                    }

                    for &(start, end) in language.quotes {
                        if window.starts_with(start.as_bytes()) {
                            quote = Some(end);
                            trace!(r#"Start of "{}"."#, start);
                            skip!(start.len());
                            continue 'window;
                        }
                    }
                }

                for &(start, end) in language.nested_comments {
                    if window.starts_with(start.as_bytes()) {
                        stack.push(end);
                        trace!(r#"Start of "{}"."#, start);
                        skip!(start.len());
                        continue 'window;
                    }
                }

                for &(start, end) in language.multi_line {
                    if window.starts_with(start.as_bytes()) {
                        if (language.nested && nested_is_empty) ||
                            stack.is_empty()
                        {
                            trace!(r#"Start of nested "{}"."#, start);
                            stack.push(end);
                        }

                        skip!(start.len());
                        continue 'window;
                    }
                }
            }

            let starts_with_comment = language.multi_line.iter()
                .map(|&(s, _)| s)
                .chain(language.line_comment.iter().map(|s| *s))
                .chain(language.nested_comments.iter().map(|&(s, _)| s))
                .any(|comment| line.starts_with(comment));

            trace!("{}", line);

            if ((!stack.is_empty() || ended_with_comments) && !had_code) ||
                (starts_with_comment && quote.is_none())
            {
                stats.comments += 1;
                trace!("Determined to be comment. So far: {} lines", stats.comments);
                trace!("Did it have code?: {}", had_code);
            } else {
                stats.code += 1;
                trace!("Determined to be code. So far: {} lines", stats.code);
            }
        }

        Some(stats)
    }).collect();

    for stat in stats {
        **language += stat;
    }
}


/// A collection of existing languages([_List of Languages_](https://github.com/Aaronepower/tokei#supported-languages))
#[derive(Default)]
pub struct Languages {
    inner: BTreeMap<LanguageType, Language>,
}

#[cfg(feature = "io")]
impl serde::Serialize for Languages {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let map = self.remove_empty();
        map.serialize(serializer)
    }
}

#[cfg(feature = "io")]
impl<'de> serde::Deserialize<'de> for Languages {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let map = <_>::deserialize(deserializer)?;

        Ok(Self::from_previous(map))
    }
}

impl Languages {
    #[cfg(feature = "io")]
    fn from_previous(map: BTreeMap<LanguageType, Language>) -> Self {
        use std::collections::btree_map::Entry::*;
        let mut _self = Self::new();

        for (name, input_language) in map {
            match _self.entry(name) {
                Occupied(mut entry) => {
                    *entry.get_mut() += input_language;
                }
                Vacant(entry) => {
                    entry.insert(input_language);
                }
            }
        }
        _self
    }

    /// Get statistics from the list of paths provided, and a list ignored
    /// keywords to ignore paths containing them.
    ///
    /// ```no_run
    /// # use tokei::*;
    /// let mut languages = Languages::new();
    /// languages.get_statistics(&["."], vec![".git", "target"]);
    /// ```
    pub fn get_statistics(&mut self, paths: &[&str], ignored: Vec<&str>) {
        utils::fs::get_all_files(paths, ignored, &mut self.inner);
        self.inner.par_iter_mut().for_each(count_files);
    }

    /// Constructs a new, blank `Languages`.
    ///
    /// ```
    /// # use tokei::*;
    /// let languages = Languages::new();
    /// ```
    pub fn new() -> Self {
        Languages::default()
    }

    /// Creates a new map that only contains non empty languages.
    ///
    /// ```
    /// use tokei::*;
    /// use std::collections::BTreeMap;
    ///
    /// let mut languages = Languages::new();
    /// languages.get_statistics(&["doesnt/exist"], vec![".git"]);
    ///
    /// let empty_map = languages.remove_empty();
    ///
    /// assert_eq!(empty_map.len(), 0);
    /// ```
    pub fn remove_empty(&self) -> BTreeMap<&LanguageType, &Language> {
        let mut map = BTreeMap::new();

        for (name, language) in &self.inner {
            if !language.is_empty() {
                map.insert(name, language);
            }
        }
        map
    }
}

impl IntoIterator for Languages {
    type Item = <BTreeMap<LanguageType, Language> as IntoIterator>::Item;
    type IntoIter =
        <BTreeMap<LanguageType, Language> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Languages {
    type Item = (&'a LanguageType, &'a Language);
    type IntoIter = btree_map::Iter<'a, LanguageType, Language>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut Languages {
    type Item = (&'a LanguageType, &'a mut Language);
    type IntoIter = btree_map::IterMut<'a, LanguageType, Language>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

impl AddAssign<BTreeMap<LanguageType, Language>> for Languages {
    fn add_assign(&mut self, rhs: BTreeMap<LanguageType, Language>) {

        for (name, language) in rhs {

            if let Some(result) = self.inner.get_mut(&name) {
                *result += language;
            }
        }
    }
}

impl Deref for Languages {
    type Target = BTreeMap<LanguageType, Language>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Languages {
    fn deref_mut(&mut self) -> &mut BTreeMap<LanguageType, Language> {
        &mut self.inner
    }
}
