use std::{
    fmt,
    io::{self, Write},
    process,
    str::FromStr,
};

use clap::crate_version;
use num_format::{Format as NumFormat, ToFormattedString};

use crate::input::Format;
use tokei::{Language, LanguageType, Stats};

pub const FALLBACK_ROW_LEN: usize = 79;
const NO_LANG_HEADER_ROW_LEN: usize = 67;
const NO_LANG_ROW_LEN: usize = 61;
const NO_LANG_ROW_LEN_NO_SPACES: usize = 54;
const IDENT_INACCURATE: &str = "(!)";

pub fn crate_version() -> String {
    if Format::supported().is_empty() {
        format!(
            "{} compiled without serialization formats.",
            crate_version!()
        )
    } else {
        format!(
            "{} compiled with serialization support: {}",
            crate_version!(),
            Format::supported().join(", ")
        )
    }
}

pub fn setup_logger(verbose_option: u64) {
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

pub fn parse_or_exit<T>(s: &str) -> T
where
    T: FromStr,
    T::Err: fmt::Display,
{
    T::from_str(s).unwrap_or_else(|e| {
        eprintln!("Error:\n{}", e);
        process::exit(1);
    })
}

pub fn print_header<W: Write>(sink: &mut W, row: &str, columns: usize) -> io::Result<()> {
    writeln!(sink, "{}", row)?;
    writeln!(
        sink,
        " {:<6$} {:>12} {:>12} {:>12} {:>12} {:>12}",
        "Language",
        "Files",
        "Lines",
        "Code",
        "Comments",
        "Blanks",
        columns - NO_LANG_HEADER_ROW_LEN
    )?;

    writeln!(sink, "{}", row)
}

pub fn print_results<'a, I, W, F>(
    sink: &mut W,
    row: &str,
    languages: I,
    list_files: bool,
    num_format: &F,
) -> io::Result<()>
where
    I: Iterator<Item = (&'a LanguageType, &'a Language)>,
    W: Write,
    F: NumFormat,
{
    let path_len = row.len() - NO_LANG_ROW_LEN_NO_SPACES;
    let columns = row.len();
    for (name, language) in languages.filter(isnt_empty) {
        print_language(sink, columns, language, name.name(), num_format)?;

        if list_files {
            writeln!(sink, "{}", row)?;
            for stat in &language.stats {
                writeln!(
                    sink,
                    "{:1$}",
                    FormattableStats::new(stat, num_format),
                    path_len
                )?;
            }
            writeln!(sink, "{}", row)?;
        }
    }

    Ok(())
}

pub fn isnt_empty(&(_, language): &(&LanguageType, &Language)) -> bool {
    !language.is_empty()
}

pub fn print_language<W, F>(
    sink: &mut W,
    columns: usize,
    language: &Language,
    name: &str,
    num_format: &F,
) -> io::Result<()>
where
    W: Write,
    F: NumFormat,
{
    let mut lang_section_len = columns - NO_LANG_ROW_LEN;
    if language.inaccurate {
        lang_section_len -= IDENT_INACCURATE.len();
    }
    // truncate and replace the last char with a `|` if the name is too long
    if lang_section_len < name.len() {
        write!(sink, " {:.len$}", name, len = lang_section_len - 1)?;
        write!(sink, "|")?;
    } else {
        write!(sink, " {:<len$}", name, len = lang_section_len)?;
    }
    if language.inaccurate {
        write!(sink, "{}", IDENT_INACCURATE)?;
    };
    write!(sink, " ")?;
    writeln!(
        sink,
        "{:>6} {:>12} {:>12} {:>12} {:>12}",
        language.stats.len().to_formatted_string(num_format),
        language.lines.to_formatted_string(num_format),
        language.code.to_formatted_string(num_format),
        language.comments.to_formatted_string(num_format),
        language.blanks.to_formatted_string(num_format)
    )
}

pub fn print_inaccuracy_warning<W>(sink: &mut W) -> io::Result<()>
where
    W: Write,
{
    writeln!(
        sink,
        "Note: results can be inaccurate for languages marked with '{}'",
        IDENT_INACCURATE
    )
}

pub struct FormattableStats<'a, F: NumFormat> {
    stats: &'a Stats,
    num_format: &'a F,
}

impl<'a, F: NumFormat> FormattableStats<'a, F> {
    pub fn new(stats: &'a Stats, num_format: &'a F) -> Self {
        FormattableStats { stats, num_format }
    }
}

macro_rules! display_stats {
    ($f:expr, $this:expr, $name:expr, $max:expr, $format:expr) => {
        write!(
            $f,
            " {: <max$} {:>12} {:>12} {:>12} {:>12}",
            $name,
            $this.lines.to_formatted_string($format),
            $this.code.to_formatted_string($format),
            $this.comments.to_formatted_string($format),
            $this.blanks.to_formatted_string($format),
            max = $max
        )
    };
}

fn find_char_boundary(s: &str, index: usize) -> usize {
    for i in 0..4 {
        if s.is_char_boundary(index + i) {
            return index + i;
        }
    }
    unreachable!();
}

impl<'a, F: NumFormat> fmt::Display for FormattableStats<'a, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.stats.name.to_string_lossy();
        let name_length = name.len();

        let max_len = f.width().unwrap_or(25);

        if name_length <= max_len {
            display_stats!(f, self.stats, name, max_len, self.num_format)
        } else {
            let mut formatted = String::from("|");
            // Add 1 to the index to account for the '|' we add to the output string
            let from = find_char_boundary(&name, name_length + 1 - max_len);
            formatted.push_str(&name[from..]);
            display_stats!(f, self.stats, formatted, max_len, self.num_format)
        }
    }
}
