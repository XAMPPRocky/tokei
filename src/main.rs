// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE, MIT} file.

#[macro_use] extern crate clap;
extern crate env_logger;
extern crate log;
extern crate tokei;

mod input;
use input::*;

use std::borrow::Cow;

use env_logger::Builder;
use log::LevelFilter;

use tokei::{Languages, Language, LanguageType};
use tokei::Sort::*;

const BLANKS: &str = "blanks";
const CODE: &str = "code";
const COMMENTS: &str = "comments";
const FILES: &str = "files";
const LINES: &str = "lines";
const ROW: &str = "------------------------------------------------------------\
                   -------------------";

fn main() {
    // Get options at the beginning, so the program doesn't have to make any
    // extra calls to get the information, and there isn't any magic strings.
    let matches = clap_app!(tokei =>
        (version: crate_version!())
        (author: "Aaron P. <theaaronepower@gmail.com> + Contributors")
        (about: crate_description!())
        (@arg exclude: -e --exclude +takes_value +multiple number_of_values(1) "Ignore all files & directories containing the word.")
        (@arg file_input: -i --input +takes_value "Gives statistics from a previous tokei run. Can be given a file path, or \"stdin\" to read from stdin.")
        (@arg files: -f --files "Will print out statistics on individual files.")
        (@arg input: conflicts_with[languages] ... "The input file(s)/directory(ies) to be counted.")
        (@arg languages: -l --languages conflicts_with[input] "Prints out supported languages and their extensions.")
        (@arg output: -o --output possible_values(&["cbor", "json", "toml", "yaml"]) +takes_value "Outputs Tokei in a specific format.")
        (@arg verbose: -v --verbose ... "Set log output level:
         1: to show unknown file extensions,
         2: reserved for future debugging,
         3: enable file level trace. Not recommended on multiple files")
        (@arg sort: -s --sort possible_values(&["files", "lines", "blanks", "code", "comments"]) +takes_value "Sort languages based on column")
    ).get_matches();
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

    let mut builder = Builder::new();

    let filter_level = match verbose_option {
        1 => LevelFilter::Warn,
        2 => LevelFilter::Debug,
        3 => LevelFilter::Trace,
        _ => LevelFilter::Error,
    };

    builder.filter(None, filter_level);
    builder.init();

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

    languages.get_statistics(&paths, ignored_directories);

    if let Some(format) = output_option {
        match_output(format, languages);
    }

    println!("{}", ROW);
    println!(" {:<12} {:>12} {:>12} {:>12} {:>12} {:>12}",
                "Language",
                "Files",
                "Lines",
                "Code",
                "Comments",
                "Blanks");
    println!("{}", ROW);

    if let Some(sort_category) = sort_option {

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
                    print_language(language, name);
                } else {
                    print_language(language, name);
                    println!("{}", ROW);
                    for file in &language.stats {
                        println!("{}", file);
                    }
                    println!("{}", ROW);
                }
            }
        }
    } else  {
        for (name, language) in languages.iter().filter(isnt_empty) {
            if files_option {
                print_language(language, name);
                println!("{}", ROW);

                for stat in &language.stats {
                    println!("{}", stat);
                }
                println!("{}", ROW);
            } else  {
                print_language(language, name);
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
