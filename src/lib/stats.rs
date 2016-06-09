use std::fmt;

/// A struct representing the statistics of a file.
#[derive(Clone, Default, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
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


impl Stats {
    /// Create a new `Stats` from a file path.
    ///
    /// ```
    /// let stats = Stats::new("src/main.rs");
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Stats { name: name.into(), ..Self::default() }
    }
}


impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name_length = self.name.len();

        let name = if name_length == 25 {
            self.name.clone()
        } else if self.name.len() > 24 {
            let mut name = String::from("|");
            name.push_str(&self.name[self.name.len() - 24..]);
            name
        } else {
            self.name.clone()
        };
        write!(f,
               " {: <25} {:>12} {:>12} {:>12} {:>12}",
               name,
               self.lines,
               self.code,
               self.comments,
               self.blanks)
    }
}
