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

pub struct Printer<W> {
    writer: W,
    columns: usize,
    path_length: usize,
    row: String,
    subrow: String,
    list_files: bool,
}

impl<W> Printer<W> {
    pub fn new(columns: usize, list_files: bool, writer: W) -> Self {
        Self {
            columns,
            list_files,
            path_length: columns - NO_LANG_ROW_LEN_NO_SPACES,
            writer,
            row: "=".repeat(columns),
            subrow: "-".repeat(columns),
        }
    }
}

impl<W: Write> Printer<W> {
    pub fn print_header(&mut self) -> io::Result<()> {
        self.print_row()?;
        writeln!(
            self.writer,
            " {:<6$} {:>12} {:>12} {:>12} {:>12} {:>12}",
            "Language",
            "Files",
            "Lines",
            "Code",
            "Comments",
            "Blanks",
            self.columns - NO_LANG_HEADER_ROW_LEN
        )?;
        self.print_row()
    }

    pub fn print_inaccuracy_warning(&mut self) -> io::Result<()> {
        writeln!(
            self.writer,
            "Note: results can be inaccurate for languages marked with '{}'",
            IDENT_INACCURATE
        )
    }

    pub fn print_language(&mut self, language: &Language, name: &str) -> io::Result<()>
    where
        W: Write,
    {
        self.print_language_name(language.inaccurate, name, None)?;
        write!(self.writer, " ")?;
        writeln!(
            self.writer,
            "{:>6} {:>12} {:>12} {:>12} {:>12}",
            language.reports.len(),
            language.lines(),
            language.code,
            language.comments,
            language.blanks
        )
    }

    pub fn print_language_name(
        &mut self,
        inaccurate: bool,
        name: &str,
        prefix: Option<&str>,
    ) -> io::Result<()> {
        let mut lang_section_len =
            self.columns - NO_LANG_ROW_LEN - prefix.as_deref().map_or(0, str::len);
        if inaccurate {
            lang_section_len -= IDENT_INACCURATE.len();
        }

        if let Some(prefix) = prefix {
            write!(self.writer, "{}", prefix)?;
        }
        // truncate and replace the last char with a `|` if the name is too long
        if lang_section_len < name.len() {
            write!(self.writer, " {:.len$}", name, len = lang_section_len - 1)?;
            write!(self.writer, "|")?;
        } else {
            write!(self.writer, " {:<len$}", name, len = lang_section_len)?;
        }
        if inaccurate {
            write!(self.writer, "{}", IDENT_INACCURATE)?;
        };

        Ok(())
    }

    fn print_code_stats<'a, 'b>(
        &mut self,
        language_type: LanguageType,
        stats: &[CodeStats],
    ) -> io::Result<()> {
        self.print_language_name(false, &language_type.to_string(), Some(&(" |-")))?;
        let mut code = 0;
        let mut comments = 0;
        let mut blanks = 0;

        for stats in stats.iter().map(|s| s.summarise()) {
            code += stats.code;
            comments += stats.comments;
            blanks += stats.blanks;
        }

        if !stats.is_empty() {
            writeln!(
                self.writer,
                " {:>6} {:>12} {:>12} {:>12} {:>12}",
                stats.len(),
                code + comments + blanks,
                code,
                comments,
                blanks,
            )
        } else {
            Ok(())
        }
    }

    fn print_language_total(&mut self, parent: &Language) -> io::Result<()> {
        for (language, reports) in &parent.children {
            self.print_code_stats(
                *language,
                &reports
                    .iter()
                    .map(|r| r.stats.summarise())
                    .collect::<Vec<_>>(),
            )?;
        }
        let mut subtotal = tokei::Report::new(format!("(Total)").into());
        let summary = parent.summarise();
        subtotal.stats.code += summary.code;
        subtotal.stats.comments += summary.comments;
        subtotal.stats.blanks += summary.blanks;
        writeln!(self.writer, "{:1$}", subtotal, self.path_length)?;

        Ok(())
    }

    pub fn print_results<'a, I>(&mut self, languages: I) -> io::Result<()>
    where
        I: Iterator<Item = (&'a LanguageType, &'a Language)>,
    {
        let (a, b): (Vec<_>, Vec<_>) = languages
            .filter(|(_, v)| !v.is_empty())
            .partition(|(_, l)| l.children.is_empty());
        let mut first = true;

        for languages in &[&a, &b] {
            for &(name, language) in *languages {
                let has_children = !language.children.is_empty();
                if first {
                    first = false;
                } else if has_children || self.list_files {
                    self.print_subrow()?;
                }

                self.print_language(language, name.name())?;
                if has_children {
                    self.print_language_total(language)?;
                }

                if self.list_files {
                    self.print_subrow()?;
                    let (a, b): (Vec<_>, Vec<_>) = language
                        .reports
                        .iter()
                        .partition(|r| r.stats.contexts.is_empty());
                    for reports in &[&a, &b] {
                        let mut first = true;
                        for report in reports.iter() {
                            if !report.stats.contexts.is_empty() {
                                if first && a.is_empty() {
                                    writeln!(self.writer, " {}", report.name.display())?;
                                    first = false;
                                } else {
                                    writeln!(
                                        self.writer,
                                        " {} {}",
                                        report.name.display(),
                                        "-".repeat(
                                            self.columns
                                                - 2
                                                - report.name.display().to_string().len()
                                        )
                                    )?;
                                }
                                let mut new_report = (*report).clone();
                                new_report.name = name.to_string().into();
                                writeln!(
                                    self.writer,
                                    " |-{:1$}",
                                    new_report,
                                    self.path_length - 3
                                )?;
                                self.print_report_total(&report, language.inaccurate)?;
                            } else {
                                writeln!(self.writer, "{:1$}", report, self.path_length)?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn print_row(&mut self) -> io::Result<()> {
        writeln!(self.writer, "{}", self.row)
    }

    fn print_subrow(&mut self) -> io::Result<()> {
        writeln!(self.writer, "{}", self.subrow)
    }

    fn print_report(
        &mut self,
        language_type: LanguageType,
        stats: &CodeStats,
        inaccurate: bool,
    ) -> io::Result<()> {
        self.print_language_name(inaccurate, &language_type.to_string(), Some(" |-"))?;

        writeln!(
            self.writer,
            " {:>6} {:>12} {:>12} {:>12} {:>12}",
            " ",
            stats.lines(),
            stats.code,
            stats.comments,
            stats.blanks,
        )
    }

    fn print_report_total(&mut self, report: &Report, inaccurate: bool) -> io::Result<()> {
        if report.stats.contexts.is_empty() {
            return Ok(());
        }

        let mut subtotal = tokei::Report::new(format!("|- (Total)").into());
        subtotal.stats.code += report.stats.code;
        subtotal.stats.comments += report.stats.comments;
        subtotal.stats.blanks += report.stats.blanks;

        // writeln!(sink, "{}", row)?;
        for (language_type, stats) in &report.stats.contexts {
            self.print_report(*language_type, stats, inaccurate)?;
            subtotal.stats += stats.summarise();
        }

        writeln!(self.writer, "{:1$}", subtotal, self.path_length)?;

        Ok(())
    }

    pub fn print_total(&mut self, languages: tokei::Languages) -> io::Result<()> {
        let mut total = Language::new();

        for (_, language) in languages {
            total += language.summarise();
        }

        self.print_row()?;
        self.print_language(&total, "Total")?;
        self.print_row()
    }
}
