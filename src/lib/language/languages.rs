// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the APACHE2.0/MIT licence that can be
// found in the LICENCE-{APACHE/MIT} file.

use std::borrow::Cow;
use std::collections::{btree_map, BTreeMap};
use std::fs::File;
use std::io::Read;
use std::iter::IntoIterator;
use std::ops::{AddAssign, Deref, DerefMut};

#[cfg(feature = "cbor")]
use serde_cbor;
#[cfg(feature = "json")]
use serde_json;
#[cfg(feature = "yaml")]
use serde_yaml;
#[cfg(feature = "toml-io")]
use toml;
use rayon::prelude::*;

use utils::*;
use super::{Language, LanguageType};
use super::LanguageType::*;
use stats::Stats;

const CBOR_ERROR: &'static str = "Tokei was not compiled with the `cbor` flag.";
const JSON_ERROR: &'static str = "Tokei was not compiled with the `json` flag.";
const TOML_ERROR: &'static str = "Tokei was not compiled with the `toml-io` flag.";
const YAML_ERROR: &'static str = "Tokei was not compiled with the `yaml` flag.";

/// A collection of existing languages([_List of Languages_](https://github.com/Aaronepower/tokei#supported-languages))
#[derive(Debug, Clone)]
pub struct Languages {
    inner: BTreeMap<LanguageType, Language>,
}


impl Languages {
    /// Creates a `Languages` struct from cbor.
    ///
    /// ```
    /// # extern crate tokei;
    /// # use tokei::*;
    /// # extern crate rustc_serialize;
    /// # use rustc_serialize::hex::FromHex;
    /// # fn main () {
    /// let cbor = "a16452757374a666626c616e6b730564636f64650c68636f6d6d656e7473\
    ///     0065737461747381a566626c616e6b730564636f64650c68636f6d6d656e74730065\
    ///     6c696e657311646e616d65722e5c7372635c6c69625c6275696c642e7273656c696e\
    ///     6573116b746f74616c5f66696c657301";
    ///
    /// let mut languages = Languages::from_cbor(&*cbor.from_hex().unwrap()).unwrap();
    /// assert_eq!(12, languages.get_mut(&LanguageType::Rust).unwrap().code);
    /// # }
    /// ```
    #[cfg(feature = "cbor")]
    pub fn from_cbor<'a, I: Into<&'a [u8]>>(cbor: I) -> serde_cbor::Result<Self> {
        let map = try!(serde_cbor::from_slice(cbor.into()));

        Ok(Self::from_previous(map))
    }

    #[cfg(not(feature = "cbor"))]
    pub fn from_cbor<'a, I: Into<&'a [u8]>>(cbor: I) -> ! {
        panic!(CBOR_ERROR)
    }

    /// Creates a `Languages` struct from json.
    ///
    /// ```
    /// # use tokei::*;
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
    pub fn from_json<'a, I: Into<&'a [u8]>>(json: I) -> serde_json::Result<Self> {
        let map = try!(serde_json::from_slice(json.into()));

        Ok(Self::from_previous(map))
    }

    #[cfg(not(feature = "json"))]
    pub fn from_json<'a, I: Into<&'a [u8]>>(json: I) -> ! {
        panic!(JSON_ERROR)
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
    pub fn from_yaml<'a, I: Into<&'a [u8]>>(yaml: I) -> serde_yaml::Result<Self> {
        let map = try!(serde_yaml::from_slice(yaml.into()));

        Ok(Self::from_previous(map))
    }

    #[cfg(not(feature = "yaml"))]
    pub fn from_yaml<'a, I: Into<&'a [u8]>>(yaml: I) -> ! {
        panic!(YAML_ERROR)
    }

    fn from_previous(map: BTreeMap<LanguageType, Language>) -> Self {
        let mut _self = Self::new();

        for (name, input_language) in map {
            if let Some(language) = _self.get_mut(&name) {
                *language += input_language;
            }
        }
        _self
    }

    /// Get statistics from the list of paths provided, and a list ignored keywords
    /// to ignore paths containing them.
    ///
    /// ```no_run
    /// # use tokei::*;
    /// let mut languages = Languages::new();
    /// languages.get_statistics(&*vec!["."], &*vec![".git", "target"]);
    ///
    /// println!("{:?}", languages);
    /// ```
    pub fn get_statistics<'a, I>(&mut self, paths: I, ignored: I)
        where I: Into<Cow<'a, [&'a str]>>
    {

        get_all_files(paths.into(), ignored.into(), &mut self.inner);

        let mut language_iter: Vec<_> = self.inner.iter_mut().collect();

        language_iter.par_iter_mut().for_each(|&mut (name, ref mut language)| {
            if language.files.is_empty() {
                return;
            }

            let is_fortran = name == &FortranModern || name == &FortranLegacy;

            let files: Vec<_> = language.files.drain(..).collect();
            for file in files {
                let mut is_in_comments = false;
                let mut previous_comment_start = "";
                let mut comment_depth: usize = 0;
                let mut stats = Stats::new(opt_or_cont!(file.to_str()));

                let contents = {
                    let mut contents = String::new();
                    let _ = rs_or_cont!(rs_or_cont!(File::open(file))
                        .read_to_string(&mut contents));
                    contents
                };

                let lines = contents.lines();

                if language.is_blank() {
                    stats.code += lines.count();
                    continue;
                }

                'line: for line in lines {
                    stats.lines += 1;

                    // FORTRAN has a rule where it only counts as a comment if it's the first
                    // character in the column, so removing starting whitespace could cause a
                    // miscount.
                    let line = if is_fortran {
                        line
                    } else {
                        line.trim()
                    };

                    if line.trim().is_empty() {
                        stats.blanks += 1;
                        continue;
                    }

                    for &(multi_line, multi_line_end) in &language.multi_line {
                        if line.starts_with(multi_line) ||
                           has_trailing_comments(line,
                                                 multi_line,
                                                 multi_line_end,
                                                 language.nested) {
                            previous_comment_start = multi_line;
                            is_in_comments = true;
                            if language.nested {
                                comment_depth += 1;
                            }
                        }
                    }


                    if is_in_comments {
                        for &(multi_line, multi_line_end) in &language.multi_line {
                            if multi_line == previous_comment_start &&
                               line.contains(multi_line_end) {
                                if language.nested {
                                    comment_depth -= 1;
                                    if comment_depth == 0 {
                                        is_in_comments = false;
                                    }
                                } else {
                                    is_in_comments = false;
                                }
                            }
                        }
                        stats.comments += 1;
                        continue;
                    }

                    for single in &language.line_comment {
                        if line.starts_with(single) {
                            stats.comments += 1;
                            continue 'line;
                        }
                    }
                    stats.code += 1;
                }

                **language += stats;
            }
        });
    }

    /// Constructs a new, blank `Languages`.
    ///
    /// ```
    /// # use tokei::*;
    /// let languages = Languages::new();
    /// ```
    pub fn new() -> Self {
        use super::LanguageType::*;
        let map = btreemap! {
            ActionScript => Language::new_c(),
            Assembly => Language::new_single(vec![";"]),
            Autoconf => Language::new_single(vec!["#", "dnl"]),
            Bash => Language::new_hash(),
            Batch => Language::new_single(vec!["REM"]),
            C => Language::new_c(),
            CHeader => Language::new_c(),
            Clojure => Language::new_single(vec![";","#"]),
            CoffeeScript => Language::new(vec!["#"], vec![("###", "###")]),
            ColdFusion => Language::new_multi(vec![("<!---", "--->")]),
            ColdFusionScript => Language::new_c(),
            Coq => Language::new_func(),
            Cpp => Language::new_c(),
            CppHeader => Language::new_c(),
            CSharp => Language::new_c(),
            CShell => Language::new_hash(),
            Css => Language::new_c(),
            D => Language::new_c(),
            Dart => Language::new_c(),
            DeviceTree => Language::new_c(),
            Erlang => Language::new_single(vec!["%"]),
            FortranLegacy => Language::new_single(vec!["c","C","!","*"]),
            FortranModern => Language::new_single(vec!["!"]),
            Go => Language::new_c(),
            Haskell => Language::new_single(vec!["--"]),
            Html => Language::new_html(),
            Idris => Language::new(vec!["--"], vec![("{-", "-}")]),
            Isabelle => Language::new(
                vec!["--"],
                vec![   ("{*","*}"),
                        ("(*","*)"),
                        ("‹","›"),
                        ("\\<open>", "\\<close>"),
                    ]
            ),
            Jai => Language::new_c(),
            Java => Language::new_c(),
            JavaScript => Language::new_c(),
            Json => Language::new_blank(),
            Jsx => Language::new_c(),
            Julia => Language::new(vec!["#"], vec![("#=", "=#")]),
            Kotlin => Language::new_c(),
            Less => Language::new_c(),
            LinkerScript => Language::new_c(),
            Lisp => Language::new(vec![";"], vec![("#|", "|#")]),
            Lua => Language::new(vec!["--"], vec![("--[[", "]]")]),
            Makefile => Language::new_hash(),
            Markdown => Language::new_blank(),
            Mustache => Language::new_multi(vec![("{{!", "}}")]),
            Nim => Language::new_hash(),
            ObjectiveC => Language::new_c(),
            ObjectiveCpp => Language::new_c(),
            OCaml => Language::new_func(),
            Oz => Language::new_pro(),
            Pascal => Language::new(vec!["//","(*"], vec![("{", "}")]),
            Perl => Language::new(vec!["#"], vec![("=", "=cut")]),
            Php => Language::new(vec!["#","//"], vec![("/*", "*/")]),
            Polly => Language::new_html(),
            Prolog => Language::new_pro(),
            Protobuf => Language::new_single(vec!["//"]),
            Python => Language::new(vec!["#"], vec![("'''", "'''")]),
            Qcl => Language::new_c(),
            R => Language::new_hash(),
            Ruby => Language::new(vec!["#"], vec![("=begin", "=end")]),
            RubyHtml => Language::new_html(),
            Rust => Language::new_c().nested(),
            Sass => Language::new_c(),
            Scala => Language::new_c(),
            Sml => Language::new_func(),
            Sql => Language::new(vec!["--"], vec![("/*", "*/")]),
            Swift => Language::new_c(),
            Tex => Language::new_single(vec!["%"]),
            Text => Language::new_blank(),
            Toml => Language::new_hash(),
            TypeScript => Language::new_c(),
            UnrealScript => Language::new_c(),
            VimScript => Language::new_single(vec!["\""]),
            Wolfram => Language::new_func(),
            Xml => Language::new_html(),
            Yaml => Language::new_hash(),
            Zsh => Language::new_hash(),
        };

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
    /// let new_map: BTreeMap<LanguageType, Language> = BTreeMap::new();
    ///
    /// assert_eq!(empty_map, new_map);
    /// ```
    pub fn remove_empty(&self) -> BTreeMap<LanguageType, Language> {
        let mut map = BTreeMap::new();

        for (name, language) in &self.inner {
            if !language.is_empty() {
                map.insert(name.clone(), language.clone());
            }
        }
        map
    }

    /// Converts `Languages` to CBOR.
    ///
    /// ```no_run
    /// extern crate tokei;
    /// # use tokei::*;
    /// extern crate rustc_serialize;
    /// use rustc_serialize::hex::ToHex;
    ///
    /// # fn main () {
    /// let cbor = "a16452757374a666626c616e6b730564636f64650c68636f6d6d656e74730\
    ///     065737461747381a566626c616e6b730564636f64650c68636f6d6d656e747300656c\
    ///     696e657311646e616d65722e5c7372635c6c69625c6275696c642e7273656c696e657\
    ///     3116b746f74616c5f66696c657301";
    ///
    /// let mut languages = Languages::new();
    /// languages.get_statistics(&*vec!["src/lib/build.rs"], &*vec![".git"]);
    ///
    /// assert_eq!(cbor, languages.to_cbor().unwrap().to_hex());
    /// # }
    /// ```
    #[cfg(feature = "cbor")]
    pub fn to_cbor(&self) -> Result<Vec<u8>, serde_cbor::Error> {
        serde_cbor::to_vec(&self.remove_empty())
    }

    #[cfg(not(feature = "cbor"))]
    pub fn to_cbor(&self) -> ! {
        panic!(CBOR_ERROR)
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
    ///                 "name": ".\\src\\lib\\build.rs"
    ///             }
    ///         ],
    ///         "lines": 17
    ///     }
    /// }"#;
    /// let mut languages = Languages::new();
    /// languages.get_statistics(&*vec!["src/lib/build.rs"], &*vec![".git"]);
    ///
    /// assert_eq!(json, languages.to_json().unwrap());
    /// ```
    #[cfg(feature = "json")]
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.remove_empty())
    }

    #[cfg(not(feature = "json"))]
    pub fn to_json(&self) -> ! {
        panic!(JSON_ERROR)
    }

    #[cfg(feature = "toml-io")]
    pub fn to_toml(&self) -> String {
        toml::encode_str(&self.remove_empty())
    }

    #[cfg(not(feature = "toml-io"))]
    pub fn to_toml(&self) -> ! {
        panic!(TOML_ERROR)
    }

    /// Converts `Languages` to YAML.
    ///
    /// ```no_run
    /// # use tokei::*;
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
    ///         "name": ".\\src\\lib\\build.rs"#;
    /// let mut languages = Languages::new();
    /// languages.get_statistics(&*vec!["src/lib/build.rs"], &*vec![".git"]);
    ///
    /// assert_eq!(yaml, languages.to_yaml().unwrap());
    #[cfg(feature = "yaml")]
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(&self.remove_empty())
    }

    #[cfg(not(feature = "yaml"))]
    pub fn to_yaml(&self) -> ! {
        panic!(YAML_ERROR)
    }
}

impl IntoIterator for Languages {
    type Item = <BTreeMap<LanguageType, Language> as IntoIterator>::Item;
    type IntoIter = <BTreeMap<LanguageType, Language> as IntoIterator>::IntoIter;

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
