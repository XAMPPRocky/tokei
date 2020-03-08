use std::path::PathBuf;

/// A struct representing the statistics of a file.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
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
    /// Create a new `Stats` from a [`PathBuf`].
    ///
    /// [`PathBuf`]: //doc.rust-lang.org/std/path/struct.PathBuf.html
    pub fn new(name: PathBuf) -> Self {
        Stats {
            blanks: 0,
            code: 0,
            comments: 0,
            lines: 0,
            name,
        }
    }
}
