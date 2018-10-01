// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE - MIT} file.

pub mod languages;
pub mod language_type;

use std::mem;
use std::ops::AddAssign;

pub use self::languages::Languages;
pub use self::language_type::*;

use sort::Sort::*;
use sort::Sort;
use stats::Stats;

/// Struct representing a single Language.
#[cfg_attr(feature = "io", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default)]
pub struct Language {
    /// Number of blank lines.
    pub blanks: usize,
    /// Number of lines of code.
    pub code: usize,
    /// Number of comments(both single, and multi-line)
    pub comments: usize,
    /// Number of total lines.
    pub lines: usize,
    /// A collection of statistics based on the files provide from `files`
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

    /// Adds file stats to the Language.
    pub fn add_stat(&mut self, stat: Stats) {
        self.stats.push(stat);
    }

    /// Marks this language as possibly not reflecting correct stats.
    #[inline]
    pub fn mark_inaccurate(&mut self) {
        self.inaccurate = true;
    }

    /// Totals up all the statistics currently in the language.
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
    /// on what category is provided
    /// panic!'s if given the wrong category.
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
        if rhs.inaccurate {
            self.inaccurate = rhs.inaccurate
        };
    }
}

