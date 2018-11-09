// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE, MIT} file.

#[macro_use] extern crate clap;
extern crate env_logger;
extern crate log;
extern crate term_size;
extern crate tokei;

mod cli;
mod input;
mod cli_utils;

use std::{error::Error, process, io::{self, Write}};

use tokei::{Language, Languages, Sort};

use cli::Cli;
use cli_utils::*;
use input::*;

fn main() -> Result<(), Box<Error>> {
    let mut cli = Cli::from_args();
    if cli.print_languages {
        Cli::print_supported_languages();
        process::exit(0);
    }

    setup_logger(cli.verbose);
    let mut languages = Languages::new();

    if let Some(input) = cli.file_input() {
        if !add_input(input, &mut languages) {
            Cli::print_input_parse_failure(input);
            process::exit(1);
        }
    }

    {
        let types = ::std::mem::replace(&mut cli.types, None);
        let input = cli.input();

        for path in &input {
            if ::std::fs::metadata(path).is_err() {
                eprintln!("Error: '{}' not found.", path);
                process::exit(1);
            }
        }

        languages.get_statistics(&input, cli.ignored_directories(), types);
    }

    if let Some(format) = cli.output {
        print!("{}", format.print(languages).unwrap());
        process::exit(0);
    }

    let row = "-".repeat(cli.columns);

    let mut stdout = io::BufWriter::new(io::stdout());

    if languages.iter().any(|(_, lang)| lang.inaccurate) {
        print_inaccuracy_warning(&mut stdout)?;
    }

    print_header(&mut stdout, &row, cli.columns)?;

    if let Some(sort_category) = cli.sort {
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

        print_results(&mut stdout, &row, languages.into_iter(), cli.files)?
    } else  {
        print_results(&mut stdout, &row, languages.iter(), cli.files)?
    }

    // If we're listing files there's already a trailing row so we don't want an
    // extra one.
    if !cli.files {
        writeln!(stdout, "{}", row)?;
    }

    let mut total = Language::new();

    for (_, language) in languages {
        total += language;
    }

    print_language(&mut stdout, cli.columns, &total, "Total")?;
    writeln!(stdout, "{}", row)?;

    Ok(())
}
