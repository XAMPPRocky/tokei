// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE - MIT} file.

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
    #[serde(skip_serializing, skip_deserializing)]
    pub files: Vec<PathBuf>,
    pub stats: Vec<Stats>,
    pub lines: usize,
    #[serde(skip_deserializing,skip_serializing, rename(serialize="lineComment"))]
    pub line_comment: &'static str,
    #[serde(skip_deserializing,skip_serializing, rename(serialize="multiLine"))]
    pub multi_line: &'static str,
    #[serde(skip_deserializing,skip_serializing, rename(serialize="multiLineEnd"))]
    pub multi_line_end: &'static str,
    pub total_files: usize,
}

impl Language {
    pub fn new(line_comment: &'static str,
               multi_line: &'static str,
               multi_line_end: &'static str)
               -> Self {

        Language {
            line_comment: line_comment,
            multi_line: multi_line,
            multi_line_end: multi_line_end,
            ..Self::default()
        }
    }

    pub fn new_c() -> Self {
        Language {
            line_comment: "//",
            multi_line: "/*",
            multi_line_end: "*/",
            ..Self::default()
        }
    }

    pub fn new_html() -> Self {
        Language {
            line_comment: "<!--",
            multi_line: "<!--",
            multi_line_end: "-->",
            ..Self::default()
        }
    }

    pub fn new_blank() -> Self {
        Self::default()
    }

    pub fn new_func() -> Self {
        Language {
            multi_line: "(*",
            multi_line_end: "*)",
            ..Self::default()
        }
    }

    pub fn new_hash() -> Self {
        Self::new_single("#")
    }

    pub fn new_multi(multi_line: &'static str, multi_line_end: &'static str) -> Self {
        Language {
            multi_line: multi_line,
            multi_line_end: multi_line_end,
            ..Self::default()
        }
    }

    pub fn new_pro() -> Self {
        Language {
            line_comment: "%",
            multi_line: "/*",
            multi_line_end: "*/",
            ..Self::default()
        }
    }

    pub fn new_single(line_comment: &'static str) -> Self {
        Language { line_comment: line_comment, ..Self::default() }
    }
    pub fn is_empty(&self) -> bool {
        self.code == 0 && self.comments == 0 && self.blanks == 0 && self.lines == 0
    }

    pub fn is_blank(&self) -> bool {
        self.line_comment == "" && self.multi_line == ""
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

// Adding a file to the language.
impl AddAssign<Stats> for Language {
    fn add_assign(&mut self, rhs: Stats) {
        self.lines += rhs.lines;
        self.code += rhs.code;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
        self.stats.push(rhs);
    }
}

// impl AddAssign<BTreeMap<LanguageName, Language>> for BTreeMap<LanguageName, Language> {
//     fn add_assign(&mut self, rhs: BTreeMap<LanguageName, Language>) {
//         for (name, rhs_language) in rhs {
//             if let Some(language) = self.get_mut(name) {
//                 language += rhs_language;
//             }
//         }
//     }
// }
