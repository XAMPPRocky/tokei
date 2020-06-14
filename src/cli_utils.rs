use std::{
    fmt,
    io::{self, Write},
    process,
    str::FromStr,
};

use clap::crate_version;

use crate::input::Format;
use tokei::{Language, LanguageType};

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

pub fn print_results<'a, I, W>(
    sink: &mut W,
    row: &str,
    languages: I,
    list_files: bool,
) -> io::Result<()>
where
    I: Iterator<Item = (&'a LanguageType, &'a Language)>,
    W: Write,
{
    let path_len = row.len() - NO_LANG_ROW_LEN_NO_SPACES;
    let columns = row.len();
    for (name, language) in languages.filter(isnt_empty) {
        let has_children = !language.children.is_empty();

        //writeln!(sink, "{}", row)?;
        if has_children {
        //    writeln!(sink, "{}", row)?;

        }


        if has_children {
            writeln!(sink, "{}", row)?;
            print_language(sink, columns, language, name.name(), Some("|"))?;
            let mut subtotal = tokei::Report::new(format!("{} (Total)", name.name()).into());
            subtotal.stats.code += language.code;
            subtotal.stats.comments += language.comments;
            subtotal.stats.blanks += language.blanks;

            // writeln!(sink, "{}", row)?;
            for (language_type, stats) in &language.children {
                print_language_name(sink, columns, language, &language_type.to_string(), Some("| >"))?;
                let code = stats.iter().map(|r| r.stats.code).sum::<usize>();
                let comments = stats.iter().map(|r| r.stats.comments).sum::<usize>();
                let blanks = stats.iter().map(|r| r.stats.blanks).sum::<usize>();
                let lines = code + comments + blanks;

                writeln!(
                    sink,
                    " {:>6} {:>12} {:>12} {:>12} {:>12}",
                    stats.len(),
                    lines,
                    code,
                    comments,
                    blanks,
                )?;

                subtotal.stats.code += code;
                subtotal.stats.comments += comments;
                subtotal.stats.blanks += blanks;
            }
            writeln!(sink, "|{:1$}", subtotal, path_len-1)?;
            writeln!(sink, "{}", row)?;
        } else {
            print_language(sink, columns, language, name.name(), None)?;
        }

        if list_files {
            writeln!(sink, "{}", row)?;
            for stat in &language.reports {
                writeln!(sink, "{:1$}", stat, path_len)?;
            }
            writeln!(sink, "{}", row)?;
        }
    }

    Ok(())
}

pub fn isnt_empty(&(_, language): &(&LanguageType, &Language)) -> bool {
    !language.is_empty()
}

pub fn print_language_name<W>(
    sink: &mut W,
    columns: usize,
    language: &Language,
    name: &str,
    prefix: Option<&str>,
) -> io::Result<()>
where
    W: Write,
{
    let mut lang_section_len = columns - NO_LANG_ROW_LEN - prefix.as_deref().map_or(0, str::len);
    if language.inaccurate {
        lang_section_len -= IDENT_INACCURATE.len();
    }

    if let Some(prefix) = prefix {
        write!(sink, "{}", prefix)?;
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

    Ok(())
}

pub fn print_language<W>(
    sink: &mut W,
    columns: usize,
    language: &Language,
    name: &str,
    prefix: Option<&str>,
) -> io::Result<()>
where
    W: Write,
{
    print_language_name(sink, columns, language, name, prefix)?;
    write!(sink, " ")?;
    writeln!(
        sink,
        "{:>6} {:>12} {:>12} {:>12} {:>12}",
        language.reports.len(),
        language.lines,
        language.code,
        language.comments,
        language.blanks
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
