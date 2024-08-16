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
