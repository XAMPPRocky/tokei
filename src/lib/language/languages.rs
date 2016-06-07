// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the APACHE2.0/MIT licence that can be
// found in the LICENCE-{APACHE/MIT} file.

use std::borrow::Cow;
use std::collections::{btree_map, BTreeMap};
use std::fs::File;
use std::io::Read;
use std::iter::IntoIterator;
use std::ops::{AddAssign, Deref, DerefMut};

use serde_cbor;
use serde_json;
use serde_yaml;
use rayon::prelude::*;

use utils::*;
use super::{Language, LanguageType};
use super::LanguageType::*;
use stats::Stats;

#[derive(Debug, Clone)]
pub struct Languages {
    inner: BTreeMap<LanguageType, Language>,
}


impl Languages {
    pub fn from_cbor<'a, I: Into<&'a [u8]>>(cbor: I) -> serde_cbor::Result<Self> {
        let map = try!(serde_cbor::from_slice(cbor.into()));

        Ok(Self::from_previous(map))
    }


    pub fn from_json<'a, I: Into<&'a [u8]>>(json: I) -> serde_json::Result<Self> {
        let map = try!(serde_json::from_slice(json.into()));

        Ok(Self::from_previous(map))
    }

    pub fn from_yaml<'a, I: Into<&'a [u8]>>(yaml: I) -> serde_yaml::Result<Self> {
        let map = try!(serde_yaml::from_slice(yaml.into()));

        Ok(Self::from_previous(map))
    }

    fn from_previous(map: BTreeMap<LanguageType, Language>) -> Self {
        let mut _self = Self::new();

        for (name, input_language) in map {
            if let Some(language) = _self.get_mut(&LanguageType::from(name)) {
                *language += input_language;
            }
        }
        _self
    }

    pub fn get_statistics<'a, I>(&mut self, paths: I, ignored: I)
        where I: Into<Cow<'a, [&'a str]>>
    {

        get_all_files(paths.into(), ignored.into(), &mut self.inner);

        let mut language_iter: Vec<_> = self.inner.iter_mut().collect();

        language_iter.par_iter_mut().for_each(|&mut (name, ref mut language)| {
            if language.files.is_empty() {
                return;
            }

            language.total_files = language.files.len();
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

    pub fn new() -> Self {
        use super::LanguageType::*;
        let map = btreemap! {
            ActionScript => Language::new_c(),
            Assembly => Language::new_single(vec![";"]),
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

    fn remove_empty(&self) -> BTreeMap<LanguageType, Language> {
        let mut map: BTreeMap<LanguageType, Language> = BTreeMap::new();

        for (name, language) in &self.inner {
            if !language.is_empty() {
                map.insert(name.clone(), language.clone());
            }
        }
        map
    }

    pub fn to_cbor(&self) -> Result<Vec<u8>, serde_cbor::Error> {
        serde_cbor::to_vec(&self.remove_empty())
    }
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.remove_empty())
    }
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(&self.remove_empty())
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
