// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE, MIT} file.

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
pub mod stats;

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

use fsutil::*;
use language::{Language, LanguageName};
use language::LanguageName::*;

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

    let files_option = matches.is_present(FILES);
    let language_option = matches.is_present("languages");

    // Languages are placed inside a BTreeMap, in order to print alphabetically by default
    let mut languages = btreemap! {
        ActionScript => Language::new_c(),
        Assembly => Language::new_single(";"),
        Bash => Language::new_single("#"),
        Batch => Language::new_single("REM"),
        C => Language::new_c(),
        CHeader => Language::new_c(),
        CSharp => Language::new_c(),
        CShell => Language::new_single("#"),
        Clojure => Language::new_single(";,#,#_"),
        CoffeeScript => Language::new("#", "###", "###"),
        ColdFusion => Language::new_multi("<!---", "--->"),
        ColdFusionScript => Language::new_c(),
        Cpp => Language::new_c(),
        CppHeader => Language::new_c(),
        Css => Language::new_c(),
        D => Language::new_c(),
        Dart => Language::new_c(),
        DeviceTree => Language::new_c(),
        Lisp => Language::new(";", "#|", "|#"),
        FortranLegacy => Language::new_single("c,C,!,*"),
        FortranModern => Language::new_single("!"),
        Go => Language::new_c(),
        Haskell => Language::new_single("--"),
        Html => Language::new_html(),
        Jai => Language::new_c(),
        Java => Language::new_c(),
        JavaScript => Language::new_c(),
        Julia => Language::new("#", "#=", "=#"),
        Json => Language::new_blank(),
        Jsx => Language::new_c(),
        Less => Language::new_c(),
        LinkerScript => Language::new_c(),
        Lua => Language::new("--", "--[[", "]]"),
        Makefile => Language::new_single("#"),
        Markdown => Language::new_blank(),
        Mustache => Language::new_multi("{{!", "}}"),
        ObjectiveC => Language::new_c(),
        ObjectiveCpp => Language::new_c(),
        OCaml => Language::new_multi("(*", "*)"),
        Php => Language::new("#,//", "/*", "*/"),
        Pascal => Language::new("//,(*", "{", "}"),
        Polly => Language::new_html(),
        Perl => Language::new("#", "=", "=cut"),
        Protobuf => Language::new_single("//"),
        Python => Language::new("#", "'''", "'''"),
        R => Language::new_single("#"),
        Ruby => Language::new("#", "=begin", "=end"),
        RubyHtml => Language::new_html(),
        Rust => Language::new("//,///,//!", "/*", "*/"),
        Sass => Language::new_c(),
        Sml => Language::new_multi("(*", "*)"),
        Sql => Language::new("--", "/*", "*/"),
        Swift => Language::new_c(),
        Tex => Language::new_single("%"),
        Text => Language::new_blank(),
        Toml => Language::new_single("#"),
        TypeScript => Language::new_c(),
        VimScript => Language::new_single("\""),
        Xml => Language::new_html(),
        Yaml => Language::new_single("#"),
        Zsh => Language::new_single("#"),
    };

    // Print every supported language.
    if language_option {
        for key in languages.keys() {
            println!("{:<25}", key);
        }
        return;
    }

    let paths = matches.values_of("input").unwrap();

    let ignored_directories = {
        let mut ignored_directories = vec![".git"];
        if let Some(user_ignored) = matches.values_of("exclude") {
            for ignored in user_ignored {
                ignored_directories.push(ignored);
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

    get_all_files(paths, &mut languages, ignored_directories);

    let mut total = Language::new_blank();
    for (name, language) in &mut languages {

        if language.files.len() == 0 {
            continue;
        }

        let (tx, rx) = channel();
        let child = thread::spawn(move || {
            loop {
                if let Ok(_) = rx.try_recv() {
                    break;
                }
                // print!("\x1B[?25l;");
                print!(" Counting {} files.  \r", name);
                thread::sleep(Duration::from_millis(4));
                print!(" Counting {} files..\r", name);
                thread::sleep(Duration::from_millis(4));
                print!(" Counting {} files...\r", name);
                thread::sleep(Duration::from_millis(4));
            }
        });

        let files = language.files.clone();
        for file in files {
            let mut contents = String::new();
            let is_fortran = *name == FortranModern || *name == FortranLegacy;
            let mut stats = stats::Stats::new(unwrap_opt_cont!(file.to_str()));
            let _ = unwrap_rs_cont!(unwrap_rs_cont!(File::open(file))
                                        .read_to_string(&mut contents));

            let mut is_in_comments = false;
            let lines = contents.lines();


            if language.is_blank() {
                stats.code += lines.count();
                continue;
            }

            'line: for line in lines {
                let line = if is_fortran {
                    line
                } else {
                    line.trim()
                };
                stats.lines += 1;

                if line.trim().is_empty() {
                    stats.blanks += 1;
                    continue;
                }

                if !language.multi_line.is_empty() {
                    let multi_line = language.multi_line;
                    let multi_line_end = language.multi_line_end;
                    if line.starts_with(multi_line) {
                        is_in_comments = true;
                    } else if contains_comments(line, multi_line, multi_line_end) {
                        stats.code += 1;
                        is_in_comments = true;
                    }
                }


                if is_in_comments {
                    if line.contains(language.multi_line_end) {
                        is_in_comments = false;
                    }
                    stats.comments += 1;
                    continue;
                }

                let single_comments = language.line_comment.split(',');

                for single in single_comments {
                    if line.starts_with(single) {
                        stats.comments += 1;
                        continue 'line;
                    }
                }
                stats.code += 1;
            }

            if files_option {
                println!("{}", stats);
            }

            *language += stats;
        }

        let _ = tx.send(());
        let _ = child.join();
        print!("                                                       \r");
        if !language.is_empty() {
            if let None = sort {
                if files_option {
                    println!("{}", ROW);
                    println!("{}", language);
                    println!("{}", ROW);
                } else {
                    println!("{}", language);
                }
            }
        }


        total += language;
    }

    if let Some(sort_category) = sort {
        let mut sorted: Vec<&Language> = languages.values().collect();
        match &*sort_category {
            BLANKS => sorted.sort_by(|a, b| b.blanks.cmp(&a.blanks)),
            COMMENTS => sorted.sort_by(|a, b| b.comments.cmp(&a.comments)),
            CODE => sorted.sort_by(|a, b| b.code.cmp(&a.code)),
            FILES => sorted.sort_by(|a, b| b.files.len().cmp(&a.files.len())),
            TOTAL => sorted.sort_by(|a, b| b.lines.cmp(&a.lines)),
            _ => unreachable!(),
        }

        for language in sorted {
            if !language.is_empty() {
                println!("{}", *language);
            }
        }
    }

    if !files_option {
        println!("{}", ROW);
    }
    println!("{}", total);
    println!("{}", ROW);
    println!("\x1B[?25h");
}
