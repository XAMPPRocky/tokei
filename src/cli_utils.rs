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

    pub fn print_inaccuracy_warning(&mut self) -> io::Result<()>
    {
        writeln!(
            self.writer,
            "Note: results can be inaccurate for languages marked with '{}'",
            IDENT_INACCURATE
        )
    }

    pub fn print_language(&mut self, language: &Language, name: &str, prefix: Option<&str>) -> io::Result<()>
    where
        W: Write,
    {
        self.print_language_name(language, name, prefix)?;
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

    pub fn print_language_name(&mut self, language: &Language, name: &str, prefix: Option<&str>) -> io::Result<()>
    {
        let mut lang_section_len = self.columns - NO_LANG_ROW_LEN - prefix.as_deref().map_or(0, str::len);
        if language.inaccurate {
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
        if language.inaccurate {
            write!(self.writer, "{}", IDENT_INACCURATE)?;
        };

        Ok(())
    }

    fn print_language_total(&mut self, parent: &Language) -> io::Result<()> {
        let mut subtotal = tokei::Report::new(format!("(Total)").into());
        subtotal.stats.code += parent.code;
        subtotal.stats.comments += parent.comments;
        subtotal.stats.blanks += parent.blanks;

        for (language_type, stats) in &parent.children {
            self.print_language_name(
                parent,
                &language_type.to_string(),
                Some(" |-"),
            )?;
            let code = stats.iter().map(|r| r.stats.code).sum::<usize>();
            let comments = stats.iter().map(|r| r.stats.comments).sum::<usize>();
            let blanks = stats.iter().map(|r| r.stats.blanks).sum::<usize>();
            let lines = code + comments + blanks;

            writeln!(
                self.writer,
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

        writeln!(self.writer, "{:1$}", subtotal, self.path_length)?;

        Ok(())
    }


    pub fn print_results<'a, I>(
        &mut self,
        languages: I,
    ) -> io::Result<()>
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

                self.print_language(language, name.name(), None)?;
                if has_children {
                    self.print_language_total(language)?;
                }

                if self.list_files {
                    self.print_subrow()?;
                    let (a, b): (Vec<_>, Vec<_>) = language
                                 .reports
                                 .iter()
                                 .partition(|r| r.stats.contexts.is_empty());
                    for reports in &[a, b] {
                        for report in reports {
                            writeln!(self.writer, "{:1$}", report, self.path_length)?;
                            if !report.stats.contexts.is_empty() {
                                self.print_report_total(language, &report)?;
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
        parent: &Language,
        language_type: LanguageType,
        stats: &CodeStats,
    ) -> io::Result<()> {
        self.print_language_name(
            parent,
            &language_type.to_string(),
            Some(" |-"),
        )?;

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

    fn print_report_total(
        &mut self,
        parent: &Language,
        report: &Report,
    ) -> io::Result<()> {
        let mut subtotal = tokei::Report::new(format!("|- (Total)").into());
        subtotal.stats.code += report.stats.code;
        subtotal.stats.comments += report.stats.comments;
        subtotal.stats.blanks += report.stats.blanks;

        // writeln!(sink, "{}", row)?;
        for (language_type, stats) in &report.stats.contexts {
            self.print_report(parent, *language_type, stats)?;
            subtotal.stats.code += stats.code;
            subtotal.stats.comments += stats.comments;
            subtotal.stats.blanks += stats.blanks;

            for (language_type, stats) in dbg!(&stats.contexts) {
                self.print_report(parent, *language_type, stats)?;
                subtotal.stats.code += stats.code;
                subtotal.stats.comments += stats.comments;
                subtotal.stats.blanks += stats.blanks;
            }
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
        self.print_language(&total, "Total", None)?;
        self.print_row()
    }
}
