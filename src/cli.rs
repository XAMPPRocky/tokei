use clap::ArgMatches;
use tokei::{LanguageType, Sort};

use input::Format;
use cli_utils::*;

pub struct Cli<'a> {
    matches: ArgMatches<'a>,
    pub output: Option<Format>,
    pub files: bool,
    pub print_languages: bool,
    pub sort: Option<Sort>,
    pub types: Option<Vec<LanguageType>>,
    pub columns: usize,
    pub verbose: u64,
}

impl<'a> Cli<'a> {
    pub fn from_args() -> Self {
        let matches = clap_app!(tokei =>
            (version: &*crate_version())
            (author: "Aaron P. <theaaronepower@gmail.com> + Contributors")
            (about: crate_description!())
            (@arg columns: -c --columns
                +takes_value
                conflicts_with[output]
                "Sets a strict column width of the output, only available for \
                terminal output.")
            (@arg exclude: -e --exclude
                +takes_value
                +multiple number_of_values(1)
                "Ignore all files & directories containing the word.")
            (@arg files: -f --files
                "Will print out statistics on individual files.")
            (@arg file_input: -i --input
                +takes_value
                "Gives statistics from a previous tokei run. Can be given a file path, \
                or \"stdin\" to read from stdin.")
            (@arg input:
                conflicts_with[languages] ...
                "The input file(s)/directory(ies) to be counted.")
            (@arg languages: -l --languages
                conflicts_with[input]
                "Prints out supported languages and their extensions.")
            (@arg output: -o --output
                // `all` is used so to fail later with a better error
                possible_values(Format::all())
                +takes_value
                "Outputs Tokei in a specific format. Compile with additional features for more \
                format support.")
            (@arg sort: -s --sort
                possible_values(&["files", "lines", "blanks", "code", "comments"])
                case_insensitive(true)
                +takes_value
                "Sort languages based on column")
            (@arg types: -t --type
                +takes_value
                "Filters output by language type, seperated by a comma. i.e. -t=Rust,Markdown")
            (@arg verbose: -v --verbose ...
            "Set log output level:
            1: to show unknown file extensions,
            2: reserved for future debugging,
            3: enable file level trace. Not recommended on multiple files")
        ).get_matches();

        let columns = matches.value_of("columns").map(parse_or_exit::<usize>)
            .unwrap_or_else(|| {
                // If user passed `--files`, we output file paths and want to
                // use all available width.
                if matches.is_present("files") {
                    ::term_size::dimensions().map_or(FALLBACK_ROW_LEN, |(w, _)| {
                        w.max(FALLBACK_ROW_LEN)
                    })
                } else {
                    FALLBACK_ROW_LEN
                }
            });


        let files = matches.is_present("files");
        let print_languages = matches.is_present("languages");
        let verbose = matches.occurrences_of("verbose");
        let types = matches.value_of("types").map(|e| {
            e.split(',')
             .map(|t| t.parse::<LanguageType>())
             .filter_map(Result::ok)
             .collect()
        });

        // Sorting category should be restricted by clap but parse before we do
        // work just in case.
        let sort = matches.value_of("sort").map(parse_or_exit::<Sort>);
        // Format category is overly accepting by clap (so the user knows what
        // is supported) but this will fail if support is not compiled in and
        // give a useful error to the user.
        let output = matches.value_of("output")
                            .map(parse_or_exit::<Format>);

        Cli {
            matches,
            output,
            files,
            print_languages,
            sort,
            types,
            verbose,
            columns,
        }
    }

    pub fn file_input(&self) ->  Option<&str> {
        self.matches.value_of("file_input")
    }

    pub fn ignored_directories(&self) -> Vec<&str> {
        let mut ignored_directories: Vec<&str> = Vec::new();
        if let Some(user_ignored) = self.matches.values_of("exclude") {
            ignored_directories.extend(user_ignored);
        }
        ignored_directories
    }

    pub fn input(&self) -> Vec<&str> {
        match self.matches.values_of("input") {
            Some(vs) => vs.collect(),
            None => vec!["."],
        }
    }

    pub fn print_supported_languages() {
        for key in LanguageType::list() {
            println!("{:<25}", key);
        }
    }

    pub fn print_input_parse_failure(input_filename: &str) {
        eprintln!("Error:\n Failed to parse input file: {}", input_filename);

        let not_supported = Format::not_supported();
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
}

