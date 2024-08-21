use std::{
    borrow::Cow,
    fmt,
    io::{self, Write},
    process,
    str::FromStr,
};

use clap::crate_version;
use colored::Colorize;
use num_format::ToFormattedString;

use crate::input::Format;
use tokei::{find_char_boundary, CodeStats, Language, LanguageType, Report};

use crate::consts::{
    BLANKS_COLUMN_WIDTH, CODE_COLUMN_WIDTH, COMMENTS_COLUMN_WIDTH, FILES_COLUMN_WIDTH,
    LINES_COLUMN_WIDTH,
};

const NO_LANG_HEADER_ROW_LEN: usize = 69;
const NO_LANG_ROW_LEN: usize = 63;
const NO_LANG_ROW_LEN_NO_SPACES: usize = 56;
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

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum NumberFormatStyle {
    // 1234 (Default)
    Plain,
    // 1,234
    Commas,
    // 1.234
    Dots,
    // 1_234
    Underscores,
}

impl Default for NumberFormatStyle {
    fn default() -> Self {
        Self::Plain
    }
}

impl FromStr for NumberFormatStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(Self::Plain),
            "commas" => Ok(Self::Commas),
            "dots" => Ok(Self::Dots),
            "underscores" => Ok(Self::Underscores),
            _ => Err(format!(
                "Expected 'plain', 'commas', 'underscores', or 'dots' for num-format, but got '{}'",
                s,
            )),
        }
    }
}

impl NumberFormatStyle {
    fn separator(self) -> &'static str {
        match self {
            Self::Plain => "",
            Self::Commas => ",",
            Self::Dots => ".",
            Self::Underscores => "_",
        }
    }

    pub fn get_format(self) -> Result<num_format::CustomFormat, num_format::Error> {
        num_format::CustomFormat::builder()
            .grouping(num_format::Grouping::Standard)
            .separator(self.separator())
            .build()
    }
}

pub struct Printer<W> {
    writer: W,
    columns: usize,
    path_length: usize,
    row: String,
    subrow: String,
    list_files: bool,
    number_format: num_format::CustomFormat,
}

impl<W> Printer<W> {
    pub fn new(
        columns: usize,
        list_files: bool,
        writer: W,
        number_format: num_format::CustomFormat,
    ) -> Self {
        Self {
            columns,
            list_files,
            path_length: columns - NO_LANG_ROW_LEN_NO_SPACES,
            writer,
            row: "━".repeat(columns),
            subrow: "─".repeat(columns),
            number_format,
        }
    }
}

impl<W: Write> Printer<W> {
    pub fn print_header(&mut self) -> io::Result<()> {
        self.print_row()?;

        let files_column_width: usize = FILES_COLUMN_WIDTH + 6;
        writeln!(
            self.writer,
            " {:<6$} {:>files_column_width$} {:>LINES_COLUMN_WIDTH$} {:>CODE_COLUMN_WIDTH$} {:>COMMENTS_COLUMN_WIDTH$} {:>BLANKS_COLUMN_WIDTH$}",
            "Language".bold().blue(),
            "Files".bold().blue(),
            "Lines".bold().blue(),
            "Code".bold().blue(),
            "Comments".bold().blue(),
            "Blanks".bold().blue(),
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
            "{:>FILES_COLUMN_WIDTH$} {:>LINES_COLUMN_WIDTH$} {:>CODE_COLUMN_WIDTH$} {:>COMMENTS_COLUMN_WIDTH$} {:>BLANKS_COLUMN_WIDTH$}",
            language
                .reports
                .len()
                .to_formatted_string(&self.number_format),
            language.lines().to_formatted_string(&self.number_format),
            language.code.to_formatted_string(&self.number_format),
            language.comments.to_formatted_string(&self.number_format),
            language.blanks.to_formatted_string(&self.number_format),
        )
    }

    fn print_language_in_print_total(&mut self, language: &Language) -> io::Result<()>
    where
        W: Write,
    {
        self.print_language_name(language.inaccurate, "Total", None)?;
        write!(self.writer, " ")?;
        writeln!(
            self.writer,
            "{:>FILES_COLUMN_WIDTH$} {:>LINES_COLUMN_WIDTH$} {:>CODE_COLUMN_WIDTH$} {:>COMMENTS_COLUMN_WIDTH$} {:>BLANKS_COLUMN_WIDTH$}",
            language
                .children
                .values()
                .map(Vec::len)
                .sum::<usize>()
                .to_formatted_string(&self.number_format)
                .blue(),
            language
                .lines()
                .to_formatted_string(&self.number_format)
                .blue(),
            language
                .code
                .to_formatted_string(&self.number_format)
                .blue(),
            language
                .comments
                .to_formatted_string(&self.number_format)
                .blue(),
            language
                .blanks
                .to_formatted_string(&self.number_format)
                .blue(),
        )
    }

    pub fn print_language_name(
        &mut self,
        inaccurate: bool,
        name: &str,
        prefix: Option<&str>,
    ) -> io::Result<()> {
        let mut lang_section_len = self.columns - NO_LANG_ROW_LEN - prefix.map_or(0, str::len);
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
            write!(
                self.writer,
                " {:<len$}",
                name.bold().magenta(),
                len = lang_section_len
            )?;
        }
        if inaccurate {
            write!(self.writer, "{}", IDENT_INACCURATE)?;
        };

        Ok(())
    }

    fn print_code_stats(
        &mut self,
        language_type: LanguageType,
        stats: &[CodeStats],
    ) -> io::Result<()> {
        self.print_language_name(false, &language_type.to_string(), Some(" |-"))?;
        let mut code = 0;
        let mut comments = 0;
        let mut blanks = 0;

        for stats in stats.iter().map(tokei::CodeStats::summarise) {
            code += stats.code;
            comments += stats.comments;
            blanks += stats.blanks;
        }

        if stats.is_empty() {
            Ok(())
        } else {
            writeln!(
                self.writer,
                " {:>FILES_COLUMN_WIDTH$} {:>LINES_COLUMN_WIDTH$} {:>CODE_COLUMN_WIDTH$} {:>COMMENTS_COLUMN_WIDTH$} {:>BLANKS_COLUMN_WIDTH$}",
                stats.len().to_formatted_string(&self.number_format),
                (code + comments + blanks).to_formatted_string(&self.number_format),
                code.to_formatted_string(&self.number_format),
                comments.to_formatted_string(&self.number_format),
                blanks.to_formatted_string(&self.number_format),
            )
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
        let mut subtotal = tokei::Report::new("(Total)".into());
        let summary = parent.summarise();
        subtotal.stats.code += summary.code;
        subtotal.stats.comments += summary.comments;
        subtotal.stats.blanks += summary.blanks;
        self.print_report_with_name(&subtotal)?;

        Ok(())
    }

    pub fn print_results<'a, I>(
        &mut self,
        languages: I,
        compact: bool,
        is_sorted: bool,
    ) -> io::Result<()>
    where
        I: Iterator<Item = (&'a LanguageType, &'a Language)>,
    {
        let (a, b): (Vec<_>, Vec<_>) = languages
            .filter(|(_, v)| !v.is_empty())
            .partition(|(_, l)| compact || l.children.is_empty());
        let mut first = true;

        for languages in &[&a, &b] {
            for &(name, language) in *languages {
                let has_children = !(compact || language.children.is_empty());
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
                    let mut reports: Vec<&Report> = language.reports.iter().collect();
                    if !is_sorted {
                        reports.sort_by(|&a, &b| a.name.cmp(&b.name));
                    }
                    if compact {
                        for &report in &reports {
                            writeln!(self.writer, "{:1$}", report, self.path_length)?;
                        }
                    } else {
                        let (a, b): (Vec<&Report>, Vec<&Report>) =
                            reports.iter().partition(|&r| r.stats.blobs.is_empty());
                        for reports in &[&a, &b] {
                            let mut first = true;
                            for report in reports.iter() {
                                if report.stats.blobs.is_empty() {
                                    writeln!(self.writer, "{:1$}", report, self.path_length)?;
                                } else {
                                    if first && a.is_empty() {
                                        writeln!(self.writer, " {}", report.name.display())?;
                                        first = false;
                                    } else {
                                        writeln!(
                                            self.writer,
                                            "-- {} {}",
                                            report.name.display(),
                                            "-".repeat(
                                                self.columns
                                                    - 4
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
                                    self.print_report_total(report, language.inaccurate)?;
                                }
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
        writeln!(self.writer, "{}", self.subrow.dimmed())
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
            " {:>FILES_COLUMN_WIDTH$} {:>LINES_COLUMN_WIDTH$} {:>CODE_COLUMN_WIDTH$} {:>COMMENTS_COLUMN_WIDTH$} {:>BLANKS_COLUMN_WIDTH$}",
            " ",
            stats.lines().to_formatted_string(&self.number_format),
            stats.code.to_formatted_string(&self.number_format),
            stats.comments.to_formatted_string(&self.number_format),
            stats.blanks.to_formatted_string(&self.number_format),
        )
    }

    fn print_report_total(&mut self, report: &Report, inaccurate: bool) -> io::Result<()> {
        if report.stats.blobs.is_empty() {
            return Ok(());
        }

        let mut subtotal = tokei::Report::new("|- (Total)".into());
        subtotal.stats.code += report.stats.code;
        subtotal.stats.comments += report.stats.comments;
        subtotal.stats.blanks += report.stats.blanks;

        for (language_type, stats) in &report.stats.blobs {
            self.print_report(*language_type, stats, inaccurate)?;
            subtotal.stats += stats.summarise();
        }

        self.print_report_with_name(report)?;

        Ok(())
    }

    fn print_report_with_name(&mut self, report: &Report) -> io::Result<()> {
        let name = report.name.to_string_lossy();
        let name_length = name.len();

        if name_length > self.path_length {
            let mut formatted = String::from("|");
            // Add 1 to the index to account for the '|' we add to the output string
            let from = find_char_boundary(&name, name_length + 1 - self.path_length);
            formatted.push_str(&name[from..]);
        }
        self.print_report_total_formatted(name, self.path_length, report)?;

        Ok(())
    }

    fn print_report_total_formatted(
        &mut self,
        name: Cow<'_, str>,
        max_len: usize,
        report: &Report,
    ) -> io::Result<()> {
        let lines_column_width: usize = FILES_COLUMN_WIDTH + 6;
        writeln!(
            self.writer,
            " {: <max$} {:>lines_column_width$} {:>CODE_COLUMN_WIDTH$} {:>COMMENTS_COLUMN_WIDTH$} {:>BLANKS_COLUMN_WIDTH$}",
            name,
            report
                .stats
                .lines()
                .to_formatted_string(&self.number_format),
            report.stats.code.to_formatted_string(&self.number_format),
            report
                .stats
                .comments
                .to_formatted_string(&self.number_format),
            report.stats.blanks.to_formatted_string(&self.number_format),
            max = max_len
        )
    }

    pub fn print_total(&mut self, languages: &tokei::Languages) -> io::Result<()> {
        let total = languages.total();
        self.print_row()?;
        self.print_language_in_print_total(&total)?;
        self.print_row()
    }
}
