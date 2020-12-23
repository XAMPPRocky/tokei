use std::{
    fmt, 
    io::{self, Write},
    path::PathBuf,
    process,
    str::FromStr,
};

use clap::crate_version;
use num_format::ToFormattedString;

use crate::input::Format;
use tokei::{find_char_boundary, CodeStats, Language, LanguageType, Report};

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

    pub fn all() -> &'static [&'static str] {
        &["commas", "dots", "plain", "underscores"]
    }

    pub fn get_format(self) -> Result<num_format::CustomFormat, num_format::Error> {
        num_format::CustomFormat::builder()
            .grouping(num_format::Grouping::Standard)
            .separator(self.separator())
            .build()
    }
}

pub struct Printer {
    columns: usize,
    path_length: usize,
    row: String,
    subrow: String,
    list_files: bool,
    command: std::path::PathBuf,
    folder: std::path::PathBuf,
    number_format: num_format::CustomFormat,
}

impl Printer {
    pub fn new(
        columns: usize,
        list_files: bool,
        command: std::path::PathBuf,
        folder: std::path::PathBuf,
        number_format: num_format::CustomFormat,
    ) -> Self {
        Self {
            columns,
            list_files,
            command,
            folder,
            path_length: columns - NO_LANG_ROW_LEN_NO_SPACES,
            row: "=".repeat(columns),
            subrow: "-".repeat(columns),
            number_format,
        }
    }
}

impl Printer {
    pub fn print_header(&self, writer: &mut io::BufWriter<io::Stdout>) -> io::Result<()> {
        self.print_row(writer)?;
        writeln!(
            writer,
            " {:<6$} {:>12} {:>12} {:>12} {:>12} {:>12}",
            "Language",
            "Files",
            "Lines",
            "Code",
            "Comments",
            "Blanks",
            self.columns - NO_LANG_HEADER_ROW_LEN
        )?;
        self.print_row(writer)
    }

    pub fn print_inaccuracy_warning(&self, writer: &mut io::BufWriter<io::Stdout>) -> io::Result<()> {
        writeln!(
            writer,
            "Note: results can be inaccurate for languages marked with '{}'",
            IDENT_INACCURATE
        )
    }

    pub fn print_language(&self, writer: &mut io::BufWriter<io::Stdout>, language: &Language, name: &str) -> io::Result<()>
    {
        self.print_language_name(writer, language.inaccurate, name, None)?;
        write!(writer, " ")?;
        writeln!(
            writer,
            "{:>6} {:>12} {:>12} {:>12} {:>12}",
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

    pub fn print_language_name(
        &self, writer: &mut io::BufWriter<io::Stdout>,
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
            write!(writer, "{}", prefix)?;
        }
        // truncate and replace the last char with a `|` if the name is too long
        if lang_section_len < name.len() {
            write!(writer, " {:.len$}", name, len = lang_section_len - 1)?;
            write!(writer, "|")?;
        } else {
            write!(writer, " {:<len$}", name, len = lang_section_len)?;
        }
        if inaccurate {
            write!(writer, "{}", IDENT_INACCURATE)?;
        };

        Ok(())
    }

    fn print_code_stats<'a, 'b>(
        &self, writer: &mut io::BufWriter<io::Stdout>,
        language_type: LanguageType,
        stats: &[CodeStats],
    ) -> io::Result<()> {
        self.print_language_name(writer, false, &language_type.to_string(), Some(&(" |-")))?;
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
                writer,
                " {:>6} {:>12} {:>12} {:>12} {:>12}",
                stats.len().to_formatted_string(&self.number_format),
                (code + comments + blanks).to_formatted_string(&self.number_format),
                code.to_formatted_string(&self.number_format),
                comments.to_formatted_string(&self.number_format),
                blanks.to_formatted_string(&self.number_format),
            )
        } else {
            Ok(())
        }
    }

    fn print_language_total(&self, writer: &mut io::BufWriter<io::Stdout>, parent: &Language) -> io::Result<()> {
        for (language, reports) in &parent.children {
            self.print_code_stats(writer,
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
        self.print_report_with_name(writer, &subtotal)?;

        Ok(())
    }

    fn process_file<'a>(&'a self, name: &std::path::PathBuf, lang: &str) {
       let filename = PathBuf::from(name);
       let command = PathBuf::from(&self.command);
       let folder = PathBuf::from(&self.folder);
       if command.to_str().expect("has command").len() > 0 { // there exists a command
           let _ = std::process::Command::new("/bin/sh")
            .arg(&command)
            .arg(&filename)
            .arg(&lang)
            .arg(&folder)
            .output()
            .expect(format!("{:?} failed to start command {:?} for {} at folder {:?}", &filename, &command, &lang, &folder).as_str());
        }
    }

    pub fn print_results<'a, I>(&self, writer: &mut io::BufWriter<io::Stdout>, languages: I) -> io::Result<()>
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
                    self.print_subrow(writer)?;
                }

                self.print_language(writer, language, name.name())?;
                if has_children {
                    self.print_language_total(writer, language)?;
                }

                if self.list_files {
                    self.print_subrow(writer)?;
                    let (a, b): (Vec<_>, Vec<_>) = language
                        .reports
                        .iter()
                        .partition(|r| r.stats.blobs.is_empty());
                    for reports in &[&a, &b] {
                        let mut first = true;
                        for report in reports.iter() {
                            if !report.stats.blobs.is_empty() {
                                if first && a.is_empty() {
                                    writeln!(writer, " {}", report.name.display())?;
                                    first = false;
                                } else {
                                    writeln!(
                                        writer,
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
                                    writer,
                                    " |-{:1$}",
                                    new_report,
                                    self.path_length - 3
                                )?;
                                self.print_report_total(writer, &report, language.inaccurate)?;
                            } else {
                                writeln!(writer, "{:1$}", report, self.path_length)?;
                            }
                        }
                    }
                }
            }
        }
        // parallelise the processes for handling individual files
        crossbeam_utils::thread::scope(|scope| {
            for languages in &[&a, &b] {
                for &(name, language) in *languages {
                    if self.list_files {
                        let (a, b): (Vec<_>, Vec<_>) = language
                            .reports
                            .iter()
                            .partition(|r| r.stats.blobs.is_empty());
                        scope.spawn(move |_| {
                            for reports in &[&a, &b] {
                                for report in reports.iter() {
                                    self.process_file(&report.name, name.name());
                                }
                            }
                        });
                    }
                }
            }
	    }).unwrap();
        Ok(())
    }

    fn print_row(&self, writer: &mut io::BufWriter<io::Stdout>) -> io::Result<()> {
        writeln!(writer, "{}", self.row)
    }

    fn print_subrow(&self, writer: &mut io::BufWriter<io::Stdout>) -> io::Result<()> {
        writeln!(writer, "{}", self.subrow)
    }

    fn print_report(
        &self, writer: &mut io::BufWriter<io::Stdout>,
        language_type: LanguageType,
        stats: &CodeStats,
        inaccurate: bool,
    ) -> io::Result<()> {
        self.print_language_name(writer, inaccurate, &language_type.to_string(), Some(" |-"))?;

        writeln!(
            writer,
            " {:>6} {:>12} {:>12} {:>12} {:>12}",
            " ",
            stats.lines().to_formatted_string(&self.number_format),
            stats.code.to_formatted_string(&self.number_format),
            stats.comments.to_formatted_string(&self.number_format),
            stats.blanks.to_formatted_string(&self.number_format),
        )
    }

    fn print_report_total(&self, writer: &mut io::BufWriter<io::Stdout>, report: &Report, inaccurate: bool) -> io::Result<()> {
        if report.stats.blobs.is_empty() {
            return Ok(());
        }

        let mut subtotal = tokei::Report::new(format!("|- (Total)").into());
        subtotal.stats.code += report.stats.code;
        subtotal.stats.comments += report.stats.comments;
        subtotal.stats.blanks += report.stats.blanks;

        for (language_type, stats) in &report.stats.blobs {
            self.print_report(writer, *language_type, stats, inaccurate)?;
            subtotal.stats += stats.summarise();
        }

        self.print_report_with_name(writer, &report)?;

        Ok(())
    }

    fn print_report_with_name(&self, writer: &mut io::BufWriter<io::Stdout>, report: &Report) -> io::Result<()> {
        let name = report.name.to_string_lossy();
        let name_length = name.len();

        if name_length <= self.path_length {
            self.print_report_total_formatted(writer, name, self.path_length, report)?;
        } else {
            let mut formatted = String::from("|");
            // Add 1 to the index to account for the '|' we add to the output string
            let from = find_char_boundary(&name, name_length + 1 - self.path_length);
            formatted.push_str(&name[from..]);
            self.print_report_total_formatted(writer, name, self.path_length, report)?;
        }

        Ok(())
    }

    fn print_report_total_formatted(
        &self, writer: &mut io::BufWriter<io::Stdout>,
        name: std::borrow::Cow<'_, str>,
        max_len: usize,
        report: &Report,
    ) -> io::Result<()> {
        writeln!(
            writer,
            " {: <max$} {:>12} {:>12} {:>12} {:>12}",
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

    pub fn print_total(&self, writer: &mut io::BufWriter<io::Stdout>, languages: tokei::Languages) -> io::Result<()> {
        let total = languages.total();
        self.print_row(writer)?;
        self.print_language(writer, &total, "Total")?;
        self.print_row(writer)
    }
}
