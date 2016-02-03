// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

#[macro_use]
extern crate clap;
#[macro_use]
extern crate maplit;
#[macro_use]
pub mod macros;
pub mod language;
pub mod fsutil;

use std::cell::RefCell;
use std::io::Read;
use std::path::Path;
use std::fs::File;

use clap::App;

use language::Language;

use fsutil::{get_all_files, contains_comments};
const ROW: &'static str = "-----------------------------------------------------------------------\
                           ---------";
const BLANKS: &'static str = "blanks";
const COMMENTS: &'static str = "comments";
const CODE: &'static str = "code";
const FILES: &'static str = "files";
const TOTAL: &'static str = "total";

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let action_script = Language::new_c("ActionScript");
    let bash = Language::new_single("BASH", "#");
    let batch = Language::new_single("Batch", "REM");
    let c = Language::new_c("C");
    let c_header = Language::new_c("C Header");
    let c_sharp = Language::new_c("C#");
    let clojure = Language::new_single("Clojure", ";,#,#_");
    let coffee_script = Language::new("CoffeeScript", "#", "###", "###");
    let cold_fusion = Language::new("ColdFusion", "<!---", "<!---", "--->");
    let cf_script = Language::new_c("ColdFusion CFScript");
    let cpp = Language::new_c("C++");
    let cpp_header = Language::new_c("C++ Header");
    let css = Language::new_c("CSS");
    let d = Language::new_c("D");
    let dart = Language::new_c("Dart");
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
    let lua = Language::new("Lua", "--", "--[[", "]]");
    let markdown = Language::new_blank("Markdown");
    let objective_c = Language::new_c("Objective-C");
    let objective_cpp = Language::new_c("Objective-C++");
    let ocaml = Language::new_multi("OCaml", "(*", "*)");
    let php = Language::new("PHP", "#,//", "/*", "*/");
    let pascal = Language::new("Pascal", "//,(*", "{", "}");
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
    let toml = Language::new_single("TOML", "#");
    let type_script = Language::new_c("TypeScript");
    let xml = Language::new_html("XML");
    let yaml = Language::new_single("YAML", "#");

    // Languages are placed inside a BTreeMap, in order to print alphabetically by default
    let mut languages = btreemap! {
        "as" => &action_script,
        "bat" => &batch,
        "btm" => &batch,
        "cmd" => &batch,
        "bash" => &bash,
        "sh" => &bash,
        "c" => &c,
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
        "less" => &less,
        "m" => &objective_c,
        "md" => &markdown,
        "ml" => &ocaml,
        "mli" => &ocaml,
        "mm" => &objective_cpp,
        "php" => &php,
        "pas" => &pascal,
        "pl" => &perl,
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
        for (_, language) in &languages {
            let mut language = language.borrow_mut();
            if !language.printed {
                println!("{:<25}", language.name);
                language.printed = true;
            }
        }
        return;
    }

    let paths = matches.values_of("input").unwrap();

    let mut ignored_directories: Vec<String> = Vec::new();
    if let Some(user_ignored) = matches.values_of("exclude") {
        for ignored in user_ignored {
            ignored_directories.push(ignored.to_owned());
        }
    }

    let mut sort = String::new();
    if let Some(sort_by) = matches.value_of("sort") {
        match &*sort_by.to_lowercase() {
            BLANKS | CODE | COMMENTS | FILES | TOTAL => sort.push_str(&*sort_by.to_lowercase()),
            _ => unreachable!(),
        }
    }
    let sort_empty = sort.is_empty();

    println!("{}", ROW);
    println!(" {:<10} {:>10} {:>10} {:>10} {:>10} {:>10}",
             "Language",
             "Files",
             "Total",
             "Blanks",
             "Comments",
             "Code");
    println!("{}", ROW);
    // Get every path from the paths provided.
    for path in paths {
        let files = get_all_files(path.to_owned(), &ignored_directories);
        for file in files {
            let extension = unwrap_opt_cont!(unwrap_opt_cont!(Path::new(&file).extension())
                                                 .to_str());
            let lowercase: &str = &extension.to_lowercase();
            let language = unwrap_opt_cont!(languages.get_mut(lowercase));
            language.borrow_mut().files.push(file.to_owned());
        }
    }

    let mut total = Language::new_raw("Total");
    for (_, language) in &mut languages {

        if language.borrow().printed {
            continue;
        }
        let files = language.borrow().files.clone();
        for file in files {
            let mut contents = String::new();
            let is_fortran = language.borrow().name.contains("FORTRAN");
            let _ = unwrap_rs_cont!(unwrap_rs_cont!(File::open(&file))
                                        .read_to_string(&mut contents));

            let mut is_in_comments = false;
            let lines = contents.lines();

            'line: for line in lines {
                let line = if is_fortran {
                    line
                } else {
                    line.trim()
                };
                language.borrow_mut().lines += 1;

                if line.trim().is_empty() {
                    language.borrow_mut().blanks += 1;
                    continue;
                }

                if !language.borrow().multi_line.is_empty() {
                    let multi_line = language.borrow().multi_line;
                    let multi_line_end = language.borrow().multi_line_end;
                    if line.starts_with(multi_line) {
                        is_in_comments = true;
                    } else if contains_comments(line, multi_line, multi_line_end) {
                        language.borrow_mut().code += 1;
                        is_in_comments = true;
                    }
                }


                if is_in_comments {
                    if line.contains(language.borrow().multi_line_end) {
                        is_in_comments = false;
                    }
                    language.borrow_mut().comments += 1;
                    continue;
                }

                let single_comments = language.borrow().line_comment.split(",");

                for single in single_comments {
                    if line.starts_with(single) {
                        language.borrow_mut().comments += 1;
                        continue 'line;
                    }
                }
                language.borrow_mut().code += 1;
            }
        }

        if !language.borrow().is_empty() {
            language.borrow_mut().printed = true;
            if sort_empty {
                println!("{}", *language.borrow());
                if matches.is_present(FILES) {
                    println!("{}", ROW);
                    for file in &language.borrow().files {
                        println!("{}", file);
                    }
                    println!("{}", ROW);
                }
            }
        }

        let language = language.borrow();

        total.total += language.files.len();
        total.lines += language.lines;
        total.comments += language.comments;
        total.blanks += language.blanks;
        total.code += language.code;
    }

    if !sort_empty {
        let mut unsorted_vec: Vec<(&&str, &&RefCell<Language>)> = languages.iter().collect();
        match &*sort {
            BLANKS => {
                unsorted_vec.sort_by(|a, b| {
                    let a = a.1.borrow();
                    let b = b.1.borrow();
                    b.blanks.cmp(&a.blanks)
                })
            }
            COMMENTS => {
                unsorted_vec.sort_by(|a, b| {
                    let a = a.1.borrow();
                    let b = b.1.borrow();
                    b.comments.cmp(&a.comments)
                })
            }
            CODE => {
                unsorted_vec.sort_by(|a, b| {
                    let a = a.1.borrow();
                    let b = b.1.borrow();
                    b.code.cmp(&a.code)
                })
            }
            FILES => {
                unsorted_vec.sort_by(|a, b| {
                    let a = a.1.borrow();
                    let b = b.1.borrow();
                    b.files.len().cmp(&a.files.len())
                })
            }
            TOTAL => {
                unsorted_vec.sort_by(|a, b| {
                    let a = a.1.borrow();
                    let b = b.1.borrow();
                    b.lines.cmp(&a.lines)
                })
            }
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
