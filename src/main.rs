// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE, MIT} file.

#[macro_use] extern crate clap;
extern crate env_logger;
extern crate log;
extern crate term_size;
extern crate tokei;

mod input;
use input::*;

use std::str::FromStr;
use std::{error::Error, process, io::{self, Write}};

use tokei::{Languages, Language, LanguageType};
use tokei::Sort;
use input::Format;

const FILES: &str = "files";
const FALLBACK_ROW_LEN: usize = 79;
const NO_LANG_HEADER_ROW_LEN: usize = 67;
const NO_LANG_ROW_LEN: usize = 61;
const NO_LANG_ROW_LEN_NO_SPACES: usize = 54;

fn crate_version() -> String {
    if Format::supported().is_empty() {
        format!("{} compiled without serialization formats.", crate_version!())
    } else {
        format!(
            "{} compiled with serialization support: {}",
            crate_version!(),
            Format::supported().join(", ")
        )
    }
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

fn main() -> Result<(), Box<Error>> {
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
        (@arg percent: -p --percent
            "Will print out regular statistics with percentages related to the rest of the project.")
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
            +takes_value
            "Sort languages based on column")
    ).get_matches();

    let files_option = matches.is_present(FILES);
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
    let percent_option = matches.is_present("percent");

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
        e.split(",")
         .map(|t| t.parse::<LanguageType>())
         .filter_map(Result::ok)
         .collect()
    });

    languages.get_statistics(&paths, ignored_directories, types);
    let mut total = Language::new();

    for (_, language) in &languages {
        total += language;
    }

    if let Some(format) = output_format {
        print!("{}", format.print(languages).unwrap());
        process::exit(0);
    }

    let columns = match term_size::dimensions() {
        Some((columns, _)) => columns.max(FALLBACK_ROW_LEN),
        None => FALLBACK_ROW_LEN,
    };
    let row = "-".repeat(columns);

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    writeln!(stdout, "{}", row)?;
    writeln!(stdout, " {:<6$} {:>12} {:>12} {:>12} {:>12} {:>12}",
                "Language",
                "Files",
                "Lines",
                "Code",
                "Comments",
                "Blanks",
                columns - NO_LANG_HEADER_ROW_LEN)?;
    writeln!(stdout, "{}", row)?;

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

        print_results(&mut stdout, &row, languages.into_iter(), &total, files_option, percent_option)?
    } else  {
        print_results(&mut stdout, &row, languages.iter(), &total, files_option, percent_option)?
    }

    // If we're listing files there's already a trailing row so we don't want an extra one.
    if !files_option {
        writeln!(stdout, "{}", row)?;
    }

    print_language(&mut stdout, columns - NO_LANG_ROW_LEN, &total, &total, "Total", percent_option)?;
    writeln!(stdout, "{}", row)?;

    Ok(())
}

fn print_results<'a, I, W>(sink: &mut W, row: &str, languages: I, total: &Language, list_files: bool, show_percentage: bool)
    -> io::Result<()>
    where I: Iterator<Item = (&'a LanguageType, &'a Language)>,
          W: Write,
{
    let path_len = row.len() - NO_LANG_ROW_LEN_NO_SPACES;
    let lang_section_len = row.len() - NO_LANG_ROW_LEN;
    for (name, language) in languages.filter(isnt_empty) {
        print_language(sink, lang_section_len, language, total, name.name(), show_percentage)?;

        if list_files {
            writeln!(sink, "{}", row)?;
            for stat in &language.stats {
                writeln!(sink, "{:1$}", stat, path_len)?;
            }
            writeln!(sink, "{}", row)?;
        }
    }

    Ok(())
}

fn isnt_empty(&(_, language): &(&LanguageType, &Language)) -> bool {
    !language.is_empty()
}

fn print_language<W>(sink: &mut W,
                     lang_section_len: usize,
                     language: &Language,
                     total: &Language,
                     name: &str,
                     show_percentage: bool)
    -> io::Result<()>
    where W: Write,
{
    if !show_percentage {
       return writeln!(
                 sink,
                 " {:<len$} {:>6} {:>12} {:>12} {:>12} {:>12}",
                 name,
                 language.stats.len(),
                 language.lines,
                 language.code,
                 language.comments,
                 language.blanks,
                 len = lang_section_len)
    }

    let percent_char_count = 5;
    // don't show percentage for the total row because that would be pointless
    if language.lines == total.lines {
        // start the total number of fule just before the percentages, in line with the bar
        writeln!(sink,
                 " {:<len$} {:>7}{:<4} {:>12} {:>12} {:>12} {:>12}",
                 name,
                 language.stats.len(),
                 "",
                 language.lines,
                 language.code,
                 language.comments,
                 language.blanks,
                 len = lang_section_len - percent_char_count)
    } else {
        let percentage = get_percentage(language.lines, total.lines);
        writeln!(sink,
                 " {:<len$} {:>5}|{:<5} {:>12} {:>12} {:>12} {:>12}",
                 name,
                 language.stats.len(),
                 percentage,
                 language.lines,
                 language.code,
                 language.comments,
                 language.blanks,
                 len = lang_section_len - percent_char_count)
    }
}

fn get_percentage(lines: usize, total_lines: usize) -> String {
    let percentage = lines as f64 * 100_f64 / total_lines as f64;
    if percentage > 10_f64 {
        format!("{}%", percentage as usize)
    } else if percentage > 1_f64 {
        format!("{:.1}%", percentage)
    } else {
        format!("{:.2}%", percentage)
    }
}
