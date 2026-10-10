use crate::consts::{
    BLANKS_COLUMN_WIDTH, CODE_COLUMN_WIDTH, COMMENTS_COLUMN_WIDTH, LINES_COLUMN_WIDTH,
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

    /// Returns the counts outside an included subset of this file.
    #[must_use]
    pub(crate) fn without(&self, subset: &Self) -> Self {
        let mut other = Self::new();
        other.blanks = self.blanks - subset.blanks;
        other.code = self.code - subset.code;
        other.comments = self.comments - subset.comments;
        for (language, stats) in &self.blobs {
            if let Some(part) = subset.blobs.get(language) {
                other.blobs.insert(*language, stats.without(part));
            } else {
                other.blobs.insert(*language, stats.clone());
            }
        }
        other
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
    /// Whether this file is recognized as test-associated code.
    ///
    /// Applies to the entire file, including helpers, benchmarks, and examples.
    /// False means not recognized, not necessarily production code. Older
    /// serialized reports default to false.
    #[serde(default)]
    pub is_test: bool,
    /// Statistics for test code within a file that also contains other code.
    ///
    /// This is a subset of `stats`. When absent, `is_test` classifies the
    /// entire file. Older serialized reports default to no inline test data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_stats: Option<CodeStats>,
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
    ($f:expr, $this:expr, $name:expr, $max:expr) => {
        write!(
            $f,
            " {: <max$} {:>LINES_COLUMN_WIDTH$} {:>CODE_COLUMN_WIDTH$} {:>COMMENTS_COLUMN_WIDTH$} {:>BLANKS_COLUMN_WIDTH$}",
            $name,
            $this.stats.lines(),
            $this.stats.code,
            $this.stats.comments,
            $this.stats.blanks,
            max = $max
        )
    };
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.name.to_string_lossy();
        let name_length = name.len();

        // Added 2 to max length to cover wider Files column (see https://github.com/XAMPPRocky/tokei/issues/891).
        let max_len = f.width().unwrap_or(27) + 2;

        if name_length <= max_len {
            display_stats!(f, self, name, max_len)
        } else {
            let mut formatted = String::from("|");
            // Add 1 to the index to account for the '|' we add to the output string
            let from = find_char_boundary(&name, name_length + 1 - max_len);
            formatted.push_str(&name[from..]);
            display_stats!(f, self, formatted, max_len)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_round_trip_preserves_test_classification() {
        let mut report = Report::new("main_test.go".into());
        report.is_test = true;
        report.stats.code = 3;
        let json = serde_json::to_string(&report).unwrap();
        let restored: Report = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, report);
    }

    #[test]
    fn old_report_defaults_to_unclassified() {
        let json =
            r#"{"name":"main_test.go","stats":{"code":3,"comments":0,"blanks":0,"blobs":{}}}"#;
        let report: Report = serde_json::from_str(json).unwrap();
        assert!(!report.is_test);
        assert_eq!(report.stats.code, 3);
    }
}
