/// Struct representing a single Language.
#[cfg(feature = "io")]
#[cfg_attr(feature = "io", derive(Clone, Debug, Deserialize, Serialize))]
pub struct Language {
    /// Number of blank lines.
    pub blanks: usize,
    /// Number of lines of code.
    pub code: usize,
    /// Number of comments(both single, and multi-line)
    pub comments: usize,
    /// A collection of files to be analysed.
    #[serde(skip_deserializing, skip_serializing)]
    pub files: Vec<PathBuf>,
    /// A collection of statistics based on the files provide from `files`
    pub stats: Vec<Stats>,
    /// Number of total lines.
    pub lines: usize,
    /// A collection of single line comments in the language. ie. `//` in Rust.
    #[serde(skip_deserializing, skip_serializing)]
    pub line_comment: Vec<&'static str>,
    /// A collection of tuples representing the start and end of multi line
    /// comments. ie. `/* comment */` in Rust.
    #[serde(skip_deserializing, skip_serializing)]
    pub multi_line: Vec<(&'static str, &'static str)>,
    /// Whether the language supports nested multi line comments or not.
    #[serde(skip_deserializing, skip_serializing)]
    pub nested: bool,
    /// A list of specific nested comments if this is empty all `multi_line`
    /// comments count.
    #[serde(skip_deserializing, skip_serializing)]
    pub nested_comments: Vec<(&'static str, &'static str)>,
    /// A list of quotes by default it is `""`.
    #[serde(skip_deserializing, skip_serializing)]
    pub quotes: Vec<(&'static str, &'static str)>,
    #[serde(skip_deserializing, skip_serializing)]
    pub regex: Option<Cow<'static, Regex>>
}

#[cfg(not(feature = "io"))]
#[derive(Clone, Debug)]
pub struct Language {
    /// Number of blank lines.
    pub blanks: usize,
    /// Number of lines of code.
    pub code: usize,
    /// Number of comments(both single, and multi-line)
    pub comments: usize,
    /// A collection of files to be analysed.
    pub files: Vec<PathBuf>,
    /// A collection of statistics based on the files provide from `files`
    pub stats: Vec<Stats>,
    /// Number of total lines.
    pub lines: usize,
    /// A collection of single line comments in the language. ie. `//` in Rust.
    pub line_comment: Vec<&'static str>,
    /// A collection of tuples representing the start and end of multi line
    /// comments. ie. `/* comment */` in Rust.
    pub multi_line: Vec<(&'static str, &'static str)>,
    /// Whether the language supports nested multi line comments or not.
    pub nested: bool,
    /// A list of specific nested comments if this is empty all `multi_line`
    /// comments count.
    pub nested_comments: Vec<(&'static str, &'static str)>,
    /// A list of quotes by default it is `""`.
    pub quotes: Vec<(&'static str, &'static str)>,
    /// A regular expression for searching for multi line comments.
    pub regex: Option<Cow<'static, Regex>>
}

