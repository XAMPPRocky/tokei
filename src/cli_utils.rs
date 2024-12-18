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

#[derive(Clone, Copy)]
pub enum Column {
    Language,
    Files,
    Lines,
    Code,
    Comments,
    Blanks,
}

impl Column {
    fn print_header(self, mut writer: impl Write) -> io::Result<usize> {
        let mut len = 0;
        match self {
            Column::Language => {
                let s = "Language";
                len += s.len();
                write!(writer, "{}", s.bold().blue())?;
            }
            Column::Files => {
                let s = "Files";
                len += s.len().max(FILES_COLUMN_WIDTH);
                write!(writer, "{:>FILES_COLUMN_WIDTH$}", s.bold().blue())?;
            }
            Column::Lines => {
                let s = "Lines";
                len += s.len().max(LINES_COLUMN_WIDTH);
                write!(writer, "{:>LINES_COLUMN_WIDTH$}", s.bold().blue())?;
            }
            Column::Code => {
                let s = "Code";
                len += s.len().max(CODE_COLUMN_WIDTH);
                write!(writer, "{:>CODE_COLUMN_WIDTH$}", s.bold().blue())?;
            }
            Column::Comments => {
                let s = "Comments";
                len += s.len().max(COMMENTS_COLUMN_WIDTH);
                write!(writer, "{:>COMMENTS_COLUMN_WIDTH$}", s.bold().blue(),)?;
            }
            Column::Blanks => {
                let s = "Blanks";
                len += s.len().max(BLANKS_COLUMN_WIDTH);
                write!(writer, "{:>BLANKS_COLUMN_WIDTH$}", s.bold().blue())?;
            }
        };
        Ok(len)
    }

    fn print_language(
        self,
        mut writer: impl Write,
        language: &Language,
        number_format: &num_format::CustomFormat,
    ) -> io::Result<usize> {
        let mut len = 0;
        match self {
            Column::Language => unreachable!(),
            Column::Files => {
                let s = language.reports.len().to_formatted_string(number_format);
                len += s.len().max(FILES_COLUMN_WIDTH);
                write!(writer, "{:>FILES_COLUMN_WIDTH$}", s)?;
            }
            Column::Lines => {
                let s = language.lines().to_formatted_string(number_format);
                len += s.len().max(LINES_COLUMN_WIDTH);
                write!(writer, "{:>LINES_COLUMN_WIDTH$}", s)?;
            }
            Column::Code => {
                let s = language.code.to_formatted_string(number_format);
                len += s.len().max(CODE_COLUMN_WIDTH);
                write!(writer, "{:>CODE_COLUMN_WIDTH$}", s)?;
            }
            Column::Comments => {
                let s = language.comments.to_formatted_string(number_format);
                len += s.len().max(COMMENTS_COLUMN_WIDTH);
                write!(writer, "{:>COMMENTS_COLUMN_WIDTH$}", s)?;
            }
            Column::Blanks => {
                let s = language.blanks.to_formatted_string(number_format);
                len += s.len().max(BLANKS_COLUMN_WIDTH);
                write!(writer, "{:>BLANKS_COLUMN_WIDTH$}", s)?;
            }
        }

        Ok(len)
    }

    fn print_language_in_print_total(
        self,
        mut writer: impl Write,
        language: &Language,
        number_format: &num_format::CustomFormat,
    ) -> io::Result<usize> {
        let mut len = 0;
        match self {
            Column::Language => unreachable!(),
            Column::Files => {
                let s = language
                    .children
                    .values()
                    .map(Vec::len)
                    .sum::<usize>()
                    .to_formatted_string(number_format);
                len += s.len().max(FILES_COLUMN_WIDTH);
                write!(writer, "{:>FILES_COLUMN_WIDTH$}", s.blue())?;
            }
            Column::Lines => {
                let s = language.lines().to_formatted_string(number_format);
                len += s.len().max(LINES_COLUMN_WIDTH);
                write!(writer, "{:>LINES_COLUMN_WIDTH$}", s.blue())?;
            }
            Column::Code => {
                let s = language.code.to_formatted_string(number_format);
                len += s.len().max(CODE_COLUMN_WIDTH);
                write!(writer, "{:>CODE_COLUMN_WIDTH$}", s.blue())?;
            }
            Column::Comments => {
                let s = language.comments.to_formatted_string(number_format);
                len += s.len().max(COMMENTS_COLUMN_WIDTH);
                write!(writer, "{:>COMMENTS_COLUMN_WIDTH$}", s.blue())?;
            }
            Column::Blanks => {
                let s = language.blanks.to_formatted_string(number_format);
                len += s.len().max(BLANKS_COLUMN_WIDTH);
                write!(writer, "{:>BLANKS_COLUMN_WIDTH$}", s.blue())?;
            }
        }

        Ok(len)
    }

    fn print_code_stats(
        self,
        mut writer: impl Write,
        stats: &[CodeStats],
        number_format: &num_format::CustomFormat,
        code: usize,
        comments: usize,
        blanks: usize,
    ) -> io::Result<usize> {
        let mut len = 0;
        match self {
            Column::Language => unreachable!(),
            Column::Files => {
                let s = stats.len().to_formatted_string(number_format);
                len += s.len().max(FILES_COLUMN_WIDTH);
                write!(writer, "{:>FILES_COLUMN_WIDTH$}", s)?;
            }
            Column::Lines => {
                let s = (code + comments + blanks).to_formatted_string(number_format);
                len += s.len().max(LINES_COLUMN_WIDTH);
                write!(writer, "{:>LINES_COLUMN_WIDTH$}", s)?;
            }
            Column::Code => {
                let s = code.to_formatted_string(number_format);
                len += s.len().max(CODE_COLUMN_WIDTH);
                write!(writer, "{:>CODE_COLUMN_WIDTH$}", s)?;
            }
            Column::Comments => {
                let s = comments.to_formatted_string(number_format);
                len += s.len().max(COMMENTS_COLUMN_WIDTH);
                write!(writer, "{:>COMMENTS_COLUMN_WIDTH$}", s)?;
            }
            Column::Blanks => {
                let s = blanks.to_formatted_string(number_format);
                len += s.len().max(BLANKS_COLUMN_WIDTH);
                write!(writer, "{:>BLANKS_COLUMN_WIDTH$}", s)?;
            }
        }

        Ok(len)
    }

    fn print_report_total_formatted(
        self,
        mut writer: impl Write,
        name: &str,
        report: &Report,
        max_len: usize,
        number_format: &num_format::CustomFormat,
    ) -> io::Result<usize> {
        let mut len = 0;
        match self {
            Column::Language => {
                let s = name;
                len += s.len().max(max_len);
                write!(writer, "{:<max_len$}", s)?;
            }
            Column::Files => {
                let s = "";
                len += s.len().max(FILES_COLUMN_WIDTH);
                write!(writer, "{:>FILES_COLUMN_WIDTH$}", s)?;
            }
            Column::Lines => {
                let s = report.stats.lines().to_formatted_string(number_format);
                len += s.len().max(LINES_COLUMN_WIDTH);
                write!(writer, "{:>LINES_COLUMN_WIDTH$}", s)?;
            }
            Column::Code => {
                let s = report.stats.code.to_formatted_string(number_format);
                len += s.len().max(CODE_COLUMN_WIDTH);
                write!(writer, "{:>CODE_COLUMN_WIDTH$}", s)?;
            }
            Column::Comments => {
                let s = report.stats.comments.to_formatted_string(number_format);
                len += s.len().max(COMMENTS_COLUMN_WIDTH);
                write!(writer, "{:>COMMENTS_COLUMN_WIDTH$}", s)?;
            }
            Column::Blanks => {
                let s = report.stats.blanks.to_formatted_string(number_format);
                len += s.len().max(BLANKS_COLUMN_WIDTH);
                write!(writer, "{:>BLANKS_COLUMN_WIDTH$}", s)?;
            }
        }

        Ok(len)
    }

    fn print_report(
        self,
        mut writer: impl Write,
        stats: &CodeStats,
        number_format: &num_format::CustomFormat,
    ) -> io::Result<usize> {
        let mut len = 0;
        match self {
            Column::Language => unreachable!(),
            Column::Files => {
                let s = "";
                len += s.len().max(FILES_COLUMN_WIDTH);
                write!(writer, "{:>FILES_COLUMN_WIDTH$}", s)?;
            }
            Column::Lines => {
                let s = stats.lines().to_formatted_string(number_format);
                len += s.len().max(LINES_COLUMN_WIDTH);
                write!(writer, "{:>LINES_COLUMN_WIDTH$}", s)?;
            }
            Column::Code => {
                let s = stats.code.to_formatted_string(number_format);
                len += s.len().max(CODE_COLUMN_WIDTH);
                write!(writer, "{:>CODE_COLUMN_WIDTH$}", s)?;
            }
            Column::Comments => {
                let s = stats.comments.to_formatted_string(number_format);
                len += s.len().max(COMMENTS_COLUMN_WIDTH);
                write!(writer, "{:>COMMENTS_COLUMN_WIDTH$}", s)?;
            }
            Column::Blanks => {
                let s = stats.blanks.to_formatted_string(number_format);
                len += s.len().max(BLANKS_COLUMN_WIDTH);
                write!(writer, "{:>BLANKS_COLUMN_WIDTH$}", s)?;
            }
        }

        Ok(len)
    }
}

pub struct RowPrinter<'a> {
    length: usize,
    output_columns: &'a [Column],
    row_start: Vec<u8>,
    row_end: Vec<u8>,
    printed_left: usize,
    printed_right: usize,
}

impl<'a> RowPrinter<'a> {
    pub fn new(columns: usize, output_columns: &'a [Column]) -> Self {
        Self {
            length: columns,
            output_columns,
            row_start: Vec::new(),
            row_end: Vec::new(),
            printed_left: 0,
            printed_right: 0,
        }
    }

    pub fn print_header(mut self, writer: impl Write) -> io::Result<()> {
        self.row_start.push(b' ');
        self.printed_left = 1 + Column::Language.print_header(&mut self.row_start)?;
        for col in self.output_columns {
            self.row_end.push(b' ');
            self.printed_right += 1 + col.print_header(&mut self.row_end)?;
        }

        self.write(writer)?;
        Ok(())
    }

    pub fn print_language_name(
        &mut self,
        inaccurate: bool,
        name: &str,
        prefix: Option<&str>,
    ) -> io::Result<()> {
        let mut lang_section_len = self.length - NO_LANG_ROW_LEN - prefix.map_or(0, str::len);
        self.printed_left = lang_section_len + 1; // +1 for initial ' '
        if inaccurate {
            lang_section_len -= IDENT_INACCURATE.len();
        }

        if let Some(prefix) = prefix {
            write!(&mut self.row_start, "{}", prefix)?;
            self.printed_left += prefix.len();
        }
        // truncate and replace the last char with a `|` if the name is too long
        if lang_section_len < name.len() {
            write!(
                &mut self.row_start,
                " {:.len$}",
                name,
                len = lang_section_len - 1
            )?;
            write!(&mut self.row_start, "|")?;
        } else {
            write!(
                &mut self.row_start,
                " {:<len$}",
                name.bold().magenta(),
                len = lang_section_len
            )?;
        }
        if inaccurate {
            write!(&mut self.row_start, "{}", IDENT_INACCURATE)?;
        }

        Ok(())
    }

    pub fn print_language(
        mut self,
        writer: impl Write,
        language: &Language,
        number_format: &num_format::CustomFormat,
    ) -> io::Result<()> {
        for col in self.output_columns {
            self.row_end.push(b' ');
            self.printed_right +=
                1 + col.print_language(&mut self.row_end, language, number_format)?;
        }

        self.write(writer)?;
        Ok(())
    }

    pub fn print_language_in_print_total(
        mut self,
        writer: impl Write,
        language: &Language,
        number_format: &num_format::CustomFormat,
    ) -> io::Result<()> {
        for col in self.output_columns {
            self.row_end.push(b' ');
            self.printed_right += 1 + col.print_language_in_print_total(
                &mut self.row_end,
                language,
                number_format,
            )?;
        }

        self.write(writer)?;
        Ok(())
    }

    pub fn print_code_stats(
        mut self,
        writer: impl Write,
        stats: &[CodeStats],
        number_format: &num_format::CustomFormat,
        code: usize,
        comments: usize,
        blanks: usize,
    ) -> io::Result<()> {
        for col in self.output_columns {
            self.row_end.push(b' ');
            self.printed_right += 1 + col.print_code_stats(
                &mut self.row_end,
                stats,
                number_format,
                code,
                comments,
                blanks,
            )?;
        }

        self.write(writer)?;
        Ok(())
    }

    pub fn print_report_total_formatted(
        mut self,
        writer: impl Write,
        name: Cow<'_, str>,
        report: &Report,
        max_len: usize,
        number_format: &num_format::CustomFormat,
    ) -> io::Result<()> {
        for col in self.output_columns {
            self.row_end.push(b' ');
            self.printed_right += 1 + col.print_report_total_formatted(
                &mut self.row_end,
                &name,
                report,
                max_len,
                number_format,
            )?;
        }

        self.row_start.push(b' ');
        self.printed_left = 1 + Column::Language.print_report_total_formatted(
            &mut self.row_start,
            &name,
            report,
            max_len.min(self.length - self.printed_right - 2),
            number_format,
        )?;

        self.write(writer)?;
        Ok(())
    }

    pub fn print_report(
        mut self,
        writer: impl Write,
        stats: &CodeStats,
        number_format: &num_format::CustomFormat,
    ) -> io::Result<()> {
        for col in self.output_columns {
            self.row_end.push(b' ');
            self.printed_right += 1 + col.print_report(&mut self.row_end, stats, number_format)?;
        }

        self.write(writer)?;
        Ok(())
    }

    fn write(self, mut writer: impl Write) -> io::Result<()> {
        let length = self
            .length
            .saturating_sub(self.printed_left + self.printed_right + 1);
        writeln!(
            writer,
            "{}{}{:>}",
            String::from_utf8_lossy(&self.row_start),
            " ".repeat(length),
            String::from_utf8_lossy(&self.row_end),
        )?;
        Ok(())
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
    output_columns: Vec<Column>,
}

impl<W> Printer<W> {
    pub fn new(
        columns: usize,
        list_files: bool,
        writer: W,
        number_format: num_format::CustomFormat,
        output_columns: Vec<Column>,
    ) -> Self {
        Self {
            columns,
            list_files,
            path_length: columns - NO_LANG_ROW_LEN_NO_SPACES,
            writer,
            row: "━".repeat(columns),
            subrow: "─".repeat(columns),
            number_format,
            output_columns,
        }
    }
}

impl<W: Write> Printer<W> {
    pub fn print_header(&mut self) -> io::Result<()> {
        self.print_row()?;

        let printer = RowPrinter::new(self.columns, &self.output_columns);
        printer.print_header(&mut self.writer)?;

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
        let mut printer = RowPrinter::new(self.columns, &self.output_columns);
        printer.print_language_name(language.inaccurate, name, None)?;
        printer.print_language(&mut self.writer, language, &self.number_format)?;

        Ok(())
    }

    fn print_language_in_print_total(&mut self, language: &Language) -> io::Result<()>
    where
        W: Write,
    {
        let mut printer = RowPrinter::new(self.columns, &self.output_columns);
        printer.print_language_name(language.inaccurate, "Total", None)?;
        printer.print_language_in_print_total(&mut self.writer, language, &self.number_format)?;

        Ok(())
    }

    fn print_code_stats(
        &mut self,
        language_type: LanguageType,
        stats: &[CodeStats],
    ) -> io::Result<()> {
        let mut code = 0;
        let mut comments = 0;
        let mut blanks = 0;

        for stats in stats.iter().map(tokei::CodeStats::summarise) {
            code += stats.code;
            comments += stats.comments;
            blanks += stats.blanks;
        }

        let mut printer = RowPrinter::new(self.columns, &self.output_columns);
        printer.print_language_name(false, &language_type.to_string(), Some(" |-"))?;
        if stats.is_empty() {
            Ok(())
        } else {
            printer.print_code_stats(
                &mut self.writer,
                stats,
                &self.number_format,
                code,
                comments,
                blanks,
            )?;
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
                                    write!(self.writer, " {}", &report.name.to_string_lossy())?;
                                    let printer = RowPrinter::new(
                                        self.columns - report.name.as_os_str().len() - 1,
                                        &self.output_columns,
                                    );
                                    printer.print_report(
                                        &mut self.writer,
                                        &report.stats,
                                        &self.number_format,
                                    )?;
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

                                    let mut printer =
                                        RowPrinter::new(self.columns, &self.output_columns);
                                    printer.print_language_name(
                                        false,
                                        &name.to_string(),
                                        Some(" |-"),
                                    )?;
                                    printer.print_report(
                                        &mut self.writer,
                                        &report.stats,
                                        &self.number_format,
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
        let mut printer = RowPrinter::new(self.columns, &self.output_columns);
        printer.print_language_name(inaccurate, &language_type.to_string(), Some(" |-"))?;
        printer.print_report(&mut self.writer, stats, &self.number_format)?;
        Ok(())
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

        self.print_report_total_formatted(name, self.path_length, report)?;

        Ok(())
    }

    fn print_report_total_formatted(
        &mut self,
        name: Cow<'_, str>,
        max_len: usize,
        report: &Report,
    ) -> io::Result<()> {
        let printer = RowPrinter::new(self.columns, &self.output_columns);
        printer.print_report_total_formatted(
            &mut self.writer,
            name,
            report,
            max_len,
            &self.number_format,
        )?;
        Ok(())
    }

    pub fn print_total(&mut self, languages: &tokei::Languages) -> io::Result<()> {
        let total = languages.total();
        self.print_row()?;
        self.print_language_in_print_total(&total)?;
        self.print_row()
    }
}
