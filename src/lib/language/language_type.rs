// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the APACHE2.0/MIT licence that can be
// found in the LICENCE-{APACHE/MIT} file.


use std::borrow::Cow;
use std::fmt;
use std::path::Path;

use utils::fs;
use self::LanguageType::*;

#[cfg_attr(feature = "io", derive(Deserialize, Serialize))]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LanguageType {
    /// ActionScript
    ActionScript,
    /// Ada
    Ada,
    /// Assembly
    Assembly,
    /// ASP
    Asp,
    /// ASP.Net
    AspNet,
    /// Autoconf
    Autoconf,
    /// Bash
    Bash,
    /// Batch
    Batch,
    /// C
    C,
    /// CHeader
    CHeader,
    /// Clojure
    Clojure,
    /// CoffeeScript
    CoffeeScript,
    /// ColdFusion
    ColdFusion,
    /// ColdFusionScript
    ColdFusionScript,
    /// Coq
    Coq,
    /// Cpp
    Cpp,
    /// CppHeader
    CppHeader,
    /// CSharp
    CSharp,
    /// CShell
    CShell,
    /// Css
    Css,
    /// D
    D,
    /// Dart
    Dart,
    /// DeviceTree
    DeviceTree,
    /// Erlang
    Erlang,
    /// Forth
    Forth,
    /// FortranLegacy
    FortranLegacy,
    /// FortranModern
    FortranModern,
    /// Go
    Go,
    /// Handlebars
    Handlebars,
    /// Haskell
    Haskell,
    /// Html
    Html,
    /// HEX
    Hex,
    /// Idris
    Idris,
    /// Intel HEX
    IntelHex,
    /// Isabelle
    Isabelle,
    /// Jai
    Jai,
    /// Java
    Java,
    /// JavaScript
    JavaScript,
    /// Julia
    Julia,
    /// Json
    Json,
    /// Jsx
    Jsx,
    /// Kotlin
    Kotlin,
    /// Less
    Less,
    /// LinkerScript
    LinkerScript,
    /// Lisp
    Lisp,
    /// Lua
    Lua,
    /// Makefile
    Makefile,
    /// Markdown
    Markdown,
    /// Mustache
    Mustache,
    /// Nim
    Nim,
    /// ObjectiveC
    ObjectiveC,
    /// ObjectiveCpp
    ObjectiveCpp,
    /// OCaml
    OCaml,
    /// Oz
    Oz,
    /// Pascal
    Pascal,
    /// Perl
    Perl,
    /// Polly
    Polly,
    /// Php
    Php,
    /// Protobuf
    Protobuf,
    /// Prolog
    Prolog,
    /// Python
    Python,
    /// Qcl
    Qcl,
    /// R
    R,
    /// Razor
    Razor,
    /// Ruby
    Ruby,
    /// RubyHtml
    RubyHtml,
    /// Rust
    Rust,
    /// ReStructuredText
    ReStructuredText,
    /// Sass
    Sass,
    /// Scala
    Scala,
    /// Sml
    Sml,
    /// Sql
    Sql,
    /// Swift
    Swift,
    /// Tex
    Tex,
    /// Text
    Text,
    /// Toml
    Toml,
    /// TypeScript
    TypeScript,
    /// VimScript
    VimScript,
    /// UnrealScript
    UnrealScript,
    /// Wolfram
    Wolfram,
    /// Xml
    Xml,
    /// Yaml
    Yaml,
    /// Zsh
    Zsh,
}

impl LanguageType {
    /// Returns the display name of a language.
    ///
    /// ```
    /// # use tokei::*;
    /// let bash = LanguageType::Bash;
    ///
    /// assert_eq!(bash.name(), "BASH");
    /// ```
    pub fn name(&self) -> &'static str {
        match *self {
            ActionScript => "ActionScript",
            Ada => "Ada",
            Assembly => "Assembly",
            Asp => "ASP",
            AspNet => "ASP.Net",
            Autoconf => "Autoconf",
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
            Forth => "Forth",
            FortranLegacy => "FORTRAN Legacy",
            FortranModern => "FORTRAN Modern",
            Go => "Go",
            Handlebars => "Handlebars",
            Haskell => "Haskell",
            Html => "HTML",
            Hex => "HEX",
            Idris => "Idris",
            IntelHex => "Intel HEX",
            Isabelle => "Isabelle",
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
            Razor => "Razor",
            Ruby => "Ruby",
            RubyHtml => "Ruby HTML",
            Rust => "Rust",
            ReStructuredText => "ReStructuredText",
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
        }
    }

    /// Get language from it's file extension.
    ///
    /// ```no_run
    /// # use tokei::*;
    /// let rust = LanguageType::from_extension("./main.rs");
    ///
    /// assert_eq!(rust, Some(LanguageType::Rust));
    /// ```
    pub fn from_extension<P: AsRef<Path>>(entry: P) -> Option<Self> {
        if let Some(extension) = fs::get_extension(entry) {
            match &*extension {
                "as" => Some(ActionScript),
                "ada" | "adb" | "ads" | "pad" => Some(Ada),
                "asa" | "asp" => Some(Asp),
                "asax" | "ascx"| "asmx"| "aspx"| "master"| "sitemap"| "webinfo" => Some(AspNet),
                "bash" | "sh" => Some(Bash),
                "bat" | "btm" | "cmd" => Some(Batch),
                "c" | "ec" | "pgc" => Some(C),
                "cc" | "cpp" | "cxx" | "c++" | "pcc" => Some(Cpp),
                "cfc" => Some(ColdFusionScript),
                "cfm" => Some(ColdFusion),
                "clj" => Some(Clojure),
                "coffee" => Some(CoffeeScript),
                "cs" => Some(CSharp),
                "cshtml" => Some(Razor),
                "csh" => Some(CShell),
                "css" => Some(Css),
                "d" => Some(D),
                "dart" => Some(Dart),
                "dts" | "dtsi" => Some(DeviceTree),
                "el" | "lisp" | "lsp" => Some(Lisp),
                "erl" | "hrl" => Some(Erlang),
                "4th" | "forth" | "fr" | "frt" | "fth" | "f83" | "fb" | "fpm" | "e4" | "rx" |
                "ft" => Some(Forth),
                "f" | "for" | "ftn" | "f77" | "pfo" => Some(FortranLegacy),
                "f03" | "f08" | "f90" | "f95" => Some(FortranModern),
                "go" => Some(Go),
                "h" => Some(CHeader),
                "hbs" | "handlebars" => Some(Handlebars),
                "hh" | "hpp" | "hxx" => Some(CppHeader),
                "hs" => Some(Haskell),
                "html" => Some(Html),
                "hex" => Some(Hex),
                "idr" | "lidr" => Some(Idris),
                "ihex" => Some(IntelHex),
                "in" => Some(Autoconf),
                "jai" => Some(Jai),
                "java" => Some(Java),
                "jl" => Some(Julia),
                "js" => Some(JavaScript),
                "json" => Some(Json),
                "jsx" => Some(Jsx),
                "kt" | "kts" => Some(Kotlin),
                "lds" => Some(LinkerScript),
                "less" => Some(Less),
                "lua" => Some(Lua),
                "m" => Some(ObjectiveC),
                "markdown" | "md" => Some(Markdown),
                "ml" | "mli" => Some(OCaml),
                "mm" => Some(ObjectiveCpp),
                "makefile" => Some(Makefile),
                "mustache" => Some(Mustache),
                "nim" => Some(Nim),
                "nb" | "wl" => Some(Wolfram),
                "oz" => Some(Oz),
                "p" | "pro" => Some(Prolog),
                "pas" => Some(Pascal),
                "php" => Some(Php),
                "pl" => Some(Perl),
                "qcl" => Some(Qcl),
                "text" | "txt" => Some(Text),
                "polly" => Some(Polly),
                "proto" => Some(Protobuf),
                "py" => Some(Python),
                "r" => Some(R),
                "rake" | "rb" => Some(Ruby),
                "rhtml" => Some(RubyHtml),
                "rs" => Some(Rust),
                "rst" => Some(ReStructuredText),
                "s" | "asm" => Some(Assembly),
                "sass" | "scss" => Some(Sass),
                "sc" | "scala" => Some(Scala),
                "sml" => Some(Sml),
                "sql" => Some(Sql),
                "swift" => Some(Swift),
                "tex" | "sty" => Some(Tex),
                "toml" => Some(Toml),
                "ts" => Some(TypeScript),
                "thy" => Some(Isabelle),
                "uc" | "uci" | "upkg" => Some(UnrealScript),
                "v" => Some(Coq),
                "vim" => Some(VimScript),
                "xml" => Some(Xml),
                "yaml" | "yml" => Some(Yaml),
                "zsh" => Some(Zsh),
                extension => {
                    warn!("Unknown extension: {}", extension);
                    None
                },
            }
        } else {
            None
        }
    }
}

impl From<String> for LanguageType {
    fn from(from: String) -> Self {
        LanguageType::from(&*from)
    }
}

impl<'a> From<&'a str> for LanguageType {
    fn from(from: &str) -> Self {
        match &*from {
            "ActionScript" => ActionScript,
            "Ada" => Ada,
            "Assembly" => Assembly,
            "ASP" => Asp,
            "ASP.Net" => AspNet,
            "Autoconf" => Autoconf,
            "Bash" => Bash,
            "Batch" => Batch,
            "C" => C,
            "CHeader" => CHeader,
            "Clojure" => Clojure,
            "CoffeeScript" => CoffeeScript,
            "ColdFusion" => ColdFusion,
            "ColdFusionScript" => ColdFusionScript,
            "Coq" => Coq,
            "Cpp" => Cpp,
            "CppHeader" => CppHeader,
            "CSharp" => CSharp,
            "CShell" => CShell,
            "Css" => Css,
            "D" => D,
            "Dart" => Dart,
            "DeviceTree" => DeviceTree,
            "Erlang" => Erlang,
            "Forth" => Forth,
            "FortranLegacy" => FortranLegacy,
            "FortranModern" => FortranModern,
            "Go" => Go,
            "Handlebars" => Handlebars,
            "Haskell" => Haskell,
            "Html" => Html,
            "HEX" => Hex,
            "Intel HEX" => IntelHex,
            "Idris" => Idris,
            "Jai" => Jai,
            "Java" => Java,
            "JavaScript" => JavaScript,
            "Julia" => Julia,
            "Json" => Json,
            "Jsx" => Jsx,
            "Kotlin" => Kotlin,
            "Less" => Less,
            "LinkerScript" => LinkerScript,
            "Lisp" => Lisp,
            "Lua" => Lua,
            "Makefile" => Makefile,
            "Markdown" => Markdown,
            "Mustache" => Mustache,
            "Nim" => Nim,
            "ObjectiveC" => ObjectiveC,
            "ObjectiveCpp" => ObjectiveCpp,
            "OCaml" => OCaml,
            "Oz" => Oz,
            "Pascal" => Pascal,
            "Perl" => Perl,
            "Polly" => Polly,
            "Php" => Php,
            "Protobuf" => Protobuf,
            "Prolog" => Prolog,
            "Python" => Python,
            "Qcl" => Qcl,
            "R" => R,
            "Razor" => Razor,
            "Ruby" => Ruby,
            "RubyHtml" => RubyHtml,
            "Rust" => Rust,
            "ReStructuredText" => ReStructuredText,
            "Sass" => Sass,
            "Scala" => Scala,
            "Sml" => Sml,
            "Sql" => Sql,
            "Swift" => Swift,
            "Tex" => Tex,
            "Text" => Text,
            "Toml" => Toml,
            "TypeScript" => TypeScript,
            "VimScript" => VimScript,
            "UnrealScript" => UnrealScript,
            "Wolfram" => Wolfram,
            "Xml" => Xml,
            "Yaml" => Yaml,
            "Zsh" => Zsh,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for LanguageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}


impl<'a> From<LanguageType> for Cow<'a, LanguageType> {
    fn from(from: LanguageType) -> Self {
        Cow::Owned(from)
    }
}

impl<'a> From<&'a LanguageType> for Cow<'a, LanguageType> {
    fn from(from: &'a LanguageType) -> Self {
        Cow::Borrowed(from)
    }
}
