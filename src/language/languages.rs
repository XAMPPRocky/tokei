// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the APACHE2.0/MIT licence that can be
// found in the LICENCE-{APACHE/MIT} file.

use std::borrow::Cow;
use std::collections::{btree_map, BTreeMap};
use std::fs::File;
use std::io::Read;
use std::iter::IntoIterator;
use std::ops::{AddAssign, Deref, DerefMut};
use std::sync::{mpsc, Mutex};

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
use utils::{fs, multi_line};

fn count_files(mut language_tuple: (&LanguageType, &mut Language)) {

    let (name, ref mut language) = language_tuple;

    if language.files.is_empty() {
        return;
    }

    let is_fortran = name == &FortranModern || name == &FortranLegacy;

    let (tx, rx) = mpsc::channel();
    let has_multi_line = !language.multi_line.is_empty() &&
        !language.nested_comments.is_empty();
    let synced_tx = Mutex::new(tx);
    let is_blank = language.is_blank();

    language.files.par_iter().for_each(|file| {
        let mut stats = Stats::new(
            opt_ret_error!(file.to_str(), "Couldn't convert path to String.")
        );
        let mut stack = Vec::new();
        let mut contents = Vec::new();
        let mut quote = None;

        rs_ret_error!(rs_ret_error!(File::open(file)).read_to_end(&mut contents));

        let text = match encoding::decode(&contents, Replace, UTF_8) {
            (Ok(string), _) => Cow::Owned(string),
            (Err(cow), _) => cow,
        };

        let lines = text.lines();

        if is_blank {
            let count = lines.count();
            stats.lines += count;
            stats.code += count;
            rs_ret_error!(rs_ret_error!(synced_tx.lock()).send(stats));
            return;
        }

        let should_handle_multi_line = has_multi_line && match &language.regex {
            &Some(ref regex) => regex.is_match(&text),
            &None => false,
        };

        'line: for line in lines {
            stats.lines += 1;
            let no_stack = stack.is_empty();
            if line.trim().is_empty() {
                stats.blanks += 1;
                continue;
            }

            // FORTRAN has a rule where it only counts as a comment if it's the
            // first character in the column, so removing starting whitespace
            // could cause a miscount.
            let line = if is_fortran { line } else { line.trim_left() };

            for single in &language.line_comment {
                if line.starts_with(single) {
                    stats.comments += 1;
                    continue 'line;
                }
            }

            if should_handle_multi_line {
                multi_line::handle_multi_line(
                    line,
                    &language,
                    &mut stack,
                    &mut quote
                );
            } else if language.line_comment.len() != 0 {
                let mut skip: u8 = 0;
                let window_size = language.line_comment.iter()
                    .map(|a| a.len())
                    .max()
                    .unwrap();

                'window: for window in line.as_bytes().windows(window_size) {
                    while skip != 0 {
                        skip -= 1;
                        continue 'window;
                    }

                    if quote.is_none() {
                        for single in &language.line_comment {
                            if window.starts_with(single.as_bytes()) {
                                break 'window;
                            }
                        }
                    }

                    if let &mut Some(quote_str) = &mut quote {
                        if window.starts_with(&*b"\\") {
                            skip = 1;
                        } else if window.starts_with(quote_str.as_bytes()) {
                            quote = None;
                            skip_by_str_length!(skip, quote_str);
                        }
                        continue 'window;
                    }

                    for &(start, end) in &language.quotes  {
                        if window.starts_with(start.as_bytes()) {
                            quote = Some(end);
                            skip_by_str_length!(skip, start);
                            continue 'window;
                        }
                    }
                }
            }

            if no_stack {
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
    pub fn from_cbor(cbor: &[u8]) -> serde_cbor::Result<Self>
    {
        let map = try!(serde_cbor::from_slice(cbor.into()));

        Ok(Self::from_previous(map))
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
    pub fn from_json(json: &[u8]) -> serde_json::Result<Self>
    {
        let map = try!(serde_json::from_slice(json.into()));

        Ok(Self::from_previous(map))
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
    pub fn from_yaml(yaml: &[u8]) -> serde_yaml::Result<Self>
    {
        let map = try!(serde_yaml::from_slice(yaml.into()));

        Ok(Self::from_previous(map))
    }

    #[cfg(not(feature = "yaml"))]
    pub fn from_yaml(_: &[u8]) -> Result<Self, ()> {
        Err(())
    }

    #[cfg(feature = "io")]
    fn from_previous(map: BTreeMap<LanguageType, Language>) -> Self {
        let mut _self = Self::new();

        for (name, input_language) in map {
            if let Some(language) = _self.get_mut(&name) {
                *language += input_language;
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
        let map = Self::generate_languages();
        Languages { inner: map }
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
    pub fn to_toml(self) -> String {
        toml::encode_str(&self.remove_empty())
    }

    #[cfg(not(feature = "toml-io"))]
    pub fn to_toml(&self) -> String {
        String::new()
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

impl<'a> AddAssign<&'a BTreeMap<LanguageType, Language>> for Languages {
    fn add_assign(&mut self, rhs: &'a BTreeMap<LanguageType, Language>) {

        for (name, language) in rhs {

            if let Some(result) = self.inner.get_mut(&name) {
                *result += language;
            }
        }
    }
}

impl<'a> AddAssign<&'a mut BTreeMap<LanguageType, Language>> for Languages {
    fn add_assign(&mut self, rhs: &'a mut BTreeMap<LanguageType, Language>) {

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


#[cfg(test)]
mod accuracy_tests {
    extern crate tempdir;
    use super::*;
    use std::io::Write;
    use std::fs::File;
    use language::LanguageType;
    use self::tempdir::TempDir;


    fn test_accuracy(file_name: &'static str,
                     expected: usize,
                     contents: &'static str)
    {
        let tmp_dir = TempDir::new("test").expect("Couldn't create temp dir");
        let file_name = tmp_dir.path().join(file_name);
        let mut file = File::create(&file_name).expect("Couldn't create file");
        file.write(contents.as_bytes()).expect("couldn't write to file");

        let mut l = Languages::new();
        let l_type = LanguageType::from_extension(&file_name)
            .expect("Can't find language type");
        l.get_statistics(vec![file_name.to_str().unwrap()], vec![]);
        let language = l.get_mut(&l_type).expect("Couldn't find language");

        assert_eq!(expected, language.code);
    }

    #[test]
    fn inside_quotes() {
        test_accuracy("inside_quotes.rs",
                      8,
                      r#"fn main() {
            let start = "/*";
            loop {
                if x.len() >= 2 && x[0] == '*' && x[1] == '/' { // found the */
                break;
                }
            }
        }"#)
    }

    #[test]
    fn shouldnt_panic() {
        test_accuracy("shouldnt_panic.rs",
                      9,
                      r#"fn foo() {
            let this_ends = "a \"test/*.";
            call1();
            call2();
            let this_does_not = /* a /* nested */ comment " */
                                "*/another /*test
            call3();
            */";
        }"#)
    }

    #[test]
    fn all_quotes_no_comment() {
        test_accuracy("all_quotes_no_comment.rs",
                      10,
                      r#"fn foobar() {
    let does_not_start = // "
        "until here,
        test/*
        test"; // a quote: "
    let also_doesnt_start = /* " */
        "until here,
        test,*/
        test"; // another quote: "
}"#)
    }

    #[test]
    fn commenting_on_comments() {
        test_accuracy("commenting_on_comments.rs",
                      5,
                      r#"fn foo() {
    let a = 4; // /*
    let b = 5;
    let c = 6; // */
}"#)
    }

    #[test]
    fn nesting_with_nesting_comments() {
        test_accuracy("nesting_with_nesting_comments.d",
                      5,
                      r#"void main() {
    auto x = 5; /+ a /+ nested +/ comment /* +/
    writefln("hello");
    auto y = 4; // */
}"#)
    }
}
