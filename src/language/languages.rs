// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the APACHE2.0/MIT licence that can be
// found in the LICENCE-{APACHE/MIT} file.
#![allow(unused_variables)]

use std::borrow::Cow;
use std::collections::{btree_map, BTreeMap};
use std::fs::File;
use std::io::Read;
use std::iter::IntoIterator;
use std::ops::{AddAssign, Deref, DerefMut};
use std::sync::{mpsc, Mutex};
use std::{cmp, mem};

use encoding;
use encoding::all::UTF_8;
use encoding::DecoderTrap::Replace;

#[cfg(feature = "cbor")] use serde_cbor;
#[cfg(feature = "json")] use serde_json;
#[cfg(feature = "yaml")] use serde_yaml;
#[cfg(feature = "toml")] use toml;
use rayon::prelude::*;

use stats::Stats;
use super::LanguageType::*;
use super::{Language, LanguageType};
use utils::fs;

fn count_files((name, ref mut language): (&LanguageType, &mut Language)) {

    let is_fortran = name == &FortranModern || name == &FortranLegacy;
    let (tx, rx) = mpsc::channel();
    let synced_tx = Mutex::new(tx);
    let has_multi_line = !language.multi_line.is_empty() ||
                         !language.nested_comments.is_empty();
    let is_blank = language.is_blank();
    let window_size = language.multi_line.iter()
        .chain(language.nested_comments.iter())
        .map(|&(a, b)| cmp::max(a.len(), b.len()))
        .max()
        .unwrap_or_else(|| {
            language.line_comment.iter()
                .map(|x| x.len())
                .max()
                .unwrap_or_else(|| {
                    language.quotes.iter()
                        .map(|&(a, b)| cmp::max(a.len(), b.len()))
                        .max()
                        .unwrap_or(usize::max_value())
                })
        });

    let nested_is_empty = language.nested_comments.is_empty();

    let files = mem::replace(&mut language.files, Vec::new());

    files.into_par_iter().for_each(|file| {
        let mut stack: Vec<&'static str> = Vec::new();
        let mut contents = Vec::new();
        let mut quote: Option<&'static str> = None;

        rs_ret_error!(rs_ret_error!(File::open(&file)).read_to_end(&mut contents));

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
            rs_ret_error!(rs_ret_error!(synced_tx.lock()).send(stats));
            return;
        }

        /*
        let should_handle_multi_line = has_multi_line && match &language.regex {
            &Some(ref regex) => regex.is_match(&text),
            &None => false,
        };
        */

        'line: for line in lines {
            let window_size = if window_size > line.len() {
                line.len()
            } else {
                window_size
            };

            stats.lines += 1;
            let no_stack = stack.is_empty();
            if line.chars().all(char::is_whitespace) {
                stats.blanks += 1;
                continue;
            }

            // FORTRAN has a rule where it only counts as a comment if it's the
            // first character in the column, so removing starting whitespace
            // could cause a miscount.
            let line = if !is_fortran { line.trim_left() } else { line };
            for single in &language.line_comment {
                if line.starts_with(single) {
                    stats.comments += 1;
                    continue 'line;
                }
            }

            let mut skip: u8 = 0;
            let windows = line.as_bytes().windows(window_size);

            let windows = if let (0, Some(0)) = windows.size_hint() {
                line.as_bytes().windows(line.len())
            } else {
                windows
            };

            'window: for window in windows {
                while skip != 0 {
                    skip -= 1;
                    continue 'window;
                }

                if let Some(quote_str) = quote {
                    if window.starts_with(br"\") {
                        skip = 1;
                    } else if window.starts_with(quote_str.as_bytes()) {
                        quote = None;
                        skip_by_str_length!(skip, quote_str);
                    }
                    continue;
                }


                let mut end = false;
                if let Some(last) = stack.last() {
                    if window.starts_with(last.as_bytes()) {
                        end = true;
                    }
                }

                if end {
                    let last = stack.pop().unwrap();
                    skip_by_str_length!(skip, last);
                    continue;
                }

                if stack.is_empty() {
                    for comment in &language.line_comment {
                        if window.starts_with(comment.as_bytes()) {
                            break 'window;
                        }
                    }

                    for &(start, end) in &language.quotes {
                        if window.starts_with(start.as_bytes()) {
                            quote = Some(end);
                            skip_by_str_length!(skip, start);
                            continue 'window;
                        }
                    }
                }

                for &(start, end) in &language.nested_comments {
                    if window.starts_with(start.as_bytes()) {
                        stack.push(end);
                        skip_by_str_length!(skip, start);
                        continue 'window;
                    }
                }

                for &(start, end) in &language.multi_line {
                    if window.starts_with(start.as_bytes()) {
                        if (language.nested && nested_is_empty) || stack.len() == 0 {
                            stack.push(end);
                        }
                        skip_by_str_length!(skip, start);
                        continue 'window;
                    }
                }
            }

            let starts_with_comment = language.multi_line.iter()
                .chain(&language.nested_comments)
                .any(|&(start, _)| line.starts_with(start));

            if no_stack && !starts_with_comment {
                stats.code += 1;
            } else {
                stats.comments += 1;
            }
        }

        rs_ret_error!(rs_ret_error!(synced_tx.lock()).send(stats));
    });

    drop(synced_tx);
    for stat in rx {
        **language += stat;
    }
}

/// A collection of existing languages([_List of Languages_]
/// (https://github.com/Aaronepower/tokei#supported-languages))
pub struct Languages {
    inner: BTreeMap<LanguageType, Language>,
}


impl Languages {
    /// Creates a `Languages` struct from cbor.
    ///
    /// ```no_run
    /// extern crate tokei;
    /// use tokei::*;
    /// extern crate hex;
    /// # fn main () {
    /// use hex::FromHex;
    /// let cbor = "a16452757374a666626c616e6b730564636f64650c68636f6d6d656e747\
    ///     30065737461747381a566626c616e6b730564636f64650c68636f6d6d656e747300\
    ///     656c696e657311646e616d65722e5c7372635c6c69625c6275696c642e7273656c6\
    ///     96e6573116b746f74616c5f66696c657301";
    ///
    /// let hex = Vec::from_hex(cbor).unwrap();
    ///
    /// let mut languages = Languages::from_cbor(&hex).unwrap();
    /// assert_eq!(12, languages.get_mut(&LanguageType::Rust).unwrap().code);
    /// # }
    /// ```
    #[cfg(feature = "cbor")]
    pub fn from_cbor(cbor: &[u8]) -> serde_cbor::Result<Self> {
        Ok(Self::from_previous(serde_cbor::from_slice(cbor.into())?))
    }

    #[cfg(not(feature = "cbor"))]
    pub fn from_cbor(_: &[u8]) -> Result<Self, ()> {
        Err(())
    }

    /// Creates a `Languages` struct from json.
    ///
    /// ```
    /// use tokei::*;
    /// let json = r#"{
    ///     "Rust": {
    ///         "blanks": 5,
    ///         "code": 12,
    ///         "comments": 0,
    ///         "stats": [
    ///             {
    ///                 "blanks": 5,
    ///                 "code": 12,
    ///                 "comments": 0,
    ///                 "lines": 17,
    ///                 "name": ".\\src\\lib\\build.rs"
    ///             }
    ///         ],
    ///         "lines": 17
    ///     }
    /// }"#;
    /// let mut languages = Languages::from_json(json.as_bytes()).unwrap();
    /// assert_eq!(12, languages.get_mut(&LanguageType::Rust).unwrap().code);
    /// ```
    #[cfg(feature = "json")]
    pub fn from_json(json: &[u8]) -> serde_json::Result<Self> {
        Ok(Self::from_previous(serde_json::from_slice(json.into())?))
    }

    #[cfg(not(feature = "json"))]
    pub fn from_json(_: &[u8]) -> Result<Self, ()> {
        Err(())
    }

    /// Creates a `Languages` struct from json.
    ///
    /// ```no_run
    /// # use tokei::*;
    /// let yaml = r#"\
    /// ---
    /// Rust:
    ///   blanks: 5
    ///   code: 12
    ///   comments: 0
    ///   lines: 17
    ///   stats:
    ///     -
    ///       blanks: 5
    ///       code: 12
    ///       comments: 0
    ///       lines: 17
    ///       name: .\src\lib\build.rs
    /// "#;
    ///
    /// let mut languages = Languages::from_yaml(yaml.as_bytes()).unwrap();
    ///
    /// assert_eq!(12, languages.get_mut(&LanguageType::Rust).unwrap().code);
    /// ```
    #[cfg(feature = "yaml")]
    pub fn from_yaml(yaml: &[u8]) -> serde_yaml::Result<Self> {
        Ok(Self::from_previous(serde_yaml::from_slice(yaml.into())?))
    }

    #[cfg(not(feature = "yaml"))]
    pub fn from_yaml(_: &[u8]) -> Result<Self, ()> {
        Err(())
    }

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
    /// languages.get_statistics(vec!["."], vec![".git", "target"]);
    /// ```
    pub fn get_statistics(&mut self, paths: Vec<&str>, ignored: Vec<&str>) {
        fs::get_all_files(paths.into(), ignored.into(), &mut self.inner);
        self.inner.par_iter_mut().for_each(count_files);
    }

    /// Constructs a new, blank `Languages`.
    ///
    /// ```
    /// # use tokei::*;
    /// let languages = Languages::new();
    /// ```
    pub fn new() -> Self {
        Languages { inner: BTreeMap::new() }
    }

    /// Creates a new map that only contains non empty languages.
    ///
    /// ```
    /// use tokei::*;
    /// use std::collections::BTreeMap;
    ///
    /// let mut languages = Languages::new();
    /// languages.get_statistics(vec!["doesnt/exist"], vec![".git"]);
    ///
    /// let empty_map = languages.remove_empty();
    ///
    /// assert_eq!(empty_map.len(), 0);
    /// ```
    pub fn remove_empty(self) -> BTreeMap<LanguageType, Language> {
        let mut map = BTreeMap::new();

        for (name, language) in self.inner {
            if !language.is_empty() {
                map.insert(name, language);
            }
        }
        map
    }

    /// Converts `Languages` to CBOR.
    ///
    /// ```no_run
    /// extern crate tokei;
    /// # use tokei::*;
    /// extern crate hex;
    /// use hex::ToHex;
    ///
    /// # fn main () {
    /// let cbor = "a16452757374a666626c616e6b730564636f64650c68636f6d6d656e747\
    ///     30065737461747381a566626c616e6b730564636f64650c68636f6d6d656e747300\
    ///     656c696e657311646e616d65722e5c7372635c6c69625c6275696c642e7273656c6\
    ///     96e6573116b746f74616c5f66696c657301";
    ///
    /// let mut languages = Languages::new();
    /// languages.get_statistics(vec!["build.rs"], vec![]);
    ///
    /// assert_eq!(cbor, &languages.to_cbor().unwrap().to_hex());
    /// # }
    /// ```
    #[cfg(feature = "cbor")]
    pub fn to_cbor(self) -> Result<Vec<u8>, serde_cbor::Error> {
        serde_cbor::to_vec(&self.remove_empty())
    }

    #[cfg(not(feature = "cbor"))]
    pub fn to_cbor(&self) -> Result<Vec<u8>, ()> {
        Err(())
    }

    /// Converts `Languages` to JSON.
    ///
    /// ```no_run
    /// # use tokei::*;
    ///
    /// let json = r#"{
    ///     "Rust": {
    ///         "blanks": 5,
    ///         "code": 12,
    ///         "comments": 0,
    ///         "stats": [
    ///             {
    ///                 "blanks": 5,
    ///                 "code": 12,
    ///                 "comments": 0,
    ///                 "lines": 17,
    ///                 "name": ".\\build.rs"
    ///             }
    ///         ],
    ///         "lines": 17
    ///     }
    /// }"#;
    /// let mut languages = Languages::new();
    /// languages.get_statistics(vec!["build.rs"], vec![]);
    ///
    /// assert_eq!(json, languages.to_json().unwrap());
    /// ```
    #[cfg(feature = "json")]
    pub fn to_json(self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.remove_empty())
    }

    #[cfg(not(feature = "json"))]
    pub fn to_json(&self) -> Result<String, ()> {
        Err(())
    }

    #[cfg(feature = "toml-io")]
    pub fn to_toml(self) -> Result<String, toml::ser::Error> {
        toml::to_string(&self.remove_empty())
    }

    #[cfg(not(feature = "toml-io"))]
    pub fn to_toml(&self) -> Result<String, ()> {
        Err(())
    }

    /// Converts `Languages` to YAML.
    ///
    /// ```no_run
    /// use tokei::*;
    ///
    /// let yaml = r#"
    ///     ---
    ///     "Rust":
    ///     "blanks": 5
    ///     "code": 12
    ///     "comments": 0
    ///     "lines": 17
    ///     "stats":
    ///         -
    ///         "blanks": 5
    ///         "code": 12
    ///         "comments": 0
    ///         "lines": 17
    ///         "name": ".\\build.rs"#;
    /// let mut languages = Languages::new();
    /// languages.get_statistics(vec!["build.rs"], vec![]);
    ///
    /// assert_eq!(yaml, languages.to_yaml().unwrap());
    #[cfg(feature = "yaml")]
    pub fn to_yaml(self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(&self.remove_empty())
    }

    #[cfg(not(feature = "yaml"))]
    pub fn to_yaml(&self) -> Result<String, ()> {
        Err(())
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
