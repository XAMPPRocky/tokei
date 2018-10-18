// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE, MIT} file.

#[macro_use] extern crate clap;
extern crate env_logger;
extern crate log;
extern crate term_size;
extern crate tokei;

mod input;
mod cli_utils;
use input::*;
use cli_utils::*;

use std::{error::Error, process, io::{self, Write}};

use tokei::{Languages, Language, LanguageType};
use tokei::Sort;
use input::Format;

fn main() -> Result<(), Box<Error>> {
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
        (@arg types: -t --type
            +takes_value
            "Filters output by language type, seperated by a comma. i.e. -t=Rust,Markdown")
        (@arg languages: -l --languages
            conflicts_with[input]
            "Prints out supported languages and their extensions.")
        (@arg output: -o --output
            // `all` is used so to fail later with a better error
            possible_values(Format::all())
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
            case_insensitive(true)
            +takes_value
            "Sort languages based on column")
    ).get_matches();

    let files_option = matches.is_present("files");
    let input_option = matches.value_of("file_input");
    let output_option = matches.value_of("output");
    let print_languages_option = matches.is_present("languages");
    let sort_option = matches.value_of("sort");
    let verbose_option = matches.occurrences_of("verbose");
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

    let types: Option<Vec<_>> = matches.value_of("types").map(|e| {
        e.split(',')
         .map(|t| t.parse::<LanguageType>())
         .filter_map(Result::ok)
         .collect()
    });

    languages.get_statistics(&paths, ignored_directories, types);

    if let Some(format) = output_format {
        print!("{}", format.print(languages).unwrap());
        process::exit(0);
    }

    let columns = match term_size::dimensions() {
        Some((columns, _)) => columns.max(FALLBACK_ROW_LEN),
        None => FALLBACK_ROW_LEN,
    };

    let row = "-".repeat(columns);

    let mut stdout = io::BufWriter::new(io::stdout());

    if languages.iter().any(|(_, lang)| lang.inaccurate) {
        print_inaccuracy_warning(&mut stdout)?;
    }

    print_header(&mut stdout, &row, columns)?;

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

        print_results(&mut stdout, &row, languages.into_iter(), files_option)?
    } else  {
        print_results(&mut stdout, &row, languages.iter(), files_option)?
    }

    // If we're listing files there's already a trailing row so we don't want an extra one.
    if !files_option {
        writeln!(stdout, "{}", row)?;
    }

    let mut total = Language::new();

    for (_, language) in languages {
        total += language;
    }

    print_language(&mut stdout, columns, &total, "Total")?;
    writeln!(stdout, "{}", row)?;

    Ok(())
}
