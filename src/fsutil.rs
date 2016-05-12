// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;
use clap::App;
use glob::glob;
use walkdir::{WalkDir, WalkDirIterator};

use language::{Language, LanguageName};
use language::LanguageName::*;

pub fn contains_comments(file: &str, comment: &str, comment_end: &str) -> bool {
    let mut in_comments: usize = 0;
    'window: for chars in file.chars().collect::<Vec<char>>().windows(comment.len()) {
        let section = {
            let mut section = String::new();
            for ch in chars {
                section.push(*ch);
            }
            section
        };

        if section == comment {
            in_comments += 1;
            continue 'window;
        } else if section == comment_end {
            if in_comments != 0 {
                in_comments -= 1;
            }
            continue 'window;
        }
    }
    in_comments != 0
}

pub fn get_all_files<'a, I: Iterator<Item = &'a str>>(paths: I,
                                                      languages: &mut BTreeMap<LanguageName,
                                                                               Language>,
                                                      ignored_directories: Vec<&str>) {
    for path in paths {
        if let Err(_) = Path::new(path).metadata() {
            if let Ok(paths) = glob(path) {
                for path in paths {
                    let path = unwrap_rs_cont!(path);
                    let mut language = if unwrap_opt_cont!(path.to_str()).contains("Makefile") {
                        languages.get_mut(&Makefile).unwrap()
                    } else {
                        unwrap_opt_cont!(languages.get_mut(&unwrap_opt_cont!(get_language(&path))))
                    };

                    language.files.push(path.to_owned());
                }
            } else {

            }
        } else {
            let walker = WalkDir::new(path).into_iter().filter_entry(|entry| {
                for ig in ignored_directories.to_owned() {
                    if entry.path().to_str().unwrap().contains(&*ig) {
                        return false;
                    }
                }
                true
            });

            for entry in walker {
                let entry = unwrap_rs_cont!(entry);

                let mut language = if unwrap_opt_cont!(entry.path().to_str())
                                          .contains("Makefile") {
                    languages.get_mut(&Makefile).unwrap()
                } else {
                    unwrap_opt_cont!(languages.get_mut(&unwrap_opt_cont!(get_language(entry.path()))))
                };

                language.files.push(entry.path().to_owned());
            }
        }
    }
}

pub fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
    let path = path.as_ref();
    let extension = match path.extension() {
        Some(extension_os) => {
            match extension_os.to_str() {
                Some(ext) => ext,
                None => return None,
            }
        }
        None => {
            match get_filetype_from_shebang(path) {
                Some(ext) => ext,
                None => return None,
            }
        }
    };
    Some(extension.to_lowercase())
}

pub fn get_filetype_from_shebang<P: AsRef<Path>>(file: P) -> Option<&'static str> {
    let file = match File::open(file) {
        Ok(file) => file,
        _ => return None,
    };
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let _ = buf.read_line(&mut line);

    let mut words = line.split_whitespace();
    match words.next() {
        Some("#!/bin/sh") => Some("sh"),
        Some("#!/bin/csh") => Some("csh"),
        Some("#!/usr/bin/perl") => Some("pl"),
        Some("#!/usr/bin/env") => {
            match words.next() {
                Some("python") | Some("python2") | Some("python3") => Some("py"),
                Some("sh") => Some("sh"),
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn get_language<P: AsRef<Path>>(entry: P) -> Option<LanguageName> {
    if let Some(extension) = get_extension(entry) {
        match &*extension {
            "as" => Some(ActionScript),
            "s" => Some(Assembly),
            "bat" => Some(Batch),
            "btm" => Some(Batch),
            "cmd" => Some(Batch),
            "bash" => Some(Bash),
            "sh" => Some(Bash),
            "c" => Some(C),
            "csh" => Some(CShell),
            "ec" => Some(C),
            "pgc" => Some(C),
            "cs" => Some(CSharp),
            "clj" => Some(Clojure),
            "coffee" => Some(CoffeeScript),
            "cfm" => Some(ColdFusion),
            "cfc" => Some(ColdFusionScript),
            "cc" => Some(Cpp),
            "cpp" => Some(Cpp),
            "cxx" => Some(Cpp),
            "pcc" => Some(Cpp),
            "c++" => Some(Cpp),
            "css" => Some(Css),
            "d" => Some(D),
            "dart" => Some(Dart),
            "dts" => Some(DeviceTree),
            "dtsi" => Some(DeviceTree),
            "el" => Some(Lisp),
            "lisp" => Some(Lisp),
            "lsp" => Some(Lisp),
            "lua" => Some(Lua),
            "sc" => Some(Lisp),
            "f" => Some(FortranLegacy),
            "f77" => Some(FortranLegacy),
            "for" => Some(FortranLegacy),
            "ftn" => Some(FortranLegacy),
            "pfo" => Some(FortranLegacy),
            "f90" => Some(FortranModern),
            "f95" => Some(FortranModern),
            "f03" => Some(FortranModern),
            "f08" => Some(FortranModern),
            "go" => Some(Go),
            "h" => Some(CHeader),
            "hs" => Some(Haskell),
            "hpp" => Some(CppHeader),
            "hh" => Some(CppHeader),
            "html" => Some(Html),
            "hxx" => Some(CppHeader),
            "jai" => Some(Jai),
            "java" => Some(Java),
            "js" => Some(JavaScript),
            "jl" => Some(Julia),
            "json" => Some(Json),
            "jsx" => Some(Jsx),
            "lds" => Some(LinkerScript),
            "less" => Some(Less),
            "m" => Some(ObjectiveC),
            "md" => Some(Markdown),
            "markdown" => Some(Markdown),
            "ml" => Some(OCaml),
            "mli" => Some(OCaml),
            "mm" => Some(ObjectiveCpp),
            "makefile" => Some(Makefile),
            "mustache" => Some(Mustache),
            "php" => Some(Php),
            "pas" => Some(Pascal),
            "pl" => Some(Perl),
            "text" => Some(Text),
            "txt" => Some(Text),
            "polly" => Some(Polly),
            "proto" => Some(Protobuf),
            "py" => Some(Python),
            "r" => Some(R),
            "rake" => Some(Ruby),
            "rb" => Some(Ruby),
            "rhtml" => Some(RubyHtml),
            "rs" => Some(Rust),
            "sass" => Some(Sass),
            "scss" => Some(Sass),
            "scala" => Some(Scala),
            "sml" => Some(Sml),
            "sql" => Some(Sql),
            "swift" => Some(Swift),
            "tex" => Some(Tex),
            "sty" => Some(Tex),
            "toml" => Some(Toml),
            "ts" => Some(TypeScript),
            "vim" => Some(VimScript),
            "xml" => Some(Xml),
            "yaml" => Some(Yaml),
            "yml" => Some(Yaml),
            "zsh" => Some(Zsh),
            _ => None,
        }
    } else {
        None
    }
}

#[allow(dead_code, unused_imports)]
mod tests {
    use super::*;
    #[test]
    fn comment_start_in_quotes() {
        assert!(contains_comments("Hello \"/*\" World", "/*", "*/"));
    }

    #[test]
    fn both_comments_in_quotes() {
        assert!(!contains_comments("Hello \"/**/\" World", "/*", "*/"));
    }

    #[test]
    fn both_comments_in_line() {
        assert!(!contains_comments("Hello /**/ World", "/*", "*/"));
    }

    #[test]
    fn comment_start_in_line() {
        assert!(contains_comments("Hello /* World", "/*", "*/"));
    }

    #[test]
    fn comment_start_in_quotes_ocaml() {
        assert!(contains_comments("Hello \"(*\" World", "(*", "*)"));
    }

    #[test]
    fn both_comments_in_quotes_ocaml() {
        assert!(!contains_comments("Hello \"(**)\" World", "(*", "*)"));
    }

    #[test]
    fn both_comments_in_line_ocaml() {
        assert!(!contains_comments("Hello (**) World", "(*", "*)"));
    }

    #[test]
    fn comment_start_in_line_ocaml() {
        assert!(contains_comments("Hello (* World", "(*", "*)"));
    }
}
