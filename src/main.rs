#[macro_use]
extern crate log;

mod cli;
mod cli_utils;
mod consts;
mod input;

use std::{error::Error, io, process};

use tokei::{Column, Config, Languages, Sort};

use crate::{
    cli::Cli,
    cli_utils::Printer,
    consts::{
        BLANKS_COLUMN_WIDTH, CODE_COLUMN_WIDTH, COMMENTS_COLUMN_WIDTH, FALLBACK_ROW_LEN,
        LANGUAGE_COLUMN_WIDTH, LINES_COLUMN_WIDTH, PATH_COLUMN_WIDTH,
    },
    input::add_input,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = Cli::from_args();

    if cli.print_languages {
        Cli::print_supported_languages()?;
        process::exit(0);
    }
    let config = cli.override_config(Config::from_config_files());
    let mut languages = Languages::new();

    if let Some(input) = cli.file_input() {
        if !add_input(input, &mut languages) {
            Cli::print_input_parse_failure(input);
            process::exit(1);
        }
    }

    let input = cli.input();

    for path in &input {
        if ::std::fs::metadata(path).is_err() {
            eprintln!("Error: '{}' not found.", path);
            process::exit(1);
        }
    }

    let columns = cli
        .columns
        .or(config.columns)
        .or_else(|| {
            if cli.files {
                term_size::dimensions().map(|(w, _)| w)
            } else {
                None
            }
        })
        .unwrap_or(FALLBACK_ROW_LEN)
        .max(FALLBACK_ROW_LEN);

    if cli.streaming == Some(crate::cli::Streaming::Simple) {
        println!(
            "#{:^LANGUAGE_COLUMN_WIDTH$} {:^PATH_COLUMN_WIDTH$} {:^LINES_COLUMN_WIDTH$} {:^CODE_COLUMN_WIDTH$} {:^COMMENTS_COLUMN_WIDTH$} {:^BLANKS_COLUMN_WIDTH$}",
            "language", "path", "lines", "code", "comments", "blanks"
        );
        println!(
            "{:>LANGUAGE_COLUMN_WIDTH$} {:<PATH_COLUMN_WIDTH$} {:>LINES_COLUMN_WIDTH$} {:>CODE_COLUMN_WIDTH$} {:>COMMENTS_COLUMN_WIDTH$} {:>BLANKS_COLUMN_WIDTH$}",
            (0..10).map(|_| "#").collect::<String>(),
            (0..80).map(|_| "#").collect::<String>(),
            (0..12).map(|_| "#").collect::<String>(),
            (0..12).map(|_| "#").collect::<String>(),
            (0..12).map(|_| "#").collect::<String>(),
            (0..12).map(|_| "#").collect::<String>()
        );
    }

    languages.get_statistics(&input, &cli.ignored_directories(), &config);
    if config.for_each_fn.is_some() {
        process::exit(0);
    }

    if let Some(format) = cli.output {
        print!("{}", format.print(&languages).unwrap());
        process::exit(0);
    }

    let mut printer = Printer::new(
        columns,
        cli.files,
        io::BufWriter::new(io::stdout()),
        cli.number_format,
        config.output_columns.expect("`output_columns` must be set"),
    );

    if languages.iter().any(|(_, lang)| lang.inaccurate) {
        printer.print_inaccuracy_warning()?;
    }

    printer.print_header()?;

    let mut is_sorted = false;
    if let Some(sort_category) = cli.sort.or(config.sort) {
        for (_, ref mut language) in &mut languages {
            language.sort_by(sort_category);
        }

        let mut languages: Vec<_> = languages.iter().collect();
        match sort_category {
            Sort::Blanks => languages.sort_by(|a, b| b.1.blanks.cmp(&a.1.blanks)),
            Sort::Comments => languages.sort_by(|a, b| b.1.comments.cmp(&a.1.comments)),
            Sort::Code => languages.sort_by(|a, b| b.1.code.cmp(&a.1.code)),
            Sort::Files => languages.sort_by(|a, b| b.1.reports.len().cmp(&a.1.reports.len())),
            Sort::Lines => languages.sort_by(|a, b| b.1.lines().cmp(&a.1.lines())),
        }
        is_sorted = true;
        if cli.sort_reverse {
            printer.print_results(languages.into_iter().rev(), cli.compact, is_sorted)?;
        } else {
            printer.print_results(languages.into_iter(), cli.compact, is_sorted)?;
        }
    } else {
        printer.print_results(languages.iter(), cli.compact, is_sorted)?;
    }

    printer.print_total(&languages)?;

    Ok(())
}
