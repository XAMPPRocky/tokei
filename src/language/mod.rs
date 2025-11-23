mod embedding;
pub mod language_type;
pub mod languages;
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
    /// A map of any languages found in the reports (embedded languages like Rust in Markdown).
    /// Keys are language names as strings.
    pub children: BTreeMap<String, Vec<Report>>,
    /// A map of classified files (e.g., Tests, Generated, Benchmarks).
    /// Unlike children (embedded languages), these ARE separate files and count toward file totals.
    /// Keys are classification names.
    #[serde(default)]
    pub classifications: BTreeMap<String, Vec<Report>>,
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

    /// Returns the total number of files, including classified files.
    /// Unlike children (embedded languages), classifications are separate files.
    #[inline]
    #[must_use]
    pub fn file_count(&self) -> usize {
        let classified_count: usize = self.classifications.values().map(|v| v.len()).sum();
        self.reports.len() + classified_count
    }

    /// Add a `Report` to the Language. This will not update the totals in the
    /// Language struct.
    pub fn add_report(&mut self, report: Report) {
        for (lang, stats) in &report.stats.blobs {
            let mut new_report = Report::new(report.name.clone());
            new_report.stats = stats.clone();

            self.children
                .entry(lang.to_string())
                .or_default()
                .push(new_report);
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

        // Add embedded languages (children) - these should be fully summarised
        let children_stats = self
            .children
            .values()
            .flat_map(|reports| reports.iter().map(|r| r.stats.summarise()));

        for stats in children_stats {
            summary.comments += stats.comments;
            summary.code += stats.code;
            summary.blanks += stats.blanks;
        }

        // Add classified reports' direct stats only (NOT summarised)
        // Their embedded languages are already counted in children above
        let classification_stats = self
            .classifications
            .values()
            .flat_map(|reports| reports.iter().map(|r| &r.stats));

        for stats in classification_stats {
            summary.comments += stats.comments;
            summary.code += stats.code;
            summary.blanks += stats.blanks;
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
        self.total_with_classifications(false);
    }

    /// Calculate totals, optionally separating classified reports into classifications.
    /// If `extract_classified` is true, classified reports are moved to
    /// `classifications` map under their classification name.
    pub fn total_with_classifications(&mut self, extract_classified: bool) {
        if extract_classified {
            // Partition reports by having classification or not
            let (classified, unclassified): (Vec<_>, Vec<_>) = std::mem::take(&mut self.reports)
                .into_iter()
                .partition(|r| r.classification.is_some());

            // Group classified reports by name
            for report in classified {
                let classification_name = report.classification.clone().unwrap();
                self.classifications
                    .entry(classification_name)
                    .or_default()
                    .push(report);
            }

            // Keep only unclassified files in reports
            self.reports = unclassified;
        }

        // Calculate totals from remaining reports
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
    /// statistics, no children and no classifications.
    ///
    /// ```
    /// # use tokei::*;
    /// let rust = Language::new();
    ///
    /// assert!(rust.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.code == 0
            && self.comments == 0
            && self.blanks == 0
            && self.children.is_empty()
            && self.classifications.is_empty()
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
        self.classifications
            .extend(mem::take(&mut rhs.classifications));
        self.inaccurate |= rhs.inaccurate;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::Report;
    use std::path::PathBuf;

    // Test helper functions

    fn create_report_with_stats(
        path: &str,
        code: usize,
        comments: usize,
        blanks: usize,
        classification: Option<&str>,
    ) -> Report {
        let mut report = Report::new(PathBuf::from(path));
        report.stats.code = code;
        report.stats.comments = comments;
        report.stats.blanks = blanks;
        report.classification = classification.map(|s| s.to_string());
        report
    }

    fn create_classified_report(path: &str, classification: &str) -> Report {
        let mut report = Report::new(PathBuf::from(path));
        report.classification = Some(classification.to_string());
        report
    }

    #[test]
    fn test_total_separates_classified_reports() {
        let mut lang = Language::new();

        lang.add_report(create_report_with_stats("src/main.js", 100, 20, 10, None));
        lang.add_report(create_report_with_stats(
            "src/main.test.js",
            50,
            10,
            5,
            Some("Tests"),
        ));
        lang.add_report(create_report_with_stats(
            "src/schema.generated.ts",
            200,
            5,
            15,
            Some("Generated"),
        ));

        // Call total with classification separation enabled
        lang.total_with_classifications(true);

        // Should have only unclassified stats in main totals
        assert_eq!(lang.code, 100);
        assert_eq!(lang.comments, 20);
        assert_eq!(lang.blanks, 10);

        // Should have only unclassified report
        assert_eq!(lang.reports.len(), 1);
        assert_eq!(lang.reports[0].classification, None);

        // Should have 2 classification categories in classifications (NOT children!)
        assert_eq!(lang.classifications.len(), 2);
        assert_eq!(lang.children.len(), 0); // children is for embedded languages only

        // Check Tests classification
        let tests = lang.classifications.get("Tests").unwrap();
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].stats.code, 50);

        // Check Generated classification
        let generated = lang.classifications.get("Generated").unwrap();
        assert_eq!(generated.len(), 1);
        assert_eq!(generated[0].stats.code, 200);
    }

    #[test]
    fn test_file_count_includes_classifications() {
        let mut lang = Language::new();

        // Add 2 unclassified files
        lang.add_report(Report::new(PathBuf::from("src/main.js")));
        lang.add_report(Report::new(PathBuf::from("src/utils.js")));

        // Add 3 test files (classified)
        lang.add_report(create_classified_report("src/main.test.js", "Tests"));
        lang.add_report(create_classified_report("src/utils.test.js", "Tests"));
        lang.add_report(create_classified_report("src/integration.test.js", "Tests"));

        lang.total_with_classifications(true);

        // Total file count should be 5 (2 unclassified + 3 test)
        let file_count = lang.file_count();
        assert_eq!(file_count, 5);
    }

    #[test]
    fn test_language_totals_include_classified_code() {
        let mut lang = Language::new();

        lang.add_report(create_report_with_stats("main.js", 100, 20, 10, None));
        lang.add_report(create_report_with_stats(
            "main.test.js",
            50,
            10,
            5,
            Some("Tests"),
        ));

        // After calling total_with_classifications(true),
        // the Language's totals should only reflect unclassified code
        lang.total_with_classifications(true);

        // The main language totals should only reflect unclassified code
        // (classified code is separated out)
        assert_eq!(lang.code, 100);
        assert_eq!(lang.comments, 20);
        assert_eq!(lang.blanks, 10);

        // But summarise() should include BOTH unclassified and classified
        let summary = lang.summarise();
        assert_eq!(summary.code, 150); // 100 + 50
        assert_eq!(summary.comments, 30); // 20 + 10
        assert_eq!(summary.blanks, 15); // 10 + 5
    }

    #[test]
    fn test_is_empty_with_classifications() {
        let mut lang = Language::new();

        // Initially empty
        assert!(lang.is_empty());

        // Add a classified file
        lang.add_report(create_report_with_stats("test.js", 50, 0, 0, Some("Tests")));

        lang.total_with_classifications(true);

        // After classification separation, main stats are 0 but classifications exist
        assert_eq!(lang.code, 0);
        assert_eq!(lang.comments, 0);
        assert_eq!(lang.blanks, 0);
        assert_eq!(lang.reports.len(), 0);
        assert_eq!(lang.children.len(), 0);
        assert_eq!(lang.classifications.len(), 1);

        // Should NOT be empty because classifications exist
        assert!(!lang.is_empty());
    }

    #[test]
    fn test_summarise_does_not_double_count_embedded_languages_in_classified_files() {
        use crate::stats::CodeStats;
        use crate::LanguageType;

        let mut lang = Language::new();

        // Create a classified HTML test file with embedded JavaScript
        // Using prime numbers to avoid accidental correct results from multiplication
        let mut html_report = create_report_with_stats("test.html", 53, 11, 7, Some("Tests"));

        // Add embedded JavaScript stats to the HTML file's blobs
        let mut js_stats = CodeStats::new();
        js_stats.code = 31;
        js_stats.comments = 5;
        js_stats.blanks = 3;
        html_report
            .stats
            .blobs
            .insert(LanguageType::JavaScript, js_stats);

        // add_report will automatically extract the blobs into children
        lang.add_report(html_report);

        // Separate classifications
        lang.total_with_classifications(true);

        // Summarise should NOT double count the JavaScript
        let summary = lang.summarise();

        // Expected: HTML (53+11+7) + JavaScript (31+5+3) = 110
        // Bug would give: HTML (53+11+7) + JavaScript (31+5+3) + JavaScript again (31+5+3) = 149
        assert_eq!(
            summary.code,
            53 + 31,
            "Code should not be double counted (HTML 53 + JS 31 = 84)"
        );
        assert_eq!(
            summary.comments,
            11 + 5,
            "Comments should not be double counted (HTML 11 + JS 5 = 16)"
        );
        assert_eq!(
            summary.blanks,
            7 + 3,
            "Blanks should not be double counted (HTML 7 + JS 3 = 10)"
        );
    }
}
