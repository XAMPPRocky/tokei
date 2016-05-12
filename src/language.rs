// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE - MIT} file.

use std::cell::RefCell;
use std::fmt;
use std::path::PathBuf;
use std::ops::AddAssign;
use stats::Stats;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Default, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Language {
    pub line_comment: &'static str,
    pub multi_line: &'static str,
    pub multi_line_end: &'static str,
    pub files: Vec<PathBuf>,
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,
    pub lines: usize,
    pub total: usize,
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
        Language { ..Self::default() }
    }

    pub fn new_single(line_comment: &'static str) -> Self {
        Language { line_comment: line_comment, ..Self::default() }
    }

    pub fn new_multi(multi_line: &'static str, multi_line_end: &'static str) -> Self {
        Language {
            multi_line: multi_line,
            multi_line_end: multi_line_end,
            ..Self::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.code == 0 && self.comments == 0 && self.blanks == 0 && self.lines == 0
    }

    pub fn is_blank(&self) -> bool {
        self.line_comment == "" && self.multi_line == ""
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let total = if self.total == 0 {
            self.files.len()
        } else {
            self.total
        };
        write!(f,
               " {: <18} {: >6} {:>12} {:>12} {:>12} {:>12}",
               "CHANGE",
               total,
               self.lines,
               self.blanks,
               self.comments,
               self.code)
    }
}

// Adding languages to the raw total.
impl<'a> AddAssign<&'a Language> for Language {
    fn add_assign(&mut self, rhs: &Self) {
        self.total += rhs.files.len();
        self.lines += rhs.lines;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
        self.code += rhs.code;
    }
}

// Adding languages to the raw total.
impl<'a> AddAssign<&'a mut Language> for Language {
    fn add_assign(&mut self, rhs: &mut Self) {
        self.total += rhs.files.len();
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
    }
}


#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum LanguageName {
    ActionScript,
    Assembly,
    Bash,
    Batch,
    C,
    CHeader,
    CSharp,
    CShell,
    Clojure,
    CoffeeScript,
    ColdFusion,
    ColdFusionScript,
    Cpp,
    CppHeader,
    Css,
    D,
    Dart,
    DeviceTree,
    Lisp,
    FortranLegacy,
    FortranModern,
    Go,
    Haskell,
    Html,
    Jai,
    Java,
    JavaScript,
    Julia,
    Json,
    Jsx,
    Less,
    LinkerScript,
    Lua,
    Makefile,
    Markdown,
    Mustache,
    ObjectiveC,
    ObjectiveCpp,
    OCaml,
    Php,
    Pascal,
    Polly,
    Perl,
    Protobuf,
    Python,
    R,
    Ruby,
    RubyHtml,
    Rust,
    Sass,
    Scala,
    Sml,
    Sql,
    Swift,
    Tex,
    Text,
    Toml,
    TypeScript,
    VimScript,
    Xml,
    Yaml,
    Zsh,
}

impl LanguageName {
    fn name(&self) -> &'static str {
        use self::LanguageName::*;

        match *self {
            ActionScript => "ActionScript",
            Assembly => "Assembly",
            Bash => "BASH",
            Batch => "Batch",
            C => "C",
            CHeader => "C Header",
            CSharp => "C#",
            CShell => "C Shell",
            Clojure => "Clojure",
            CoffeeScript => "CoffeeScript",
            ColdFusion => "ColdFusion",
            ColdFusionScript => "ColdFusion CFScript",
            Cpp => "C++",
            CppHeader => "C++ Header",
            Css => "CSS",
            D => "D",
            Dart => "Dart",
            DeviceTree => "Device Tree",
            Lisp => "LISP",
            FortranLegacy => "FORTRAN Legacy",
            FortranModern => "FORTRAN Modern",
            Go => "Go",
            Haskell => "Haskell",
            Html => "HTML",
            Jai => "JAI",
            Java => "Java",
            JavaScript => "JavaScript",
            Julia => "Julia",
            Json => "JSON",
            Jsx => "JSX",
            Less => "LESS",
            LinkerScript => "LD Script",
            Lua => "Lua",
            Makefile => "Makefile",
            Markdown => "Markdown",
            Mustache => "Mustache",
            ObjectiveC => "Objective C",
            ObjectiveCpp => "Objective C++",
            OCaml => "OCaml",
            Php => "PHP",
            Pascal => "Pascal",
            Polly => "Polly",
            Perl => "Perl",
            Protobuf => "Protocol Buffers",
            Python => "Python",
            R => "R",
            Ruby => "Ruby",
            RubyHtml => "Ruby HTML",
            Rust => "Rust",
            Sass => "Sass",
            Scala => "Scala",
            Sml => "Standard ML",
            Sql => "SQL",
            Swift => "Swift",
            Tex => "TeX",
            Text => "Plain Text",
            Toml => "TOML",
            TypeScript => "TypeScript",
            VimScript => "Vim Script",
            Xml => "XML",
            Yaml => "YAML",
            Zsh => "Zsh",
        }
    }
}

impl fmt::Display for LanguageName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
