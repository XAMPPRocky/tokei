mod embedding;
pub mod language_type;
pub mod languages;
mod syntax;

use std::{collections::BTreeMap, collections::HashMap, mem, ops::AddAssign, path::Path};

pub use self::{language_type::*, languages::Languages};

use crate::{sort::Sort, stats::Report};

/// A struct representing statistics about a single Language.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, Serialize)]
pub struct Language {
    /// The total number of blank lines.
    pub blanks: usize,
    /// The total number of lines of code.
    pub code: usize,
    /// The total number of comments(both single, and multi-line)
    pub comments: usize,
    /// A collection of statistics of individual files.
    pub reports: Vec<Report>,
    /// A map of any languages found in the reports.
    pub children: BTreeMap<LanguageType, Vec<Report>>,
    /// Whether this language had problems with file parsing
    pub inaccurate: bool,
}

impl Language {
    /// Constructs a new empty Language with the comments provided.
    ///
    /// ```
    /// # use tokei::*;
    /// let mut rust = Language::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the total number of lines.
    #[inline]
    #[must_use]
    pub fn lines(&self) -> usize {
        self.blanks + self.code + self.comments
    }

    /// Add a `Report` to the Language. This will not update the totals in the
    /// Language struct.
    pub fn add_report(&mut self, report: Report) {
        for (lang, stats) in &report.stats.blobs {
            let mut new_report = Report::new(report.name.clone());
            new_report.stats = stats.clone();

            self.children.entry(*lang).or_default().push(new_report);
        }

        self.reports.push(report);
    }

    /// Marks this language as possibly not reflecting correct stats.
    #[inline]
    pub fn mark_inaccurate(&mut self) {
        self.inaccurate = true;
    }

    /// Creates a new `Language` from `self`, which is a summarised version
    /// of the language that doesn't contain any children. It will count
    /// non-blank lines in child languages as code unless the child language is
    /// considered "literate" then it will be counted as comments.
    #[must_use]
    pub fn summarise(&self) -> Language {
        let mut summary = self.clone();

        for reports in self.children.values() {
            for stats in reports.iter().map(|r| r.stats.summarise()) {
                summary.comments += stats.comments;
                summary.code += stats.code;
                summary.blanks += stats.blanks;
            }
        }

        summary
    }

    /// Generates stats for directories and their child directories.
    ///
    /// ```no_run
    /// use tokei::Language;
    ///
    /// let mut language = Language::new();
    ///
    /// // Add stats, assuming 10 directories are on the path afterward...
    ///
    /// // Compute totals.
    /// language.total();
    ///
    /// let reports = language.reports.len();
    ///
    /// language.dirs();
    ///
    /// assert_eq!(reports + 10, language.reports.len());
    /// ```
    pub fn dirs(&mut self) {
        let empty = Path::new("");

        let reports: Vec<Report> = {
            let mut dir_reports = HashMap::new();

            for child_report in &self.reports {
                let mut path = child_report.name.as_path();
                while let Some(dir) = path.parent() {
                    path = dir;
                    if path == empty {
                        continue;
                    }

                    let dir_report = dir_reports
                        .entry(path)
                        .or_insert_with(|| {
                            let mut report = Report::default();
                            report.name = path.to_path_buf();
                            report
                        });

                    dir_report.stats.comments += child_report.stats.comments;
                    dir_report.stats.code += child_report.stats.code;
                    dir_report.stats.blanks += child_report.stats.blanks;
                }
            }

            dir_reports.into_values().collect()
        };

        self.reports.extend(reports);
    }

    /// Totals up the statistics of the `Stat` structs currently contained in
    /// the language.
    ///
    /// ```no_run
    /// use std::{collections::BTreeMap, path::PathBuf};
    /// use tokei::Language;
    ///
    /// let mut language = Language::new();
    ///
    /// // Add stats...
    ///
    /// assert_eq!(0, language.lines());
    ///
    /// language.total();
    ///
    /// assert_eq!(10, language.lines());
    /// ```
    pub fn total(&mut self) {
        let mut blanks = 0;
        let mut code = 0;
        let mut comments = 0;

        for report in &self.reports {
            blanks += report.stats.blanks;
            code += report.stats.code;
            comments += report.stats.comments;
        }

        self.blanks = blanks;
        self.code = code;
        self.comments = comments;
    }

    /// Checks if the language is empty. Empty meaning it doesn't have any
    /// statistics.
    ///
    /// ```
    /// # use tokei::*;
    /// let rust = Language::new();
    ///
    /// assert!(rust.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.code == 0 && self.comments == 0 && self.blanks == 0 && self.children.is_empty()
    }

    /// Sorts each of the `Report`s contained in the language based
    /// on what category is provided.
    ///
    /// ```no_run
    /// use std::{collections::BTreeMap, path::PathBuf};
    /// use tokei::{Language, Sort};
    ///
    /// let mut language = Language::new();
    ///
    /// // Add stats...
    ///
    /// language.sort_by(Sort::Lines);
    /// assert_eq!(20, language.reports[0].stats.lines());
    ///
    /// language.sort_by(Sort::Code);
    /// assert_eq!(8, language.reports[0].stats.code);
    /// ```
    pub fn sort_by(&mut self, category: Sort) {
        match category {
            Sort::Blanks => self
                .reports
                .sort_by(|a, b| b.stats.blanks.cmp(&a.stats.blanks)),
            Sort::Comments => self
                .reports
                .sort_by(|a, b| b.stats.comments.cmp(&a.stats.comments)),
            Sort::Code => self.reports.sort_by(|a, b| b.stats.code.cmp(&a.stats.code)),
            Sort::Files => self.reports.sort_by(|a, b| a.name.cmp(&b.name)),
            Sort::Lines => self
                .reports
                .sort_by(|a, b| b.stats.lines().cmp(&a.stats.lines())),
        }
    }
}

impl AddAssign for Language {
    fn add_assign(&mut self, mut rhs: Self) {
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
        self.code += rhs.code;
        self.reports.extend(mem::take(&mut rhs.reports));
        self.children.extend(mem::take(&mut rhs.children));
        self.inaccurate |= rhs.inaccurate;
    }
}
