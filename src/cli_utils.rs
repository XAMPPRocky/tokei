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
        // if there's no unclassified files left for this language (that is, if
        // all reports for this language has moved to classifications) there's
        // no line counts to report here, just zeroes, lets do nothing instead
        if language.reports.len() == 0 {
            writeln!(self.writer)?;
            return Ok(());
        }
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
        language_name: &str,
        stats: &[CodeStats],
        is_classification: bool,
    ) -> io::Result<()> {
        let prefix = if is_classification {
            Some(" |>")
        } else {
            Some(" |-")
        };

        self.print_language_name(false, language_name, prefix)?;
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

    fn print_language_total(&mut self, parent: &Language, compact: bool) -> io::Result<()> {
        // Print embedded languages (children) - skip in compact mode
        if !compact {
            for (language_name, reports) in &parent.children {
                self.print_code_stats(
                    language_name,
                    &reports
                        .iter()
                        .map(|r| r.stats.summarise())
                        .collect::<Vec<_>>(),
                    false,
                )?;
            }
        }

        // Print classifications (e.g., Tests, Generated) - always show
        for (classification_name, reports) in &parent.classifications {
            self.print_code_stats(
                classification_name,
                &reports
                    .iter()
                    .map(|r| r.stats.summarise())
                    .collect::<Vec<_>>(),
                true,
            )?;
        }

        // Print subtotal. We only show file count when there are
        // classifications. This maintains the existing behavior where no totals
        // are shown for embedded languages (as they do not represent separate
        // files). However, when also showing classified files we should show
        // unclassified + classified file count in the total.
        let mut subtotal = tokei::Report::new("(Total)".into());
        let summary = parent.summarise();
        subtotal.stats.code += summary.code;
        subtotal.stats.comments += summary.comments;
        subtotal.stats.blanks += summary.blanks;

        let file_count = if !parent.classifications.is_empty() {
            Some(parent.file_count())
        } else {
            None
        };
        self.print_report_with_name(&subtotal, file_count)?;

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
            .partition(|(_, l)| compact || (l.children.is_empty() && l.classifications.is_empty()));
        let mut first = true;

        for languages in &[&a, &b] {
            for &(name, language) in *languages {
                // In compact mode: skip children but show classifications
                let has_children_or_classifications = if compact {
                    !language.classifications.is_empty()
                } else {
                    !(language.children.is_empty() && language.classifications.is_empty())
                };

                if first {
                    first = false;
                } else if has_children_or_classifications || self.list_files {
                    self.print_subrow()?;
                }

                self.print_language(language, name.name())?;
                if has_children_or_classifications {
                    self.print_language_total(language, compact)?;
                }

                if self.list_files {
                    self.print_subrow()?;
                    // unclassified files are in language.reports. If
                    // classifications are in use, matching files have been
                    // moved to classifications
                    let mut reports: Vec<&Report> = language
                        .reports
                        .iter()
                        .chain(language.classifications.values().flat_map(|v| v.iter()))
                        .collect();

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

        self.print_report_with_name(report, None)?;

        Ok(())
    }

    fn print_report_with_name(
        &mut self,
        report: &Report,
        file_count: Option<usize>,
    ) -> io::Result<()> {
        let name = report.name.to_string_lossy();
        let name_length = name.len();

        let formatted_name = if name_length > self.path_length {
            let mut formatted = String::from("|");
            // Add 1 to the index to account for the '|' we add to the output string
            let from = find_char_boundary(&name, name_length + 1 - self.path_length);
            formatted.push_str(&name[from..]);
            formatted.into()
        } else {
            name
        };

        self.print_report_total_formatted(formatted_name, self.path_length, report, file_count)?;

        Ok(())
    }

    fn print_report_total_formatted(
        &mut self,
        name: Cow<'_, str>,
        max_len: usize,
        report: &Report,
        file_count: Option<usize>,
    ) -> io::Result<()> {
        // Calculate column widths based on whether we're showing file count
        let (name_width, lines_column_width) = if file_count.is_some() {
            // With file count: reduce name width, use standard lines width
            (max_len - FILES_COLUMN_WIDTH + 1, LINES_COLUMN_WIDTH)
        } else {
            // Without file count: keep name width, extend lines width
            (max_len, FILES_COLUMN_WIDTH + 6)
        };
        write!(self.writer, " {: <width$} ", name, width = name_width)?;

        if let Some(count) = file_count {
            let count_str = count.to_formatted_string(&self.number_format);
            write!(self.writer, "{:>FILES_COLUMN_WIDTH$} ", count_str)?;
        }

        writeln!(
            self.writer,
            "{:>lines_column_width$} {:>CODE_COLUMN_WIDTH$} {:>COMMENTS_COLUMN_WIDTH$} {:>BLANKS_COLUMN_WIDTH$}",
            report.stats.lines().to_formatted_string(&self.number_format),
            report.stats.code.to_formatted_string(&self.number_format),
            report.stats.comments.to_formatted_string(&self.number_format),
            report.stats.blanks.to_formatted_string(&self.number_format),
            lines_column_width = lines_column_width
        )
    }

    pub fn print_total(&mut self, languages: &tokei::Languages) -> io::Result<()> {
        let total = languages.total();
        self.print_row()?;
        self.print_language_in_print_total(&total)?;
        self.print_row()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::path::PathBuf;
    use tokei::{Language, LanguageType};

    // Test helper functions

    fn default_number_format() -> num_format::CustomFormat {
        num_format::CustomFormat::builder()
            .grouping(num_format::Grouping::Standard)
            .separator("")
            .build()
            .unwrap()
    }

    fn create_report(path: &str, code: usize, classification: Option<&str>) -> Report {
        let mut report = Report::new(PathBuf::from(path));
        report.stats.code = code;
        report.classification = classification.map(|s| s.to_string());
        report
    }

    fn create_mixed_language() -> Language {
        let mut lang = Language::new();
        lang.add_report(create_report("src/main.js", 100, None));
        lang.add_report(create_report("src/utils.js", 50, None));
        lang.add_report(create_report("src/main.test.js", 30, Some("Tests")));
        lang.add_report(create_report("src/utils.test.js", 20, Some("Tests")));
        lang.total_with_classifications(true);
        lang
    }

    fn print_language_results(lang: Language, list_files: bool, compact: bool) -> String {
        colored::control::set_override(false);
        let mut output = Vec::new();
        let mut printer = Printer::new(80, list_files, &mut output, default_number_format());

        let mut languages_map = std::collections::BTreeMap::new();
        languages_map.insert(LanguageType::JavaScript, lang);
        printer
            .print_results(languages_map.iter().map(|(k, v)| (k, v)), compact, false)
            .unwrap();

        String::from_utf8(output).unwrap()
    }

    #[test]
    fn test_list_files_includes_classified_files() {
        let lang = create_mixed_language();
        let output_str = print_language_results(lang, true, false);

        // Classified files should show their classification in the Files column
        // Format: filename, then spaces, then classification (e.g., "Tests"), then spaces, then lines number
        let test_files_lines: Vec<&str> = output_str
            .lines()
            .filter(|line| line.contains("test.js"))
            .collect();

        // Check that both test files have "Tests" in their output
        assert_eq!(test_files_lines.len(), 2, "Should have 2 test files");
        for line in &test_files_lines {
            assert!(
                line.contains("Tests"),
                "Test file should have 'Tests' classification: {}",
                line
            );
        }

        // Unclassified files should have no classification text, just filename then spaces then numbers
        let files_lines: Vec<&str> = output_str
            .lines()
            .filter(|line| {
                (line.contains("main.js") || line.contains("utils.js")) && !line.contains("test")
            })
            .collect();

        assert_eq!(files_lines.len(), 2, "Should have 2 unclassified files");
        for line in &files_lines {
            assert!(
                !line.contains("Tests"),
                "Unclassified file should not have 'Tests': {}",
                line
            );
        }
    }

    fn get_subtotal_line(lang: &Language) -> String {
        print_language_results(lang.clone(), true, false)
            .lines()
            .find(|line| line.contains("(Total)"))
            .expect("Should have (Total) line")
            .to_string()
    }

    #[test]
    fn test_subtotal_has_empty_files_column_without_classifications() {
        let mut lang = Language::new();
        // add a report with an embedded language blob
        let mut main_js = create_report("src/main.js", 200, None);
        main_js
            .stats
            .blobs
            .insert(LanguageType::Markdown, CodeStats::new());
        lang.add_report(main_js);
        lang.total();
        let total_line = get_subtotal_line(&lang);

        // Without classifications, File count column (used to hold the class name) should be empty
        let re = Regex::new(r"\(Total\)\s+200").unwrap();
        assert!(
            re.is_match(&total_line),
            "Subtotal should match (Total) followed by whitespace and 200: {}",
            total_line
        );
    }

    #[test]
    fn test_subtotal_shows_file_count_with_classifications() {
        let lang = create_mixed_language();

        let total_line = get_subtotal_line(&lang);

        // With classifications, subtotal SHOULD show file count (4 total)
        assert!(
            total_line.contains("4"),
            "Subtotal should show total file count with classifications: {}",
            total_line
        );
    }

    #[test]
    fn test_compact_mode_shows_classifications() {
        let mut lang = Language::new();
        lang.add_report(create_report("src/main.js", 100, None));
        lang.add_report(create_report("src/main.test.js", 50, Some("Tests")));
        lang.total_with_classifications(true);

        let output_str = print_language_results(lang.clone(), false, true);

        // In compact mode, classifications should still be shown
        assert!(
            output_str.contains("Tests"),
            "Compact mode should show classifications: {}",
            output_str
        );
    }
}
