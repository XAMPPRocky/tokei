use std::{collections::BTreeMap, fmt, ops, path::PathBuf};

use crate::LanguageType;

/// A struct representing stats about a single blob of code.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct CodeStats {
    /// The blank lines in the blob.
    pub blanks: usize,
    /// The lines of code in the blob.
    pub code: usize,
    /// The lines of comments in the blob.
    pub comments: usize,
    /// Languages contained inside the language.
    pub contexts: BTreeMap<LanguageType, Stats>,
}

impl CodeStats {
    /// Creates a new blank `CodeStats`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the total lines in a blob of code.
    pub fn lines(&self) -> usize {
        self.blanks + self.code + self.comments
    }
}

impl ops::Add for CodeStats {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::AddAssign for CodeStats {
    fn add_assign(&mut self, rhs: Self) {
        self.blanks += rhs.blanks;
        self.code += rhs.code;
        self.comments += rhs.comments;
    }
}

/// A struct representing the statistics of a file.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Stats {
    /// Number of blank lines within the file.
    pub blanks: usize,
    /// Number of lines of code within the file.
    pub code: usize,
    /// Number of comments within the file. (_includes both multi line, and
    /// single line comments_)
    pub comments: usize,
    /// File name.
    pub name: PathBuf,
}

impl Stats {
    /// Create a new `Stats` from a [`PathBuf`].
    ///
    /// [`PathBuf`]: //doc.rust-lang.org/std/path/struct.PathBuf.html
    pub fn new(name: PathBuf) -> Self {
        Stats {
            blanks: 0,
            code: 0,
            comments: 0,
            name,
        }
    }

    /// Returns the total number of lines for a given file.
    pub fn lines(&self) -> usize {
        self.blanks + self.code + self.comments
    }
}

impl ops::AddAssign<CodeStats> for Stats {
    fn add_assign(&mut self, rhs: CodeStats) {
        self.blanks += rhs.blanks;
        self.code += rhs.code;
        self.comments += rhs.comments;
    }
}

fn find_char_boundary(s: &str, index: usize) -> usize {
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
            " {: <max$} {:>12} {:>12} {:>12} {:>12}",
            $name,
            $this.lines(),
            $this.code,
            $this.comments,
            $this.blanks,
            max = $max
        )
    };
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.name.to_string_lossy();
        let name_length = name.len();

        let max_len = f.width().unwrap_or(25);

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
