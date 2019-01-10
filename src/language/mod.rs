// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE - MIT} file.

pub mod languages;
pub mod language_type;
mod syntax;

use std::mem;
use std::ops::AddAssign;

pub use self::languages::Languages;
pub use self::language_type::*;

use crate::sort::Sort::{self, *};
use crate::stats::Stats;

/// A struct representing statistics about a single Language.
#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct Language {
    /// The total number of blank lines.
    pub blanks: usize,
    /// The total number of lines of code.
    pub code: usize,
    /// The total number of comments(both single, and multi-line)
    pub comments: usize,
    /// The total number of total lines.
    pub lines: usize,
    /// A collection of statistics of individual files.
    pub stats: Vec<Stats>,
    /// Whether this language had problems with file parsing
    pub inaccurate: bool,
}

impl Language {
    /// Constructs a new empty Language with the comments provided.
    ///
    /// ```
    /// # use tokei::*;
    /// let mut rust = Language::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a `Stat` to the Language. This will not update the totals in the
    /// Language struct.
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use tokei::{Language, Stats};
    ///
    /// let mut language = Language::new();
    ///
    /// language.add_stat(Stats {
    ///     lines: 10,
    ///     code: 4,
    ///     comments: 3,
    ///     blanks: 3,
    ///     name: PathBuf::from("test.rs"),
    /// });
    /// ```
    pub fn add_stat(&mut self, stat: Stats) {
        self.stats.push(stat);
    }

    /// Marks this language as possibly not reflecting correct stats.
    #[inline]
    pub fn mark_inaccurate(&mut self) {
        self.inaccurate = true;
    }

    /// Totals up the statistics of the `Stat` structs currently contained in
    /// the language.
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use tokei::{Language, Stats};
    ///
    /// let mut language = Language::new();
    ///
    /// language.add_stat(Stats {
    ///     lines: 10,
    ///     code: 4,
    ///     comments: 3,
    ///     blanks: 3,
    ///     name: PathBuf::from("test.rs"),
    /// });
    ///
    /// assert_eq!(0, language.lines);
    ///
    /// language.total();
    ///
    /// assert_eq!(10, language.lines);
    /// ```
    pub fn total(&mut self) {
        let mut blanks = 0;
        let mut code = 0;
        let mut comments = 0;

        for stat in &self.stats {
            blanks += stat.blanks;
            code += stat.code;
            comments += stat.comments;
        }

        self.blanks = blanks;
        self.code = code;
        self.comments = comments;
        self.lines = blanks + code + comments;
    }

    /// Checks if the language is empty. Empty meaning it doesn't have any
    /// statistics.
    ///
    /// ```
    /// # use tokei::*;
    /// let rust = Language::new();
    ///
    /// assert!(rust.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.code == 0 &&
        self.comments == 0 &&
        self.blanks == 0 &&
        self.lines == 0
    }

    /// Sorts each of the `Stats` structs contained in the language based
    /// on what category is provided.
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use tokei::{Language, Sort, Stats};
    ///
    /// let mut language = Language::new();
    ///
    /// language.add_stat(Stats {
    ///     lines: 10,
    ///     code: 8,
    ///     comments: 0,
    ///     blanks: 2,
    ///     name: PathBuf::from("test.rs"),
    /// });
    ///
    /// language.add_stat(Stats {
    ///     lines: 20,
    ///     code: 4,
    ///     comments: 13,
    ///     blanks: 3,
    ///     name: PathBuf::from("testsuite.rs"),
    /// });
    ///
    /// language.sort_by(Sort::Lines);
    /// assert_eq!(20, language.stats[0].lines);
    ///
    /// language.sort_by(Sort::Code);
    /// assert_eq!(8, language.stats[0].code);
    /// ```
    pub fn sort_by(&mut self, category: Sort) {
        match category {
            Blanks => self.stats.sort_by(|a, b| b.blanks.cmp(&a.blanks)),
            Comments => self.stats.sort_by(|a, b| b.comments.cmp(&a.comments)),
            Code => self.stats.sort_by(|a, b| b.code.cmp(&a.code)),
            Files => self.stats.sort_by(|a, b| a.name.cmp(&b.name)),
            Lines => self.stats.sort_by(|a, b| b.lines.cmp(&a.lines)),
        }
    }

}

impl AddAssign for Language {
    fn add_assign(&mut self, mut rhs: Self) {
        self.lines += rhs.lines;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
        self.code += rhs.code;
        self.stats.extend(mem::replace(&mut rhs.stats, Vec::new()));
        self.inaccurate |= rhs.inaccurate
    }
}

