// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

use glob::glob;
use serde_cbor;
use serde_json;
use serde_yaml;
use walkdir::{WalkDir, WalkDirIterator};

use language::Language;
use language_name::LanguageName;
use language_name::LanguageName::*;

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
            "bash" | "sh" => Some(Bash),
            "bat" | "btm" | "cmd" => Some(Batch),
            "c" | "ec" | "pgc" => Some(C),
            "cc" | "cpp" | "cxx" | "c++" | "pcc" => Some(Cpp),
            "cfc" => Some(ColdFusionScript),
            "cfm" => Some(ColdFusion),
            "clj" => Some(Clojure),
            "coffee" => Some(CoffeeScript),
            "cs" => Some(CSharp),
            "csh" => Some(CShell),
            "css" => Some(Css),
            "d" => Some(D),
            "dart" => Some(Dart),
            "dts" | "dtsi" => Some(DeviceTree),
            "el" | "lisp" | "lsp" | "sc" => Some(Lisp),
            "erl" | "hrl" => Some(Erlang),
            "f" | "for" | "ftn" | "f77" | "pfo" => Some(FortranLegacy),
            "f03" | "f08" | "f90" | "f95" => Some(FortranModern),
            "go" => Some(Go),
            "h" => Some(CHeader),
            "hh" | "hpp" | "hxx" => Some(CppHeader),
            "hs" => Some(Haskell),
            "html" => Some(Html),
            "idr" | "lidr" => Some(Idris),
            "jai" => Some(Jai),
            "java" => Some(Java),
            "jl" => Some(Julia),
            "js" => Some(JavaScript),
            "json" => Some(Json),
            "jsx" => Some(Jsx),
            "kt" | "kts" => Some(Kotlin),
            "lds" => Some(LinkerScript),
            "less" => Some(Less),
            "lua" => Some(Lua),
            "m" => Some(ObjectiveC),
            "markdown" | "md" => Some(Markdown),
            "ml" | "mli" => Some(OCaml),
            "mm" => Some(ObjectiveCpp),
            "makefile" => Some(Makefile),
            "mustache" => Some(Mustache),
            "nim" => Some(Nim),
            "nb" | "wl" => Some(Wolfram),
            "oz" => Some(Oz),
            "p" | "pro" => Some(Prolog),
            "pas" => Some(Pascal),
            "php" => Some(Php),
            "pl" => Some(Perl),
            "qcl" => Some(Qcl),
            "text" | "txt" => Some(Text),
            "polly" => Some(Polly),
            "proto" => Some(Protobuf),
            "py" => Some(Python),
            "r" => Some(R),
            "rake" | "rb" => Some(Ruby),
            "rhtml" => Some(RubyHtml),
            "rs" => Some(Rust),
            "s" => Some(Assembly),
            "sass" | "scss" => Some(Sass),
            "scala" => Some(Scala),
            "sml" => Some(Sml),
            "sql" => Some(Sql),
            "swift" => Some(Swift),
            "tex" | "sty" => Some(Tex),
            "toml" => Some(Toml),
            "ts" => Some(TypeScript),
            "uc" | "uci" | "upkg" => Some(UnrealScript),
            "v" => Some(Coq),
            "vim" => Some(VimScript),
            "xml" => Some(Xml),
            "yaml" | "yml" => Some(Yaml),
            "zsh" => Some(Zsh),
            _ => None,
        }
    } else {
        None
    }
}

pub fn convert_input(contents: &[u8]) -> Option<BTreeMap<LanguageName, Language>> {
    if contents.is_empty() {
        None
    } else if let Ok(result) = serde_json::from_slice(contents) {
        Some(result)
    } else if let Ok(result) = serde_yaml::from_slice(contents) {
        Some(result)
    } else if let Ok(result) = serde_cbor::from_slice(contents) {
        Some(result)
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
