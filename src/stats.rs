use crate::consts::{
    BLANKS_COLUMN_WIDTH, CODE_COLUMN_WIDTH, COMMENTS_COLUMN_WIDTH, FILES_COLUMN_WIDTH,
    LINES_COLUMN_WIDTH,
};
use crate::LanguageType;
use std::{collections::BTreeMap, fmt, ops, path::PathBuf};

/// A struct representing stats about a single blob of code.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[non_exhaustive]
pub struct CodeStats {
    /// The blank lines in the blob.
    pub blanks: usize,
    /// The lines of code in the blob.
    pub code: usize,
    /// The lines of comments in the blob.
    pub comments: usize,
    /// Language blobs that were contained inside this blob.
    pub blobs: BTreeMap<LanguageType, CodeStats>,
}

impl CodeStats {
    /// Creates a new blank `CodeStats`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the total lines in a blob of code.
    #[must_use]
    pub fn lines(&self) -> usize {
        self.blanks + self.code + self.comments
    }

    /// Creates a new `CodeStats` from an existing one with all of the child
    /// blobs merged.
    #[must_use]
    pub fn summarise(&self) -> Self {
        let mut summary = self.clone();

        for (_, stats) in std::mem::take(&mut summary.blobs) {
            let child_summary = stats.summarise();

            summary.blanks += child_summary.blanks;
            summary.comments += child_summary.comments;
            summary.code += child_summary.code;
        }

        summary
    }
}

impl ops::AddAssign for CodeStats {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs);
    }
}

impl ops::AddAssign<&'_ CodeStats> for CodeStats {
    fn add_assign(&mut self, rhs: &'_ CodeStats) {
        self.blanks += rhs.blanks;
        self.code += rhs.code;
        self.comments += rhs.comments;

        for (language, stats) in &rhs.blobs {
            *self.blobs.entry(*language).or_default() += stats;
        }
    }
}

/// A struct representing the statistics of a file.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct Report {
    /// The code statistics found in the file.
    pub stats: CodeStats,
    /// File name.
    pub name: PathBuf,
    /// Classification category for this file (e.g., "Tests", "Generated", "Benchmarks").
    /// If None, the file is not classified.
    #[serde(default)]
    pub classification: Option<String>,
}

impl Report {
    /// Create a new `Report` from a [`PathBuf`].
    ///
    /// [`PathBuf`]: //doc.rust-lang.org/std/path/struct.PathBuf.html
    #[must_use]
    pub fn new(name: PathBuf) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }
}

impl ops::AddAssign<CodeStats> for Report {
    fn add_assign(&mut self, rhs: CodeStats) {
        self.stats += rhs;
    }
}

#[doc(hidden)]
#[must_use]
pub fn find_char_boundary(s: &str, index: usize) -> usize {
    for i in 0..4 {
        if s.is_char_boundary(index + i) {
            return index + i;
        }
    }
    unreachable!();
}

macro_rules! display_stats {
    ($f:expr, $this:expr, $name:expr, $max:expr, $class_name:expr, $class_width:expr) => {
        write!(
            $f,
            " {: <max$} {:<class_width$} {:>LINES_COLUMN_WIDTH$} {:>CODE_COLUMN_WIDTH$} {:>COMMENTS_COLUMN_WIDTH$} {:>BLANKS_COLUMN_WIDTH$}",
            $name,
            $class_name,
            $this.stats.lines(),
            $this.stats.code,
            $this.stats.comments,
            $this.stats.blanks,
            max = $max,
            class_width = $class_width
        )
    };
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.name.to_string_lossy();
        let name_length = name.len();

        // When listing individual files, classification name goes in the # of Files column,
        // truncated if needed
        let (class_name, class_width) = if let Some(ref classification) = self.classification {
            let class_len = classification.len();
            if class_len <= FILES_COLUMN_WIDTH {
                (classification.as_str(), FILES_COLUMN_WIDTH)
            } else {
                // Truncate classification to fit in FILES_COLUMN_WIDTH
                let to = find_char_boundary(classification, FILES_COLUMN_WIDTH);
                (&classification[..to], FILES_COLUMN_WIDTH)
            }
        } else {
            // Without classification, assign zero width
            ("", 0)
        };

        // Subtract class_width from available space for file name
        // Added 2 to max length to cover wider Files column (see https://github.com/XAMPPRocky/tokei/issues/891).
        let max_len = f.width().unwrap_or(27) + 2 - class_width - 1;

        if name_length <= max_len {
            display_stats!(f, self, name, max_len, class_name, class_width)
        } else {
            let mut formatted = String::from("|");
            // Add 1 to the index to account for the '|' we add to the output string
            let from = find_char_boundary(&name, name_length + 1 - max_len);
            formatted.push_str(&name[from..]);
            display_stats!(f, self, formatted, max_len, class_name, class_width)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_default_classification_is_none() {
        let report = Report::new(PathBuf::from("test.js"));
        assert_eq!(report.classification, None);
    }

    #[test]
    fn report_truncates_long_classification() {
        let mut report = Report::new(PathBuf::from("test.js"));
        report.classification = Some("VeryLongClassificationName".to_string());

        // Format the report with a specific width
        let output = format!("{:30}", report);

        // The classification should be truncated to FILES_COLUMN_WIDTH (8 chars)
        // Original: "VeryLongClassificationName" (26 chars)
        // Truncated: first 8 chars = "VeryLong"
        assert!(
            output.contains("VeryLong"),
            "Expected truncated classification 'VeryLong' in output: {}",
            output
        );
        assert!(
            !output.contains("ionName"),
            "Should not contain end of classification in output: {}",
            output
        );
    }

    #[test]
    fn report_keeps_short_classification() {
        let mut report = Report::new(PathBuf::from("test.js"));
        report.classification = Some("Test".to_string());

        let output = format!("{:30}", report);

        // Short classification should remain unchanged
        assert!(
            output.contains("Test"),
            "Expected classification 'Test' in output: {}",
            output
        );
    }

    #[test]
    fn report_classification_is_left_aligned() {
        let mut report = Report::new(PathBuf::from("test.js"));
        report.classification = Some("Test".to_string());

        let output = format!("{:30}", report);

        // Classification should be left-aligned: "Test    0" not "    Test0"
        // With left-align there should be multiple spaces after "Test", before the number
        assert!(
            output.contains("Test    "),
            "Classification should be left-aligned with trailing spaces: {}",
            output
        );
    }

    #[test]
    fn report_without_classification_uses_full_width() {
        let report_no_class = Report::new(PathBuf::from("test.js"));
        let output_no_class = format!("{:30}", report_no_class);

        let mut report_with_class = Report::new(PathBuf::from("test.js"));
        report_with_class.classification = Some("TestCls".to_string());
        let output_with_class = format!("{:30}", report_with_class);

        // Without classification, should have more spacing before first number
        let spaces_no_class = output_no_class.chars().filter(|&c| c == ' ').count();
        let spaces_with_class = output_with_class.chars().filter(|&c| c == ' ').count();

        // With no classification, Files column is 0-width, so more spaces before numbers
        assert!(
            spaces_no_class > spaces_with_class,
            "Without classification should have more spacing.\nNo class: {}\nWith class: {}",
            output_no_class,
            output_with_class
        );
    }
}
