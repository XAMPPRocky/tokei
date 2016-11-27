/// A struct representing the statistics of a file.
#[cfg_attr(feature = "io", derive(Deserialize, Serialize))]
#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Stats {
    /// Number of blank lines within the file.
    pub blanks: usize,
    /// Number of lines of code within the file.
    pub code: usize,
    /// Number of comments within the file. (_includes both multi line, and single line comments_)
    pub comments: usize,
    /// Total number of lines within the file.
    pub lines: usize,
    /// File name.
    pub name: String,
}

