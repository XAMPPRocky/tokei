// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE - MIT} file.

use std::fmt;
use std::path::PathBuf;
use std::ops::AddAssign;

use consts::*;
use stats::Stats;

#[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Language {
    pub blanks: usize,
    pub code: usize,
    pub comments: usize,
    pub files: Vec<PathBuf>,
    pub stats: Vec<Stats>,
    pub lines: usize,
    pub line_comment: &'static str,
    pub multi_line: &'static str,
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
        Language { ..Self::default() }
    }

    pub fn new_func() -> Self {
        Language {
            multi_line: "(*",
            multi_line_end: "*)",
            ..Self::default()
        }
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
                 self.blanks,
                 self.comments,
                 self.code)
    }
}


// Adding languages to the raw total_files.
impl<'a> AddAssign<&'a Language> for Language {
    fn add_assign(&mut self, rhs: &Self) {
        self.total_files += rhs.total_files;
        self.lines += rhs.lines;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
        self.code += rhs.code;
    }
}

// Adding languages to the raw total_files.
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


#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum LanguageName {
    ActionScript,
    Assembly,
    Bash,
    Batch,
    C,
    CHeader,
    Clojure,
    CoffeeScript,
    ColdFusion,
    ColdFusionScript,
    Coq,
    Cpp,
    CppHeader,
    CSharp,
    CShell,
    Css,
    D,
    Dart,
    DeviceTree,
    Erlang,
    FortranLegacy,
    FortranModern,
    Go,
    Haskell,
    Html,
    Idris,
    Jai,
    Java,
    JavaScript,
    Julia,
    Json,
    Jsx,
    Kotlin,
    Less,
    LinkerScript,
    Lisp,
    Lua,
    Makefile,
    Markdown,
    Mustache,
    Nim,
    ObjectiveC,
    ObjectiveCpp,
    OCaml,
    Oz,
    Pascal,
    Perl,
    Polly,
    Php,
    Protobuf,
    Prolog,
    Python,
    Qcl,
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
    UnrealScript,
    Wolfram,
    Xml,
    Yaml,
    Zsh,
    __Total,
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
            Clojure => "Clojure",
            CoffeeScript => "CoffeeScript",
            ColdFusion => "ColdFusion",
            ColdFusionScript => "ColdFusion CFScript",
            Coq => "Coq",
            Cpp => "C++",
            CppHeader => "C++ Header",
            CSharp => "C#",
            CShell => "C Shell",
            Css => "CSS",
            D => "D",
            Dart => "Dart",
            DeviceTree => "Device Tree",
            Erlang => "Erlang",
            FortranLegacy => "FORTRAN Legacy",
            FortranModern => "FORTRAN Modern",
            Go => "Go",
            Haskell => "Haskell",
            Html => "HTML",
            Idris => "Idris",
            Jai => "JAI",
            Java => "Java",
            JavaScript => "JavaScript",
            Json => "JSON",
            Jsx => "JSX",
            Julia => "Julia",
            Kotlin => "Kotlin",
            Less => "LESS",
            LinkerScript => "LD Script",
            Lisp => "LISP",
            Lua => "Lua",
            Makefile => "Makefile",
            Markdown => "Markdown",
            Mustache => "Mustache",
            Nim => "Nim",
            ObjectiveC => "Objective C",
            ObjectiveCpp => "Objective C++",
            OCaml => "OCaml",
            Oz => "Oz",
            Pascal => "Pascal",
            Perl => "Perl",
            Polly => "Polly",
            Php => "PHP",
            Protobuf => "Protocol Buffers",
            Prolog => "Prolog",
            Python => "Python",
            Qcl => "QCL",
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
            UnrealScript => "Unreal Script",
            VimScript => "Vim Script",
            Wolfram => "Wolfram",
            Xml => "XML",
            Yaml => "YAML",
            Zsh => "Zsh",
            __Total => "Total",
        }
    }
}

impl fmt::Display for LanguageName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
