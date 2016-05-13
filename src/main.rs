// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE, MIT} file.

#[macro_use]
extern crate clap;
#[macro_use]
extern crate maplit;
extern crate glob;
extern crate walkdir;
extern crate rayon;
#[macro_use]
mod macros;
mod consts;
mod fsutil;
mod language;
mod stats;

use std::io::{BufRead, Read};
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;

use clap::App;
use rayon::prelude::*;

use consts::*;
use fsutil::*;
use language::{Language, LanguageName};
use language::LanguageName::*;
use stats::Stats;

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
        Clojure => Language::new_single(";,#,#_"),
        CoffeeScript => Language::new("#", "###", "###"),
        ColdFusion => Language::new_multi("<!---", "--->"),
        ColdFusionScript => Language::new_c(),
        Coq => Language::new_func(),
        Cpp => Language::new_c(),
        CppHeader => Language::new_c(),
        CSharp => Language::new_c(),
        CShell => Language::new_single("#"),
        Css => Language::new_c(),
        D => Language::new_c(),
        Dart => Language::new_c(),
        DeviceTree => Language::new_c(),
        Erlang => Language::new_single("%"),
        FortranLegacy => Language::new_single("c,C,!,*"),
        FortranModern => Language::new_single("!"),
        Go => Language::new_c(),
        Haskell => Language::new_single("--"),
        Html => Language::new_html(),
        Idris => Language::new("--", "{-", "-}"),
        Jai => Language::new_c(),
        Java => Language::new_c(),
        JavaScript => Language::new_c(),
        Json => Language::new_blank(),
        Jsx => Language::new_c(),
        Julia => Language::new("#", "#=", "=#"),
        Kotlin => Language::new_c(),
        Less => Language::new_c(),
        LinkerScript => Language::new_c(),
        Lisp => Language::new(";", "#|", "|#"),
        Lua => Language::new("--", "--[[", "]]"),
        Makefile => Language::new_single("#"),
        Markdown => Language::new_blank(),
        Mustache => Language::new_multi("{{!", "}}"),
        Nim => Language::new_single("#"),
        ObjectiveC => Language::new_c(),
        ObjectiveCpp => Language::new_c(),
        OCaml => Language::new_func(),
        Oz => Language::new_pro(),
        Pascal => Language::new("//,(*", "{", "}"),
        Perl => Language::new("#", "=", "=cut"),
        Php => Language::new("#,//", "/*", "*/"),
        Polly => Language::new_html(),
        Prolog => Language::new_pro(),
        Protobuf => Language::new_single("//"),
        Python => Language::new("#", "'''", "'''"),
        Qcl => Language::new_c(),
        R => Language::new_single("#"),
        Ruby => Language::new("#", "=begin", "=end"),
        RubyHtml => Language::new_html(),
        Rust => Language::new("//,///,//!", "/*", "*/"),
        Sass => Language::new_c(),
        Sml => Language::new_func(),
        Sql => Language::new("--", "/*", "*/"),
        Swift => Language::new_c(),
        Tex => Language::new_single("%"),
        Text => Language::new_blank(),
        Toml => Language::new_single("#"),
        TypeScript => Language::new_c(),
        UnrealScript => Language::new_c(),
        VimScript => Language::new_single("\""),
        Wolfram => Language::new_func(),
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

    let mut languages: Vec<(LanguageName, Language)> = languages.into_iter().collect();

    let (tx, rx) = channel();
    let child = thread::spawn(move || {
        loop {
            if let Ok(_) = rx.try_recv() {
                break;
            }
            print!("\x1B[?25l");
            print!(" Counting files.  \r");
            thread::sleep(Duration::from_millis(4));
            print!(" Counting files..\r");
            thread::sleep(Duration::from_millis(4));
            print!(" Counting files...\r");
            thread::sleep(Duration::from_millis(4));
        }
    });

    languages.par_iter_mut()
             .for_each(|&mut (name, ref mut language)| {
                 if language.files.len() == 0 {
                     return;
                 }


                 language.total = language.files.len();
                 let files: Vec<_> = language.files.drain(..).collect();
                 for file in files {
                     let mut contents = String::new();
                     let is_fortran = name == FortranModern || name == FortranLegacy;
                     let mut stats = Stats::new(opt_or_cont!(file.to_str()));
                     let _ = rs_or_cont!(rs_or_cont!(File::open(file))
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
                             continue 'line;
                         }

                         if !language.multi_line.is_empty() {
                             let multi_line = language.multi_line;
                             let multi_line_end = language.multi_line_end;
                             if line.starts_with(multi_line) {
                                 is_in_comments = true;
                             } else if contains_comments(line, multi_line, multi_line_end) {
                                 is_in_comments = true;
                             }
                         }


                         if is_in_comments {
                             if line.contains(language.multi_line_end) {
                                 is_in_comments = false;
                             }
                             stats.comments += 1;
                             continue 'line;
                         }

                         if !language.line_comment.is_empty() {
                             for single in language.line_comment.split(',') {
                                 if line.starts_with(single) {
                                     stats.comments += 1;
                                     continue 'line;
                                 }
                             }
                         }
                         stats.code += 1;
                     }

                     *language += stats;
                 }

                 print!("                                                       \r");
                 if !language.is_empty() {
                     if let None = sort {
                         if files_option {
                             language.print(name);
                             println!("{}", ROW);

                             for stat in &language.stats {
                                 println!("{}", stat);
                             }
                             println!("{}", ROW);
                         } else {
                             language.print(name);
                         }
                     }
                 }
             });
    let _ = tx.send(());
    let _ = child.join();

    for &(_, ref language) in &languages {
        total += language;
    }

    if let Some(sort_category) = sort {

        for &mut (_, ref mut language) in &mut languages {
            match &*sort_category {
                BLANKS => language.sort_by(BLANKS),
                COMMENTS => language.sort_by(COMMENTS),
                CODE => language.sort_by(CODE),
                FILES => {}
                TOTAL => language.sort_by(TOTAL),
                _ => unreachable!(),
            }
        }

        match &*sort_category {
            BLANKS => languages.sort_by(|a, b| b.1.blanks.cmp(&a.1.blanks)),
            COMMENTS => languages.sort_by(|a, b| b.1.comments.cmp(&a.1.comments)),
            CODE => languages.sort_by(|a, b| b.1.code.cmp(&a.1.code)),
            FILES => languages.sort_by(|a, b| b.1.files.len().cmp(&a.1.files.len())),
            TOTAL => languages.sort_by(|a, b| b.1.lines.cmp(&a.1.lines)),
            _ => unreachable!(),
        }

        for (name, language) in languages {
            if !language.is_empty() {
                if !files_option {
                    language.print(name);
                } else {
                    language.print(name);
                    println!("{}", ROW);
                    for file in &language.stats {
                        println!("{}", file);
                    }
                    println!("{}", ROW);
                }
            }
        }
    }

    if !files_option {
        println!("{}", ROW);
    }
    total.print(__Total);
    println!("{}", ROW);
    print!("\x1B[?25h\r");
}
