use std::fmt;

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


impl Stats {
    /// Create a new `Stats` from a file path.
    ///
    /// ```
    /// # use tokei::*;
    /// let stats = Stats::new("src/main.rs");
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Stats { name: name.into(), ..Self::default() }
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

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name_length = self.name.len();

        let name = if name_length == 25 {
            self.name.clone()
        } else if self.name.len() > 24 {
            let mut name = String::from("|");
            let from = find_char_boundary(&self.name, self.name.len() - 24);
            name.push_str(&self.name[from..]);
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
