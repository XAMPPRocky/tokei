// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE - MIT} file.

mod language_name;

pub use language_name::*;

use std::path::PathBuf;
use std::ops::AddAssign;

use consts::*;
use language_name::LanguageName;
use stats::Stats;

#[derive(Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Language {
    pub blanks: usize,
    pub code: usize,
    pub comments: usize,
    #[serde(skip_deserializing, skip_serializing)]
    pub files: Vec<PathBuf>,
    pub stats: Vec<Stats>,
    pub lines: usize,
    #[serde(skip_deserializing, skip_serializing)]
    pub line_comment: Vec<&'static str>,
    #[serde(skip_deserializing, skip_serializing)]
    pub multi_line: Vec<(&'static str, &'static str)>,
    #[serde(skip_deserializing, skip_serializing)]
    pub nested: bool,
    pub total_files: usize,
}

impl Language {
    pub fn new(line_comment: Vec<&'static str>,
               multi_line: Vec<(&'static str, &'static str)>)
               -> Self {

        Language {
            line_comment: line_comment,
            multi_line: multi_line,
            ..Self::default()
        }
    }

    pub fn nested(mut self) -> Self {
        self.nested = true;
        self
    }

    pub fn new_c() -> Self {
        Language {
            line_comment: vec!["//"],
            multi_line: vec![("/*", "*/")],
            ..Self::default()
        }
    }

    pub fn new_html() -> Self {
        Language { multi_line: vec![("<!--", "-->")], ..Self::default() }
    }

    pub fn new_blank() -> Self {
        Self::default()
    }

    pub fn new_func() -> Self {
        Language { multi_line: vec![("(*", "*)")], ..Self::default() }
    }

    pub fn new_hash() -> Self {
        Self::new_single(vec!["#"])
    }

    pub fn new_multi(multi_line: Vec<(&'static str, &'static str)>) -> Self {
        Language { multi_line: multi_line, ..Self::default() }
    }

    pub fn new_pro() -> Self {
        Language {
            line_comment: vec!["%"],
            multi_line: vec![("/*", "*/")],
            ..Self::default()
        }
    }

    pub fn new_single(line_comment: Vec<&'static str>) -> Self {
        Language { line_comment: line_comment, ..Self::default() }
    }

    pub fn is_empty(&self) -> bool {
        self.code == 0 && self.comments == 0 && self.blanks == 0 && self.lines == 0
    }

    pub fn is_blank(&self) -> bool {
        self.line_comment.is_empty() && self.multi_line.is_empty()
    }

    pub fn sort_by(&mut self, category: &str) {
        match category {
            BLANKS => self.stats.sort_by(|a, b| b.blanks.cmp(&a.blanks)),
            COMMENTS => self.stats.sort_by(|a, b| b.comments.cmp(&a.comments)),
            CODE => self.stats.sort_by(|a, b| b.code.cmp(&a.code)),
            TOTAL => self.stats.sort_by(|a, b| b.lines.cmp(&a.lines)),
            _ => unreachable!(),
        }
    }

    pub fn print(&self, name: LanguageName) {
        println!(" {: <18} {: >6} {:>12} {:>12} {:>12} {:>12}",
                 name.name(),
                 self.total_files,
                 self.lines,
                 self.code,
                 self.comments,
                 self.blanks)
    }
}

impl AddAssign for Language {
    fn add_assign(&mut self, rhs: Self) {
        self.total_files += rhs.total_files;
        self.lines += rhs.lines;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
        self.code += rhs.code;
    }
}

impl<'a> AddAssign<&'a Language> for Language {
    fn add_assign(&mut self, rhs: &'a Self) {
        self.total_files += rhs.total_files;
        self.lines += rhs.lines;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
        self.code += rhs.code;
    }
}

impl<'a> AddAssign<&'a mut Language> for Language {
    fn add_assign(&mut self, rhs: &mut Self) {
        self.total_files += rhs.total_files;
        self.lines += rhs.lines;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
        self.code += rhs.code;
    }
}

impl AddAssign<Stats> for Language {
    fn add_assign(&mut self, rhs: Stats) {
        self.lines += rhs.lines;
        self.code += rhs.code;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
        self.stats.push(rhs);
    }
}
