use std::mem;
use std::process;

use clap::Arg;
use clap::{crate_description, ArgMatches};
use colored::Colorize;
use tokei::{Config, LanguageType, Sort};

use crate::{
    cli_utils::{crate_version, parse_or_exit, NumberFormatStyle},
    input::Format,
};

/// Used for sorting languages.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Streaming {
    /// simple lines.
    Simple,
    /// Json outputs.
    Json,
}

impl std::str::FromStr for Streaming {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_ref() {
            "simple" => Streaming::Simple,
            "json" => Streaming::Json,
            s => return Err(format!("Unsupported streaming option: {}", s)),
        })
    }
}

#[derive(Debug)]
pub struct Cli {
    matches: ArgMatches,
    pub columns: Option<usize>,
    pub files: bool,
    pub hidden: bool,
    pub no_ignore: bool,
    pub no_ignore_parent: bool,
    pub no_ignore_dot: bool,
    pub no_ignore_vcs: bool,
    pub output: Option<Format>,
    pub streaming: Option<Streaming>,
    pub print_languages: bool,
    pub sort: Option<Sort>,
    pub sort_reverse: bool,
    pub types: Option<Vec<LanguageType>>,
    pub compact: bool,
    pub number_format: num_format::CustomFormat,
    pub verbose: u64,
}

impl Cli {
    pub fn from_args() -> Self {
        let matches = clap::App::new("tokei")
            .version(&*crate_version())
            .author("Erin P. <xampprocky@gmail.com> + Contributors")
            .about(concat!(
                crate_description!(),
                "\n",
                "Support this project on GitHub Sponsors: https://github.com/sponsors/XAMPPRocky"
            ))
            .arg(
                Arg::new("columns")
                    .long("columns")
                    .short('c')
                    .takes_value(true)
                    .conflicts_with("output")
                    .help(
                        "Sets a strict column width of the output, only available for \
                        terminal output.",
                    ),
            )
            .arg(
                Arg::new("exclude")
                    .long("exclude")
                    .short('e')
                    .takes_value(true)
                    .multiple_values(true)
                    .help("Ignore all files & directories matching the pattern."),
            )
            .arg(
                Arg::new("files")
                    .long("files")
                    .short('f')
                    .help("Will print out statistics on individual files."),
            )
            .arg(
                Arg::new("file_input")
                    .long("input")
                    .short('i')
                    .takes_value(true)
                    .help(
                        "Gives statistics from a previous tokei run. Can be given a file path, \
                        or \"stdin\" to read from stdin.",
                    ),
            )
            .arg(
                Arg::new("hidden")
                    .long("hidden")
                    .help("Count hidden files."),
            )
            .arg(
                Arg::new("input")
                    .min_values(1)
                    .conflicts_with("languages")
                    .help("The path(s) to the file or directory to be counted.(default current directory)"),
            )
            .arg(
                Arg::new("languages")
                    .long("languages")
                    .short('l')
                    .conflicts_with("input")
                    .help("Prints out supported languages and their extensions."),
            )
            .arg(Arg::new("no_ignore").long("no-ignore").help(
                "Don't respect ignore files (.gitignore, .ignore, etc.). This implies \
                --no-ignore-parent, --no-ignore-dot, and --no-ignore-vcs.",
            ))
            .arg(Arg::new("no_ignore_parent").long("no-ignore-parent").help(
                "Don't respect ignore files (.gitignore, .ignore, etc.) in parent \
                directories.",
            ))
            .arg(Arg::new("no_ignore_dot").long("no-ignore-dot").help(
                "Don't respect .ignore and .tokeignore files, including this in \
                parent directories.",
            ))
            .arg(Arg::new("no_ignore_vcs").long("no-ignore-vcs").help(
                "Don't respect VCS ignore files (.gitignore, .hgignore, etc.) including \
                those in parent directories.",
            ))
            .arg(
                Arg::new("output")
                    .long("output")
                    .short('o')
                    .takes_value(true)
                    .possible_values(Format::all())
                    .help(
                        "Outputs Tokei in a specific format. Compile with additional features for \
                        more format support.",
                    ),
            )
            .arg(
                Arg::new("streaming")
                    .long("streaming")
                    .takes_value(true)
                    .possible_values(&["simple", "json"])
                    .ignore_case(true)
                    .help(
                        "prints the (language, path, lines, blanks, code, comments) records as \
                        simple lines or as Json for batch processing",
                    ),
            )
            .arg(
                Arg::new("sort")
                    .long("sort")
                    .short('s')
                    .takes_value(true)
                    .possible_values(&["files", "lines", "blanks", "code", "comments"])
                    .ignore_case(true)
                    .conflicts_with("rsort")
                    .help("Sort languages based on column"),
            )
            .arg(
                Arg::new("rsort")
                    .long("rsort")
                    .short('r')
                    .takes_value(true)
                    .possible_values(&["files", "lines", "blanks", "code", "comments"])
                    .ignore_case(true)
                    .conflicts_with("sort")
                    .help("Reverse sort languages based on column"),
            )
            .arg(
                Arg::new("types")
                    .long("types")
                    .short('t')
                    .takes_value(true)
                    .help(
                        "Filters output by language type, separated by a comma. i.e. \
                        -t=Rust,Markdown",
                    ),
            )
            .arg(
                Arg::new("compact")
                    .long("compact")
                    .short('C')
                    .help("Do not print statistics about embedded languages."),
            )
            .arg(
                Arg::new("num_format_style")
                    .long("num-format")
                    .short('n')
                    .takes_value(true)
                    .possible_values(NumberFormatStyle::all())
                    .conflicts_with("output")
                    .help(
                        "Format of printed numbers, i.e., plain (1234, default), \
                        commas (1,234), dots (1.234), or underscores (1_234). Cannot be \
                        used with --output.",
                    ),
            )
            .arg(
                Arg::new("verbose")
                    .long("verbose")
                    .short('v')
                    .multiple_occurrences(true)
                    .help(
                        "Set log output level:
                        1: to show unknown file extensions,
                        2: reserved for future debugging,
                        3: enable file level trace. Not recommended on multiple files",
                    ),
            )
            .get_matches();

        let columns = matches.value_of("columns").map(parse_or_exit::<usize>);
        let files = matches.is_present("files");
        let hidden = matches.is_present("hidden");
        let no_ignore = matches.is_present("no_ignore");
        let no_ignore_parent = matches.is_present("no_ignore_parent");
        let no_ignore_dot = matches.is_present("no_ignore_dot");
        let no_ignore_vcs = matches.is_present("no_ignore_vcs");
        let print_languages = matches.is_present("languages");
        let verbose = matches.occurrences_of("verbose");
        let compact = matches.is_present("compact");
        let types = matches.value_of("types").map(|e| {
            e.split(',')
                .map(str::parse::<LanguageType>)
                .filter_map(Result::ok)
                .collect()
        });

        let num_format_style: NumberFormatStyle = matches
            .value_of("num_format_style")
            .map(parse_or_exit::<NumberFormatStyle>)
            .unwrap_or_default();

        let number_format = match num_format_style.get_format() {
            Ok(format) => format,
            Err(e) => {
                eprintln!("Error:\n{}", e);
                process::exit(1);
            }
        };

        // Sorting category should be restricted by clap but parse before we do
        // work just in case.
        let sort = matches
            .value_of("sort")
            .or_else(|| matches.value_of("rsort"))
            .map(parse_or_exit::<Sort>);
        let sort_reverse = matches.value_of("rsort").is_some();

        // Format category is overly accepting by clap (so the user knows what
        // is supported) but this will fail if support is not compiled in and
        // give a useful error to the user.
        let output = matches.value_of("output").map(parse_or_exit::<Format>);
        let streaming = matches
            .value_of("streaming")
            .map(parse_or_exit::<Streaming>);

        crate::cli_utils::setup_logger(verbose);

        let cli = Cli {
            matches,
            columns,
            files,
            hidden,
            no_ignore,
            no_ignore_parent,
            no_ignore_dot,
            no_ignore_vcs,
            output,
            streaming,
            print_languages,
            sort,
            sort_reverse,
            types,
            compact,
            number_format,
            verbose,
        };

        debug!("CLI Config: {:#?}", cli);

        cli
    }

    pub fn file_input(&self) -> Option<&str> {
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

    pub fn print_supported_languages() -> Result<(), Box<dyn std::error::Error>> {
        use table_formatter::table::*;
        use table_formatter::{cell, table};
        let term_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(75) - 8;
        let (lang_w, suffix_w) = if term_width <= 80 {
            (term_width / 2, term_width / 2)
        } else {
            (40, term_width - 40)
        };

        let header = vec![
            cell!(
                "Language",
                align = Align::Left,
                padding = Padding::NONE,
                width = Some(lang_w)
            )
            .with_formatter(vec![Colorize::bold]),
            cell!(
                "Extensions",
                align = Align::Left,
                padding = Padding::new(3, 0),
                width = Some(suffix_w)
            )
            .with_formatter(vec![Colorize::bold]),
        ];
        let content = LanguageType::list()
            .iter()
            .map(|(key, ext)| {
                vec![
                    // table::TableCell::new(table::Cell::TextCell(key.name().to_string()))
                    //     .with_width(lang_w),
                    cell!(key.name()).with_width(Some(lang_w)),
                    cell!(
                        if matches!(key, LanguageType::Emojicode) {
                            ext.join(", ") + "\u{200b}"
                        } else if ext.is_empty() {
                            "<None>".to_string()
                        } else {
                            ext.join(", ")
                        },
                        align = Align::Left,
                        padding = Padding::new(3, 0),
                        width = Some(suffix_w)
                    ),
                ]
            })
            .collect();
        let t = table!(header - content with Border::ALL);

        let mut render_result = Vec::new();
        t.render(&mut render_result)?;
        println!("{}", String::from_utf8(render_result)?);
        Ok(())
    }

    /// Overrides the shared options (See `tokei::Config` for option
    /// descriptions) between the CLI and the config files. CLI flags have
    /// higher precedence than options present in config files.
    ///
    /// #### Shared options
    /// * `no_ignore`
    /// * `no_ignore_parent`
    /// * `no_ignore_dot`
    /// * `no_ignore_vcs`
    /// * `types`
    pub fn override_config(&mut self, mut config: Config) -> Config {
        config.hidden = if self.hidden {
            Some(true)
        } else {
            config.hidden
        };

        config.no_ignore = if self.no_ignore {
            Some(true)
        } else {
            config.no_ignore
        };

        config.no_ignore_parent = if self.no_ignore_parent {
            Some(true)
        } else {
            config.no_ignore_parent
        };

        config.no_ignore_dot = if self.no_ignore_dot {
            Some(true)
        } else {
            config.no_ignore_dot
        };

        config.no_ignore_vcs = if self.no_ignore_vcs {
            Some(true)
        } else {
            config.no_ignore_vcs
        };

        config.for_each_fn = match self.streaming {
            Some(Streaming::Json) => Some(|l: LanguageType, e| {
                println!("{}", serde_json::json!({"language": l.name(), "stats": e}));
            }),
            Some(Streaming::Simple) => Some(|l: LanguageType, e| {
                println!(
                    "{:>10} {:<80} {:>12} {:>12} {:>12} {:>12}",
                    l.name(),
                    e.name.to_string_lossy().to_string(),
                    e.stats.lines(),
                    e.stats.code,
                    e.stats.comments,
                    e.stats.blanks
                );
            }),
            _ => None,
        };

        config.types = mem::replace(&mut self.types, None).or(config.types);

        config
    }

    pub fn print_input_parse_failure(input_filename: &str) {
        eprintln!("Error:\n Failed to parse input file: {}", input_filename);

        let not_supported = Format::not_supported();
        if !not_supported.is_empty() {
            eprintln!(
                "
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
