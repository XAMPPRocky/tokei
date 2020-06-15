use std::{
    fmt,
    io::{self, Write},
    process,
    str::FromStr,
};

use clap::crate_version;

use crate::input::Format;
use tokei::{CodeStats, Language, LanguageType, Report};

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
    let (a, b): (Vec<_>, Vec<_>) = languages
        .filter(isnt_empty)
        .partition(|(_, l)| l.children.is_empty());
    let path_len = row.len() - NO_LANG_ROW_LEN_NO_SPACES;
    let columns = row.len();
    let mut first = true;

    for languages in &[&a, &b] {
        for &(name, language) in *languages {
            let has_children = !language.children.is_empty();
            if first {
                first = false;
            } else if has_children || list_files {
                writeln!(sink, "{}", row)?;
            }

            print_language(sink, columns, language, name.name(), None)?;
            if has_children {
                print_language_total(language, sink, columns, path_len)?;
            }

            if list_files {
                writeln!(sink, "{}", row)?;
                let (a, b): (Vec<_>, Vec<_>) = language
                    .reports
                    .iter()
                    .partition(|r| r.stats.contexts.is_empty());
                for reports in &[a, b] {
                    for report in reports {
                        writeln!(sink, "{:1$}", report, path_len)?;
                        if !report.stats.contexts.is_empty() {
                            print_report_total(language, &report, sink, columns, path_len)?;
                        }
                    }
                }
            }
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

fn print_language_total<W: Write>(
    parent: &Language,
    sink: &mut W,
    columns: usize,
    path_len: usize,
) -> io::Result<()> {
    let mut subtotal = tokei::Report::new(format!("(Total)").into());
    subtotal.stats.code += parent.code;
    subtotal.stats.comments += parent.comments;
    subtotal.stats.blanks += parent.blanks;

    // writeln!(sink, "{}", row)?;
    for (language_type, stats) in &parent.children {
        print_language_name(
            sink,
            columns,
            parent,
            &language_type.to_string(),
            Some(" |-"),
        )?;
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

    writeln!(sink, "{:1$}", subtotal, path_len)?;

    Ok(())
}

fn print_report_total<W: Write>(
    parent: &Language,
    report: &Report,
    sink: &mut W,
    columns: usize,
    path_len: usize,
) -> io::Result<()> {
    let mut subtotal = tokei::Report::new(format!("|- (Total)").into());
    subtotal.stats.code += report.stats.code;
    subtotal.stats.comments += report.stats.comments;
    subtotal.stats.blanks += report.stats.blanks;

    fn print_report<W: Write>(
        parent: &Language,
        language_type: LanguageType,
        stats: &CodeStats,
        sink: &mut W,
        columns: usize,
    ) -> io::Result<()> {
        print_language_name(
            sink,
            columns,
            parent,
            &language_type.to_string(),
            Some(" |-"),
        )?;

        writeln!(
            sink,
            " {:>6} {:>12} {:>12} {:>12} {:>12}",
            " ",
            stats.lines(),
            stats.code,
            stats.comments,
            stats.blanks,
        )
    }

    // writeln!(sink, "{}", row)?;
    for (language_type, stats) in &report.stats.contexts {
        print_report(parent, *language_type, stats, sink, columns)?;
        subtotal.stats.code += stats.code;
        subtotal.stats.comments += stats.comments;
        subtotal.stats.blanks += stats.blanks;
    }

    writeln!(sink, "{:1$}", subtotal, path_len)?;

    Ok(())
}
