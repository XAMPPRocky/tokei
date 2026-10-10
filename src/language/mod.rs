mod embedding;
pub mod language_type;
pub mod languages;
mod rust_tests;
mod syntax;

use std::{collections::BTreeMap, mem, ops::AddAssign};

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

    /// Returns statistics for recognized test code, including its file reports.
    ///
    /// These counts are a subset of the existing totals, not additional lines.
    /// Older saved reports without test classification are not reclassified when loaded.
    /// Embedded languages retain the same representation as in this language;
    /// use [`Language::summarise`] to include them in the returned counts.
    ///
    /// ```no_run
    /// use tokei::{Config, LanguageType, Languages};
    /// let mut languages = Languages::new();
    /// languages.get_statistics(&["."], &[], &Config::default());
    /// if let Some(go) = languages.get(&LanguageType::Go) {
    ///     let tests = go.test_statistics();
    ///     println!("{} test files, {} code lines", tests.reports.len(), tests.code);
    /// }
    /// ```
    #[must_use]
    pub fn test_statistics(&self) -> Self {
        self.statistics_for_tests(true)
    }

    /// Returns statistics outside recognized test code.
    ///
    /// Together with [`Language::test_statistics`], this partitions counted lines.
    /// A file containing inline tests can appear in both subsets. Other lines
    /// are not guaranteed to be production code. Embedded languages retain
    /// their usual representation; use [`Language::summarise`] to include them.
    #[must_use]
    pub fn non_test_statistics(&self) -> Self {
        self.statistics_for_tests(false)
    }

    fn statistics_for_tests(&self, is_test: bool) -> Self {
        let mut subset = Self::new();
        for report in &self.reports {
            let stats = match (is_test, report.is_test, &report.test_stats) {
                (true, true, _) | (false, false, None) => Some(report.stats.clone()),
                (true, false, Some(tests)) => Some(tests.clone()),
                (false, false, Some(tests)) => Some(report.stats.without(tests)),
                _ => None,
            };
            if let Some(stats) = stats {
                if stats.summarise().lines() > 0 {
                    let mut part = report.clone();
                    part.stats = stats;
                    part.is_test = is_test;
                    part.test_stats = None;
                    subset.add_report(part);
                }
            }
        }
        subset.total();
        subset.inaccurate = self.inaccurate;
        subset
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_partition_reports_without_changing_totals() {
        let mut test = Report::new("main_test.go".into());
        test.is_test = true;
        test.stats.code = 3;
        test.stats.comments = 2;
        test.stats.blanks = 1;
        let mut other = Report::new("main.go".into());
        other.stats.code = 7;
        let mut language = Language::new();
        language.add_report(test.clone());
        language.add_report(other.clone());
        language.total();
        let original = language.clone();

        let tests = language.test_statistics();
        assert_eq!(tests.reports, vec![test]);
        assert_eq!(tests.code, 3);
        assert_eq!(tests.comments, 2);
        assert_eq!(tests.blanks, 1);
        let non_tests = language.non_test_statistics();
        assert_eq!(non_tests.reports, vec![other]);
        assert_eq!(non_tests.code, 7);
        assert_eq!(non_tests.comments, 0);
        assert_eq!(non_tests.blanks, 0);
        assert_eq!(tests.lines() + non_tests.lines(), language.lines());
        assert_eq!(language, original);
    }
    #[test]
    fn embedded_counts_are_included_once() {
        let mut report = Report::new("test.html".into());
        report.is_test = true;
        report.stats.code = 3;
        let mut embedded = crate::CodeStats::new();
        embedded.code = 5;
        report
            .stats
            .blobs
            .insert(LanguageType::JavaScript, embedded);
        let mut language = Language::new();
        language.add_report(report);
        language.total();
        let tests = language.test_statistics();
        assert_eq!(tests.code, 3);
        assert_eq!(tests.summarise().code, 8);
        assert_eq!(tests.reports.len(), 1);
        assert_eq!(language.summarise().code, 8);
    }
    #[test]
    fn partitions_handle_all_test_and_no_test_files() {
        for is_test in [false, true] {
            let mut report = Report::new("example.go".into());
            report.is_test = is_test;
            report.stats.code = 7;
            let mut language = Language::new();
            language.add_report(report);
            language.total();
            let tests = language.test_statistics();
            let non_tests = language.non_test_statistics();
            assert_eq!(tests.code, if is_test { 7 } else { 0 });
            assert_eq!(non_tests.code, if is_test { 0 } else { 7 });
            assert_eq!(tests.reports.len() + non_tests.reports.len(), 1);
        }
    }
}
