// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

#[macro_use]
extern crate clap;

#[macro_use]
pub mod macros;
pub mod language;
pub mod fsutil;

use std::cell::RefCell;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::collections::BTreeMap;

use clap::App;

use language::Language;

use fsutil::{get_all_files, contains_comments};
const ROW: &'static str = "-----------------------------------------------------------------------\
                           ---------";
fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Big list of languages, this should changed if there is ever a map! macro.
    let action_script = RefCell::new(Language::new_c("ActionScript"));
    let bash = RefCell::new(Language::new_single("BASH", "#"));
    let batch = RefCell::new(Language::new_single("Batch", "REM"));
    let c = RefCell::new(Language::new_c("C"));
    let c_header = RefCell::new(Language::new_c("C Header"));
    let c_sharp = RefCell::new(Language::new_c("C#"));
    let clojure = RefCell::new(Language::new_single("Clojure", ";,#,#_"));
    let coffee_script = RefCell::new(Language::new("CoffeeScript", "#", "###", "###"));
    let cold_fusion = RefCell::new(Language::new("ColdFusion", "<!---", "<!---", "--->"));
    let cf_script = RefCell::new(Language::new_c("ColdFusion CFScript"));
    let cpp = RefCell::new(Language::new_c("C++"));
    let cpp_header = RefCell::new(Language::new_c("C++ Header"));
    let css = RefCell::new(Language::new_c("CSS"));
    let d = RefCell::new(Language::new_c("D"));
    let dart = RefCell::new(Language::new_c("Dart"));
    let lisp = RefCell::new(Language::new("LISP", ";", "#|", "|#"));
    let fortran_legacy = RefCell::new(Language::new_single("FORTRAN Legacy", "c,C,!,*"));
    let fortran_modern = RefCell::new(Language::new_single("FORTRAN Modern", "!"));
    let go = RefCell::new(Language::new_c("Go"));
    let haskell = RefCell::new(Language::new_single("Haskell", "--"));
    let html = RefCell::new(Language::new_html("HTML"));
    let jai = RefCell::new(Language::new_c("JAI"));
    let java = RefCell::new(Language::new_c("Java"));
    let java_script = RefCell::new(Language::new_c("JavaScript"));
    let julia = RefCell::new(Language::new("Julia", "#", "#=", "=#"));
    let json = RefCell::new(Language::new_blank("JSON"));
    let jsx = RefCell::new(Language::new_c("JSX"));
    let less = RefCell::new(Language::new_c("LESS"));
    let markdown = RefCell::new(Language::new_blank("Markdown"));
    let objective_c = RefCell::new(Language::new_c("Objective-C"));
    let objective_cpp = RefCell::new(Language::new_c("Objective-C++"));
    let php = RefCell::new(Language::new("PHP", "#,//", "/*", "*/"));
    let pascal = RefCell::new(Language::new("Pascal", "//,(*", "{", "}"));
    let perl = RefCell::new(Language::new("Perl", "#", "=", "=cut"));
    let python = RefCell::new(Language::new("Python", "#", "'''", "'''"));
    let r = RefCell::new(Language::new("R", "#", "", ""));
    let ruby = RefCell::new(Language::new("Ruby", "#", "=begin", "=end"));
    let ruby_html = RefCell::new(Language::new_html("Ruby HTML"));
    let rust = RefCell::new(Language::new("Rust", "//,///,//!", "/*", "*/"));
    let sass = RefCell::new(Language::new_c("Sass"));
    let sql = RefCell::new(Language::new("SQL", "--", "/*", "*/"));
    let swift = RefCell::new(Language::new_c("Swift"));
    let tex = RefCell::new(Language::new_single("TeX", "%"));
    let toml = RefCell::new(Language::new_single("TOML", "#"));
    let type_script = RefCell::new(Language::new_c("TypeScript"));
    let xml = RefCell::new(Language::new_html("XML"));
    let yaml = RefCell::new(Language::new_single("YAML", "#"));

    // Languages are placed inside a BTreeMap, in order to print alphabetically by default
    let mut languages: BTreeMap<&str, &RefCell<Language>> = BTreeMap::new();
    languages.insert("as", &action_script);
    languages.insert("bat", &batch);
    languages.insert("btm", &batch);
    languages.insert("cmd", &batch);
    languages.insert("bash", &bash);
    languages.insert("sh", &bash);
    languages.insert("c", &c);
    languages.insert("ec", &c);
    languages.insert("pgc", &c);
    languages.insert("cs", &c_sharp);
    languages.insert("clj", &clojure);
    languages.insert("coffee", &coffee_script);
    languages.insert("cfm", &cold_fusion);
    languages.insert("cfc", &cf_script);
    languages.insert("cc", &cpp);
    languages.insert("cpp", &cpp);
    languages.insert("cxx", &cpp);
    languages.insert("pcc", &cpp);
    languages.insert("c++", &cpp);
    languages.insert("css", &css);
    languages.insert("d", &d);
    languages.insert("dart", &dart);
    languages.insert("el", &lisp);
    languages.insert("lisp", &lisp);
    languages.insert("lsp", &lisp);
    languages.insert("sc", &lisp);
    languages.insert("f", &fortran_legacy);
    languages.insert("f77", &fortran_legacy);
    languages.insert("for", &fortran_legacy);
    languages.insert("ftn", &fortran_legacy);
    languages.insert("pfo", &fortran_legacy);
    languages.insert("f90", &fortran_modern);
    languages.insert("f95", &fortran_modern);
    languages.insert("f03", &fortran_modern);
    languages.insert("f08", &fortran_modern);
    languages.insert("go", &go);
    languages.insert("h", &c_header);
    languages.insert("hs", &haskell);
    languages.insert("hpp", &cpp_header);
    languages.insert("hh", &cpp_header);
    languages.insert("html", &html);
    languages.insert("jai", &jai);
    languages.insert("java", &java);
    languages.insert("js", &java_script);
    languages.insert("jl", &julia);
    languages.insert("json", &json);
    languages.insert("jsx", &jsx);
    languages.insert("less", &less);
    languages.insert("m", &objective_c);
    languages.insert("md", &markdown);
    languages.insert("mm", &objective_cpp);
    languages.insert("php", &php);
    languages.insert("pas", &pascal);
    languages.insert("pl", &perl);
    languages.insert("py", &python);
    languages.insert("r", &r);
    languages.insert("rake", &ruby);
    languages.insert("rb", &ruby);
    languages.insert("rhtml", &ruby_html);
    languages.insert("rs", &rust);
    languages.insert("sass", &sass);
    languages.insert("scss", &sass);
    languages.insert("sql", &sql);
    languages.insert("swift", &swift);
    languages.insert("tex", &tex);
    languages.insert("sty", &tex);
    languages.insert("toml", &toml);
    languages.insert("ts", &type_script);
    languages.insert("xml", &xml);
    languages.insert("yaml", &yaml);
    languages.insert("yml", &yaml);

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
    if let Some(user_ignored) = matches.value_of("exclude") {
        for ignored in user_ignored.split(",") {
            ignored_directories.push(ignored.to_owned());
        }
    }

    let mut sort = String::new();
    if let Some(sort_by) = matches.value_of("sort") {
        match &*sort_by.to_lowercase() {
            "files" | "total" | "blanks" | "comments" | "code" => {
                sort.push_str(&*sort_by.to_lowercase())
            }
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

    let mut total = Language::new_blank("Total");
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
                if matches.is_present("files") {
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
            "files" => {
                unsorted_vec.sort_by(|a, b| {
                    let a = a.1.borrow();
                    let b = b.1.borrow();
                    b.files.len().cmp(&a.files.len())
                })
            }
            "total" => {
                unsorted_vec.sort_by(|a, b| {
                    let a = a.1.borrow();
                    let b = b.1.borrow();
                    b.lines.cmp(&a.lines)
                })
            }
            "blanks" => {
                unsorted_vec.sort_by(|a, b| {
                    let a = a.1.borrow();
                    let b = b.1.borrow();
                    b.blanks.cmp(&a.blanks)
                })
            }
            "comments" => {
                unsorted_vec.sort_by(|a, b| {
                    let a = a.1.borrow();
                    let b = b.1.borrow();
                    b.comments.cmp(&a.comments)
                })
            }
            "code" => {
                unsorted_vec.sort_by(|a, b| {
                    let a = a.1.borrow();
                    let b = b.1.borrow();
                    b.code.cmp(&a.code)
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
