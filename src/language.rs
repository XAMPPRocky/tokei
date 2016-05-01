// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE - MIT} file.

use std::cell::RefCell;
use std::fmt;
use std::path::PathBuf;
use std::ops::AddAssign;
use stats::Stats;

#[derive(Debug, Default)]
pub struct Language<'a> {
    pub name: &'a str,
    pub line_comment: &'a str,
    pub multi_line: &'a str,
    pub multi_line_end: &'a str,
    pub files: Vec<PathBuf>,
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,
    pub lines: usize,
    pub total: usize,
    pub printed: bool,
}


impl<'a> Language<'a> {
    pub fn new(name: &'a str,
               line_comment: &'a str,
               multi_line: &'a str,
               multi_line_end: &'a str)
               -> RefCell<Self> {

        RefCell::new(Language {
            name: name,
            line_comment: line_comment,
            multi_line: multi_line,
            multi_line_end: multi_line_end,
            ..Self::default()
        })
    }

    pub fn new_raw(name: &'a str) -> Self {
        Language { name: name, ..Self::default() }
    }

    pub fn new_c(name: &'a str) -> RefCell<Self> {
        RefCell::new(Language {
            name: name,
            line_comment: "//",
            multi_line: "/*",
            multi_line_end: "*/",
            ..Self::default()
        })
    }

    pub fn new_html(name: &'a str) -> RefCell<Self> {
        RefCell::new(Language {
            name: name,
            line_comment: "<!--",
            multi_line: "<!--",
            multi_line_end: "-->",
            ..Self::default()
        })
    }

    pub fn new_blank(name: &'a str) -> RefCell<Self> {
        RefCell::new(Language { name: name, ..Self::default() })
    }

    pub fn new_single(name: &'a str, line_comment: &'a str) -> RefCell<Self> {
        RefCell::new(Language {
            name: name,
            line_comment: line_comment,
            ..Self::default()
        })
    }

    pub fn new_multi(name: &'a str, multi_line: &'a str, multi_line_end: &'a str) -> RefCell<Self> {
        RefCell::new(Language {
            name: name,
            multi_line: multi_line,
            multi_line_end: multi_line_end,
            ..Self::default()
        })
    }

    pub fn is_empty(&self) -> bool {
        self.code == 0 && self.comments == 0 && self.blanks == 0 && self.lines == 0
    }
}

impl<'a> fmt::Display for Language<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let total = if self.total == 0 {
            self.files.len()
        } else {
            self.total
        };
        write!(f,
               " {: <18} {: >6} {:>12} {:>12} {:>12} {:>12}",
               self.name,
               total,
               self.lines,
               self.blanks,
               self.comments,
               self.code)
    }
}
// Adding languages to the raw total.
impl<'a, 'b> AddAssign<&'b Language<'a>> for Language<'a> {
    fn add_assign(&mut self, rhs: &Self) {
        self.total += rhs.files.len();
        self.lines += rhs.lines;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
        self.code += rhs.code;
    }
}

// Adding a file to the language.
impl<'a> AddAssign<Stats> for Language<'a> {
    fn add_assign(&mut self, rhs: Stats) {
        self.lines += rhs.lines;
        self.code += rhs.code;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
    }
}
