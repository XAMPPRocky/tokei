use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::btree_map;
use std::fs::File;
use std::io::Read;
use std::iter::IntoIterator;

use serde_cbor;
use serde_json;
use serde_yaml;
use rustc_serialize::hex::FromHex;
use rayon::prelude::*;

use utils::*;
use super::{Language, LanguageName};
use super::LanguageName::*;
use self::LanguageError::*;
use stats::Stats;

#[derive(Debug)]
pub struct Languages {
    inner: BTreeMap<LanguageName, Language>,
}


impl Languages {
    pub fn from_previous(previous: String) -> Result<Self, LanguageError> {
        let mut _self = Self::new();
        let map: Result<BTreeMap<LanguageName, Language>, LanguageError> = {
            if previous.is_empty() {
                Err(SerdeEmpty)
            } else if let Ok(result) = serde_json::from_str(&*previous) {
                Ok(result)
            } else if let Ok(result) = serde_yaml::from_str(&*previous) {
                Ok(result)
            } else if let Ok(result) = serde_cbor::from_slice(&*previous.from_hex().unwrap()) {
                Ok(result)
            } else {
                Err(InvalidFormat)
            }
        };

        match map {
            Ok(map) => {
                for (name, input_language) in map {
                    if let Some(language) = _self.get_mut(&LanguageName::from(name)) {
                        *language += input_language;
                    }
                }
                Ok(_self)
            }
            Err(error) => Err(error),
        }
    }

    pub fn add_previous(&mut self, previous: String) -> Result<(), LanguageError> {
        let previous_languages = match Self::from_previous(previous) {
            Ok(result) => result,
            Err(error) => return Err(error),
        };

        for (key, previous) in previous_languages {
            if let Some(language) = self.get_mut(&key) {
                *language += previous;
            }
        }
        Ok(())
    }

    pub fn get_statistics<'a, I, C>(&mut self, paths: C, ignored: C)
        where I: 'a + Iterator<Item = &'a str> + Clone,
              C: Into<Cow<'a, I>>
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

    pub fn get_mut(&mut self, key: &LanguageName) -> Option<&mut Language> {
        self.inner.get_mut(key)
    }

    pub fn new() -> Self {
        use super::LanguageName::*;
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
            Isabelle => Language::new(vec!["--"], vec![("{*","*}"), ("(*","*)"), ("‹","›"), ("\\<open>", "\\<close>")]),
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

    pub fn to_cbor(&self) -> Result<Vec<u8>, serde_cbor::Error> {
        serde_cbor::to_vec(&self.inner)
    }
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.inner)
    }
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(&self.inner)
    }
}


impl IntoIterator for Languages {
    type Item = <BTreeMap<LanguageName, Language> as IntoIterator>::Item;
    type IntoIter = <BTreeMap<LanguageName, Language> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Languages {
    type Item = (&'a LanguageName, &'a Language);
    type IntoIter = btree_map::Iter<'a, LanguageName, Language>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut Languages {
    type Item = (&'a LanguageName, &'a mut Language);
    type IntoIter = btree_map::IterMut<'a, LanguageName, Language>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

#[derive(Debug)]
pub enum LanguageError {
    SerdeEmpty,
    InvalidFormat,
}
