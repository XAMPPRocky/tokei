// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use glob::glob;
use walkdir::{WalkDir, WalkDirIterator};

use language::{Language, LanguageName};
use language::LanguageName::*;

pub fn contains_comments(file: &str, comment: &str, comment_end: &str) -> bool {
    let mut in_comments: usize = 0;
    for chars in file.chars().collect::<Vec<char>>().windows(comment.len()) {
        let window = {
            let mut window = String::new();
            for ch in chars {
                window.push(*ch);
            }
            window
        };

        if window == comment {
            in_comments += 1;
            continue;
        } else if window == comment_end {
            if in_comments != 0 {
                in_comments -= 1;
            }
            continue;
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
                    let path = rs_or_cont!(path);
                    let mut language = if opt_or_cont!(path.to_str()).contains("Makefile") {
                        languages.get_mut(&Makefile).unwrap()
                    } else {
                        opt_or_cont!(languages.get_mut(&opt_or_cont!(get_language(&path))))
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
                let entry = rs_or_cont!(entry);

                let mut language = if opt_or_cont!(entry.path().to_str()).contains("Makefile") {
                    languages.get_mut(&Makefile).unwrap()
                } else {
                    opt_or_cont!(languages.get_mut(&opt_or_cont!(get_language(entry.path()))))
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
            "bash" => Some(Bash),
            "bat" => Some(Batch),
            "btm" => Some(Batch),
            "c" => Some(C),
            "cc" => Some(Cpp),
            "cfc" => Some(ColdFusionScript),
            "cfm" => Some(ColdFusion),
            "clj" => Some(Clojure),
            "cmd" => Some(Batch),
            "coffee" => Some(CoffeeScript),
            "cs" => Some(CSharp),
            "csh" => Some(CShell),
            "css" => Some(Css),
            "cpp" => Some(Cpp),
            "cxx" => Some(Cpp),
            "c++" => Some(Cpp),
            "d" => Some(D),
            "dart" => Some(Dart),
            "dts" => Some(DeviceTree),
            "dtsi" => Some(DeviceTree),
            "ec" => Some(C),
            "el" => Some(Lisp),
            "erl" => Some(Erlang),
            "f" => Some(FortranLegacy),
            "for" => Some(FortranLegacy),
            "ftn" => Some(FortranLegacy),
            "f03" => Some(FortranModern),
            "f08" => Some(FortranModern),
            "f77" => Some(FortranLegacy),
            "f90" => Some(FortranModern),
            "f95" => Some(FortranModern),
            "go" => Some(Go),
            "h" => Some(CHeader),
            "hh" => Some(CppHeader),
            "hpp" => Some(CppHeader),
            "hrl" => Some(Erlang),
            "hs" => Some(Haskell),
            "html" => Some(Html),
            "hxx" => Some(CppHeader),
            "idr" => Some(Idris),
            "jai" => Some(Jai),
            "java" => Some(Java),
            "jl" => Some(Julia),
            "js" => Some(JavaScript),
            "json" => Some(Json),
            "jsx" => Some(Jsx),
            "kt" => Some(Kotlin),
            "kts" => Some(Kotlin),
            "lds" => Some(LinkerScript),
            "less" => Some(Less),
            "lidr" => Some(Idris),
            "lisp" => Some(Lisp),
            "lsp" => Some(Lisp),
            "lua" => Some(Lua),
            "m" => Some(ObjectiveC),
            "markdown" => Some(Markdown),
            "md" => Some(Markdown),
            "ml" => Some(OCaml),
            "mli" => Some(OCaml),
            "mm" => Some(ObjectiveCpp),
            "makefile" => Some(Makefile),
            "mustache" => Some(Mustache),
            "nim" => Some(Nim),
            "nb" => Some(Wolfram),
            "oz" => Some(Oz),
            "p" => Some(Prolog),
            "pas" => Some(Pascal),
            "pfo" => Some(FortranLegacy),
            "pcc" => Some(Cpp),
            "php" => Some(Php),
            "pl" => Some(Perl),
            "pro" => Some(Prolog),
            "qcl" => Some(Qcl),
            "text" => Some(Text),
            "txt" => Some(Text),
            "pgc" => Some(C),
            "polly" => Some(Polly),
            "proto" => Some(Protobuf),
            "py" => Some(Python),
            "r" => Some(R),
            "rake" => Some(Ruby),
            "rb" => Some(Ruby),
            "rhtml" => Some(RubyHtml),
            "rs" => Some(Rust),
            "s" => Some(Assembly),
            "sass" => Some(Sass),
            "sc" => Some(Lisp),
            "scss" => Some(Sass),
            "scala" => Some(Scala),
            "sh" => Some(Bash),
            "sml" => Some(Sml),
            "sql" => Some(Sql),
            "swift" => Some(Swift),
            "tex" => Some(Tex),
            "sty" => Some(Tex),
            "toml" => Some(Toml),
            "ts" => Some(TypeScript),
            "uc" => Some(UnrealScript),
            "uci" => Some(UnrealScript),
            "upkg" => Some(UnrealScript),
            "v" => Some(Coq),
            "vim" => Some(VimScript),
            "wl" => Some(Wolfram),
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
