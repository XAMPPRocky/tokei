// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE, MIT} file.

#[macro_use] extern crate clap;
extern crate env_logger;
extern crate log;
extern crate tokei;

mod input;
use input::*;

use std::str::FromStr;
use std::process;

use tokei::{Languages, Language, LanguageType};
use tokei::Sort;
use input::Format;

const FILES: &str = "files";
const ROW: &str = "------------------------------------------------------------\
                   -------------------";

fn crate_version() -> String {
    if Format::supported().is_empty() {
        return crate_version!().into()
    }

    format!("{} compiled with serialization support: {}",
        crate_version!(), Format::supported().join(", "))
}

fn setup_logger(verbose_option: u64) {
    use log::LevelFilter;

    let mut builder = env_logger::Builder::new();

    let filter_level = match verbose_option {
        1 => LevelFilter::Warn,
        2 => LevelFilter::Debug,
        3 => LevelFilter::Trace,
        _ => LevelFilter::Error,
    };

    builder.filter(None, filter_level);
    builder.init();
}

fn print_input_parse_failure(input_filename: &str) {
    eprintln!("Error:\n Failed to parse input file: {}", input_filename);

    let not_supported = input::Format::not_supported();
    if !not_supported.is_empty() {
        eprintln!("
This version of tokei was compiled without serialization support for the following formats:

    {not_supported}

You may want to install any comma separated combination of {all:?}:

    cargo install tokei --features {all:?}

Or use the 'all' feature:

    cargo install tokei --features all
    \n",
            not_supported = not_supported.join(", "),
            // no space after comma to ease copypaste
            all = self::Format::all_feature_names().join(",")
        );
    }
}

fn print_supported_languages() {
    for key in LanguageType::list() {
        println!("{:<25}", key);
    }
}

fn parse_or_exit<T>(s: &str) -> T
where T: FromStr,
      T::Err: std::fmt::Display {
    T::from_str(s).unwrap_or_else(|e| {
        eprintln!("Error:\n{}", e);
        std::process::exit(1);
    })
}

fn main() {
    // Get options at the beginning, so the program doesn't have to make any
    // extra calls to get the information, and there isn't any magic strings.

    let matches = clap_app!(tokei =>
        (version: &*crate_version())
        (author: "Aaron P. <theaaronepower@gmail.com> + Contributors")
        (about: crate_description!())
        (@arg exclude: -e --exclude
            +takes_value
            +multiple number_of_values(1)
            "Ignore all files & directories containing the word.")
        (@arg file_input: -i --input
            +takes_value
            "Gives statistics from a previous tokei run. Can be given a file path, \
            or \"stdin\" to read from stdin.")
        (@arg files: -f --files
            "Will print out statistics on individual files.")
        (@arg input:
            conflicts_with[languages] ...
            "The input file(s)/directory(ies) to be counted.")
        (@arg languages: -l --languages
            conflicts_with[input]
            "Prints out supported languages and their extensions.")
        (@arg output: -o --output
            possible_values(/* `all` is used so to fail later with a better error */Format::all())
            +takes_value
            "Outputs Tokei in a specific format. Compile with additional features for more \
            format support.")
        (@arg verbose: -v --verbose ...
        "Set log output level:
         1: to show unknown file extensions,
         2: reserved for future debugging,
         3: enable file level trace. Not recommended on multiple files")
        (@arg sort: -s --sort
            possible_values(&["files", "lines", "blanks", "code", "comments"])
            +takes_value
            "Sort languages based on column")
    ).get_matches();
    let files_option = matches.is_present(FILES);
    let input_option = matches.value_of("file_input");
    let output_option = matches.value_of("output");
    let print_languages_option = matches.is_present("languages");
    let verbose_option = matches.occurrences_of("verbose");
    let sort_option = matches.value_of("sort");
    let ignored_directories = {
        let mut ignored_directories: Vec<&str> = Vec::new();
        if let Some(user_ignored) = matches.values_of("exclude") {
            ignored_directories.extend(user_ignored);
        }
        ignored_directories
    };

    // Sorting category should be restricted by clap but parse before we do work just in case.
    let sort_category = sort_option.map(parse_or_exit::<Sort>);
    // Format category is overly accepting by clap (so the user knows what is supported)
    // but this will fail if support is not compiled in and give a useful error to the user.
    let output_format = output_option.map(parse_or_exit::<Format>);

    setup_logger(verbose_option);

    let mut languages = Languages::new();

    if print_languages_option {
        print_supported_languages();
        process::exit(0);
    }

    let paths: Vec<&str> = match matches.values_of("input") {
        Some(vs) => vs.collect(),
        None => vec!["."],
    };

    if let Some(input) = input_option {
        if !add_input(input, &mut languages) {
            print_input_parse_failure(input);
            process::exit(1);
        }
    }

    languages.get_statistics(&paths, ignored_directories);

    if let Some(format) = output_format {
        print!("{}", format.print(languages).unwrap());
        process::exit(0);
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

    if let Some(sort_category) = sort_category {
        for (_, ref mut language) in &mut languages {
            language.sort_by(sort_category)
        }

        let mut languages: Vec<_> = languages.iter().collect();

        match sort_category {
            Sort::Blanks => languages.sort_by(|a, b| b.1.blanks.cmp(&a.1.blanks)),
            Sort::Comments => languages.sort_by(|a, b| b.1.comments.cmp(&a.1.comments)),
            Sort::Code => languages.sort_by(|a, b| b.1.code.cmp(&a.1.code)),
            Sort::Files => languages.sort_by(|a, b| b.1.stats.len().cmp(&a.1.stats.len())),
            Sort::Lines => languages.sort_by(|a, b| b.1.lines.cmp(&a.1.lines)),
        }

        print_results(languages.into_iter(), files_option)
    } else  {
        print_results(languages.iter(), files_option)
    }

    // If we're listing files there's already a trailing row so we don't want an extra one.
    if !files_option {
        println!("{}", ROW);
    }
    let mut total = Language::new_blank();
    for (_, language) in languages {
        total += language;
    }

    print_language(&total, "Total");
    println!("{}", ROW);
}

fn print_results<'a, I>(languages: I, list_files: bool)
where I: std::iter::Iterator<Item = (&'a LanguageType, &'a Language)> {
    for (name, language) in languages.filter(isnt_empty) {
        print_language(language, name.name());

        if list_files {
            println!("{}", ROW);
            for stat in &language.stats {
                println!("{}", stat);
            }
            println!("{}", ROW);
        }
    }
}

fn isnt_empty(&(_, language): &(&LanguageType, &Language)) -> bool {
    !language.is_empty()
}

fn print_language(language: &Language, name: &str) {
    println!(" {: <18} {: >6} {:>12} {:>12} {:>12} {:>12}",
             name,
             language.stats.len(),
             language.lines,
             language.code,
             language.comments,
             language.blanks)
}
