// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE, MIT} file.

#[macro_use] extern crate clap;
#[macro_use] extern crate errln;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate tokei;

mod input;
use input::*;

use std::borrow::Cow;

use clap::App;
use env_logger::LogBuilder;
use log::LogLevelFilter;

use tokei::{Languages, Language, LanguageType};
use tokei::Sort::*;

const BLANKS: &'static str = "blanks";
const CODE: &'static str = "code";
const COMMENTS: &'static str = "comments";
const FILES: &'static str = "files";
const LINES: &'static str = "lines";
const ROW: &'static str = "----------------------------------------------------\
          ---------------------------";

fn main() {
    // Get options at the beginning, so the program doesn't have to make any
    // extra calls to get the information, and there isn't any magic strings.
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let files_option = matches.is_present(FILES);
    let input_option = matches.value_of("file_input");
    let output_option = matches.value_of("output");
    let language_option = matches.is_present("languages");
    let verbose_option = matches.occurrences_of("verbose");
    let sort_option = matches.value_of("sort");
    let ignored_directories = {
        let mut ignored_directories: Vec<&str> = Vec::new();
        if let Some(user_ignored) = matches.values_of("exclude") {
            ignored_directories.extend(user_ignored);
        }
        ignored_directories
    };

    let mut builder = LogBuilder::new();

    let filter_level = match verbose_option {
        1 => LogLevelFilter::Warn,
        _ => LogLevelFilter::Error,
    };

    builder.filter(None, filter_level);
    builder.init().expect("Couldn't create ENV Logger");

    let mut languages = Languages::new();

    if language_option {
        for key in LanguageType::list() {
            println!("{:<25}", key);
        }
        return;
    }

    let paths: Vec<&str> = match matches.values_of("input") {
        Some(vs) => vs.collect(),
        None => vec!["."],
    };

    if let Some(input) = input_option {
        add_input(input, &mut languages);
    }

    languages.get_statistics(paths, ignored_directories);

    if output_option.is_none() {
        println!("{}", ROW);
        println!(" {:<12} {:>12} {:>12} {:>12} {:>12} {:>12}",
                 "Language",
                 "Files",
                 "Lines",
                 "Code",
                 "Comments",
                 "Blanks");
        println!("{}", ROW);

        if sort_option.is_none() {
            for (name, language) in languages.iter().filter(isnt_empty) {
                if files_option {
                    print_language(language, name);
                    println!("{}", ROW);

                    for stat in &language.stats {
                        println!("{}", stat);
                    }
                    println!("{}", ROW);
                } else if output_option.is_none() {
                    print_language(language, name);
                }
            }
        }
    }


    if let Some(format) = output_option {
        match_output(format, languages);
        return
    } else if let Some(sort_category) = sort_option {

        for (_, ref mut language) in &mut languages {
            match &*sort_category {
                BLANKS => language.sort_by(Blanks),
                COMMENTS => language.sort_by(Comments),
                CODE => language.sort_by(Code),
                FILES => language.sort_by(Files),
                LINES => language.sort_by(Lines),
                _ => unreachable!(),
            }
        }

        let mut languages: Vec<_> = languages.iter().collect();

        match &*sort_category {
            BLANKS => languages.sort_by(|a, b| b.1.blanks.cmp(&a.1.blanks)),
            COMMENTS => languages.sort_by(|a, b| b.1.comments.cmp(&a.1.comments)),
            CODE => languages.sort_by(|a, b| b.1.code.cmp(&a.1.code)),
            FILES => languages.sort_by(|a, b| b.1.stats.len().cmp(&a.1.stats.len())),
            LINES => languages.sort_by(|a, b| b.1.lines.cmp(&a.1.lines)),
            _ => unreachable!(),
        }

        for (name, language) in languages {
            if !language.is_empty() {
                if !files_option {
                    print_language(&language, name);
                } else {
                    print_language(&language, name);
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
    let mut total = Language::new_blank();
    for (_, language) in languages {
        total += language;
    }
    println!(" {: <18} {: >6} {:>12} {:>12} {:>12} {:>12}",
             "Total",
             total.stats.len(),
             total.lines,
             total.code,
             total.comments,
             total.blanks);
    println!("{}", ROW);
}

fn isnt_empty(&(_, language): &(&LanguageType, &Language)) -> bool {
    !language.is_empty()
}

fn print_language<'a, C>(language: &'a Language, name: C)
    where C: Into<Cow<'a, LanguageType>>
{
    println!(" {: <18} {: >6} {:>12} {:>12} {:>12} {:>12}",
             name.into().name(),
             language.stats.len(),
             language.lines,
             language.code,
             language.comments,
             language.blanks)
}
