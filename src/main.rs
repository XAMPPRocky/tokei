// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

#[macro_use]
extern crate clap;
#[macro_use]
extern crate maplit;
extern crate glob;
extern crate walkdir;
#[macro_use]
pub mod macros;
pub mod language;
pub mod fsutil;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read};
use std::fs::File;
use std::path::Path;

use clap::App;
use glob::glob;
use walkdir::{WalkDir, WalkDirIterator};

use language::Language;

use fsutil::contains_comments;
const ROW: &'static str = "-----------------------------------------------------------------------\
                           --------";
const BLANKS: &'static str = "blanks";
const COMMENTS: &'static str = "comments";
const CODE: &'static str = "code";
const FILES: &'static str = "files";
const TOTAL: &'static str = "total";

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let action_script = Language::new_c("ActionScript");
    let asm = Language::new_single("Assembly", ";");
    let bash = Language::new_single("BASH", "#");
    let batch = Language::new_single("Batch", "REM");
    let c = Language::new_c("C");
    let c_header = Language::new_c("C Header");
    let c_sharp = Language::new_c("C#");
    let c_shell = Language::new_single("C Shell", "#");
    let clojure = Language::new_single("Clojure", ";,#,#_");
    let coffee_script = Language::new("CoffeeScript", "#", "###", "###");
    let cold_fusion = Language::new_multi("ColdFusion", "<!---", "--->");
    let cf_script = Language::new_c("ColdFusion CFScript");
    let cpp = Language::new_c("C++");
    let cpp_header = Language::new_c("C++ Header");
    let css = Language::new_c("CSS");
    let d = Language::new_c("D");
    let dart = Language::new_c("Dart");
    let device_tree = Language::new_c("Device Tree");
    let lisp = Language::new("LISP", ";", "#|", "|#");
    let fortran_legacy = Language::new_single("FORTRAN Legacy", "c,C,!,*");
    let fortran_modern = Language::new_single("FORTRAN Modern", "!");
    let go = Language::new_c("Go");
    let haskell = Language::new_single("Haskell", "--");
    let html = Language::new_html("HTML");
    let jai = Language::new_c("JAI");
    let java = Language::new_c("Java");
    let java_script = Language::new_c("JavaScript");
    let julia = Language::new("Julia", "#", "#=", "=#");
    let json = Language::new_blank("JSON");
    let jsx = Language::new_c("JSX");
    let less = Language::new_c("LESS");
    let linker_script = Language::new_c("LD Script");
    let lua = Language::new("Lua", "--", "--[[", "]]");
    let makefile = Language::new_single("Makefile", "#");
    let markdown = Language::new_blank("Markdown");
    let objective_c = Language::new_c("Objective C");
    let objective_cpp = Language::new_c("Objective C++");
    let ocaml = Language::new_multi("OCaml", "(*", "*)");
    let php = Language::new("PHP", "#,//", "/*", "*/");
    let pascal = Language::new("Pascal", "//,(*", "{", "}");
    let polly = Language::new_html("Polly");
    let perl = Language::new("Perl", "#", "=", "=cut");
    let python = Language::new("Python", "#", "'''", "'''");
    let r = Language::new_single("R", "#");
    let ruby = Language::new("Ruby", "#", "=begin", "=end");
    let ruby_html = Language::new_html("Ruby HTML");
    let rust = Language::new("Rust", "//,///,//!", "/*", "*/");
    let sass = Language::new_c("Sass");
    let sml = Language::new_multi("Standard ML", "(*", "*)");
    let sql = Language::new("SQL", "--", "/*", "*/");
    let swift = Language::new_c("Swift");
    let tex = Language::new_single("TeX", "%");
    let text = Language::new_blank("Plain Text");
    let toml = Language::new_single("TOML", "#");
    let type_script = Language::new_c("TypeScript");
    let xml = Language::new_html("XML");
    let yaml = Language::new_single("YAML", "#");

    // Languages are placed inside a BTreeMap, in order to print alphabetically by default
    let languages = btreemap! {
        "as" => &action_script,
        "s" => &asm,
        "bat" => &batch,
        "btm" => &batch,
        "cmd" => &batch,
        "bash" => &bash,
        "sh" => &bash,
        "c" => &c,
        "csh" => &c_shell,
        "ec" => &c,
        "pgc" => &c,
        "cs" => &c_sharp,
        "clj" => &clojure,
        "coffee" => &coffee_script,
        "cfm" => &cold_fusion,
        "cfc" => &cf_script,
        "cc" => &cpp,
        "cpp" => &cpp,
        "cxx" => &cpp,
        "pcc" => &cpp,
        "c++" => &cpp,
        "css" => &css,
        "d" => &d,
        "dart" => &dart,
        "dts" => &device_tree,
        "dtsi" => &device_tree,
        "el" => &lisp,
        "lisp" => &lisp,
        "lsp" => &lisp,
        "lua" => &lua,
        "sc" => &lisp,
        "f" => &fortran_legacy,
        "f77" => &fortran_legacy,
        "for" => &fortran_legacy,
        "ftn" => &fortran_legacy,
        "pfo" => &fortran_legacy,
        "f90" => &fortran_modern,
        "f95" => &fortran_modern,
        "f03" => &fortran_modern,
        "f08" => &fortran_modern,
        "go" => &go,
        "h" => &c_header,
        "hs" => &haskell,
        "hpp" => &cpp_header,
        "hh" => &cpp_header,
        "html" => &html,
        "hxx" => &cpp_header,
        "jai" => &jai,
        "java" => &java,
        "js" => &java_script,
        "jl" => &julia,
        "json" => &json,
        "jsx" => &jsx,
        "lds" => &linker_script,
        "less" => &less,
        "m" => &objective_c,
        "md" => &markdown,
        "markdown" => &markdown,
        "ml" => &ocaml,
        "mli" => &ocaml,
        "mm" => &objective_cpp,
        "makefile" => &makefile,
        "php" => &php,
        "pas" => &pascal,
        "pl" => &perl,
        "text" => &text,
        "txt" => &text,
        "polly" => &polly,
        "py" => &python,
        "r" => &r,
        "rake" => &ruby,
        "rb" => &ruby,
        "rhtml" => &ruby_html,
        "rs" => &rust,
        "sass" => &sass,
        "scss" => &sass,
        "sml" => &sml,
        "sql" => &sql,
        "swift" => &swift,
        "tex" => &tex,
        "sty" => &tex,
        "toml" => &toml,
        "ts" => &type_script,
        "xml" => &xml,
        "yaml" => &yaml,
        "yml" => &yaml,
    };

    // Print every supported language.
    if matches.is_present("languages") {
        for language in languages.values() {
            let mut language = language.borrow_mut();
            if !language.printed {
                println!("{:<25}", language.name);
                language.printed = true;
            }
        }
        return;
    }

    let paths = matches.values_of("input").unwrap();

    let ignored_directories = {
        let mut ignored_directories = vec![String::from(".git")];
        if let Some(user_ignored) = matches.values_of("exclude") {
            for ignored in user_ignored {
                ignored_directories.push(ignored.to_owned());
            }
        }
        ignored_directories
    };

    let sort = if let Some(sort_by) = matches.value_of("sort") {
        match &*sort_by.to_lowercase() {
            BLANKS | CODE | COMMENTS | FILES | TOTAL => Some(sort_by.to_lowercase()),
            _ => unreachable!(),
        }
    } else {
        None
    };

    println!("{}", ROW);
    println!(" {:<12} {:>12} {:>12} {:>12} {:>12} {:>12}",
             "Language",
             "Files",
             "Total",
             "Blanks",
             "Comments",
             "Code");
    println!("{}", ROW);

    get_all_files(paths, &languages, ignored_directories);

    let mut total = Language::new_raw("Total");
    for language in languages.values() {
        let mut language = language.borrow_mut();

        if language.printed {
            continue;
        }
        let is_blank_lang = if language.line_comment == "" && language.multi_line == "" {
            true
        } else {
            false
        };

        let files = language.files.clone();
        for file in files {
            let mut contents = String::new();
            let is_fortran = language.name.contains("FORTRAN");
            let _ = unwrap_rs_cont!(unwrap_rs_cont!(File::open(file))
                                        .read_to_string(&mut contents));

            let mut is_in_comments = false;
            let lines = contents.lines();

            if is_blank_lang {
                language.code += lines.count();
                continue;
            }

            'line: for line in lines {
                let line = if is_fortran {
                    line
                } else {
                    line.trim()
                };
                language.lines += 1;

                if line.trim().is_empty() {
                    language.blanks += 1;
                    continue;
                }

                if !language.multi_line.is_empty() {
                    let multi_line = language.multi_line;
                    let multi_line_end = language.multi_line_end;
                    if line.starts_with(multi_line) {
                        is_in_comments = true;
                    } else if contains_comments(line, multi_line, multi_line_end) {
                        language.code += 1;
                        is_in_comments = true;
                    }
                }


                if is_in_comments {
                    if line.contains(language.multi_line_end) {
                        is_in_comments = false;
                    }
                    language.comments += 1;
                    continue;
                }

                let single_comments = language.line_comment.split(',');

                for single in single_comments {
                    if line.starts_with(single) {
                        language.comments += 1;
                        continue 'line;
                    }
                }
                language.code += 1;
            }
        }

        if !language.is_empty() {
            language.printed = true;
            if let None = sort {
                println!("{}", *language);
                if matches.is_present(FILES) {
                    println!("{}", ROW);
                    for file in &language.files {
                        println!("{}", unwrap_opt_cont!(file.to_str()));
                    }
                    println!("{}", ROW);
                }
            }
        }

        total.total += language.files.len();
        total.lines += language.lines;
        total.comments += language.comments;
        total.blanks += language.blanks;
        total.code += language.code;
    }

    if let Some(sort_category) = sort {
        let mut unsorted_vec: Vec<(&&str, &&RefCell<Language>)> = languages.iter().collect();
        match &*sort_category {
            BLANKS => unsorted_vec.sort_by(|a, b| b.1.borrow().blanks.cmp(&a.1.borrow().blanks)),
            COMMENTS => {
                unsorted_vec.sort_by(|a, b| b.1.borrow().comments.cmp(&a.1.borrow().comments))
            }
            CODE => unsorted_vec.sort_by(|a, b| b.1.borrow().code.cmp(&a.1.borrow().code)),
            FILES => {
                unsorted_vec.sort_by(|a, b| b.1.borrow().files.len().cmp(&a.1.borrow().files.len()))
            }
            TOTAL => unsorted_vec.sort_by(|a, b| b.1.borrow().lines.cmp(&a.1.borrow().lines)),
            _ => unreachable!(),
        }

        for (_, language) in unsorted_vec {

            if !language.borrow().is_empty() && language.borrow().printed {
                language.borrow_mut().printed = false;
                println!("{}", *language.borrow());
            }
        }
    }

    println!("{}", ROW);
    println!("{}", total);
    println!("{}", ROW);
}


fn get_all_files<'a, I: Iterator<Item = &'a str>>(paths: I,
                                                  languages: &BTreeMap<&str, &RefCell<Language>>,
                                                  ignored_directories: Vec<String>) {
    for path in paths {
        if let Err(_) = Path::new(path).metadata() {
            if let Ok(paths) = glob(path) {
                for path in paths {
                    let path = unwrap_rs_cont!(path);
                    let extension = unwrap_opt_cont!(get_extension(&path));
                    let language = if unwrap_opt_cont!(path.to_str()).contains("Makefile") {
                        languages.get("makefile").unwrap()
                    } else {
                        unwrap_opt_cont!(languages.get(&*extension))
                    };

                    language.borrow_mut().files.push(path.to_owned());
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

                let extension = unwrap_opt_cont!(get_extension(entry.path()));
                let language = if unwrap_opt_cont!(entry.path().to_str()).contains("Makefile") {
                    languages.get("makefile").unwrap()
                } else {
                    unwrap_opt_cont!(languages.get(&*extension))
                };

                language.borrow_mut().files.push(entry.path().to_owned());
            }
        }
    }
}


fn get_filetype_from_shebang<P: AsRef<Path>>(file: P) -> Option<&'static str> {
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

fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
    let path = path.as_ref();
    let extension = match path.extension() {
        Some(extension_os) => {
            let extension = match extension_os.to_str() {
                Some(ext) => ext,
                None => return None,
            };
            extension.to_lowercase()
        }
        None => {
            match get_filetype_from_shebang(path) {
                Some(ext) => String::from(ext).to_lowercase(),
                None => return None,
            }
        }
    };
    Some(extension)
}
