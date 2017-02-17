use std::fmt;
use std::path::PathBuf;

/// A struct representing the statistics of a file.
#[cfg_attr(feature = "io", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Stats {
    /// Number of blank lines within the file.
    pub blanks: usize,
    /// Number of lines of code within the file.
    pub code: usize,
    /// Number of comments within the file. (_includes both multi line, and
    /// single line comments_)
    pub comments: usize,
    /// Total number of lines within the file.
    pub lines: usize,
    /// File name.
    pub name: PathBuf,
}

impl Stats {
    /// Create a new `Stats` from a file path.
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use tokei::Stats;
    ///
    /// let path = PathBuf::from("src/main.rs");
    ///
    /// let stats = Stats::new(path);
    /// ```
    pub fn new(name: PathBuf) -> Self {
        Stats { name: name, ..Self::default() }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            name: PathBuf::new(),
            lines: usize::default(),
            code: usize::default(),
            comments: usize::default(),
            blanks: usize::default(),
        }
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
    ($f:expr, $this:expr, $name:expr) => {
        write!($f,
               " {: <25} {:>12} {:>12} {:>12} {:>12}",
               $name,
               $this.lines,
               $this.code,
               $this.comments,
               $this.blanks)
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.name.to_string_lossy();
        let name_length = name.len();

        if name_length == 25 || name_length <= 24 {
            display_stats!(f, self, name)
        } else {
            let mut formatted = String::from("|");
            let from = find_char_boundary(&name, name_length - 24);
            formatted.push_str(&name[from..]);
            display_stats!(f, self, formatted)
        }
    }
}
