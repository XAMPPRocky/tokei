// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the APACHE2.0/MIT licence that can be
// found in the LICENCE-{APACHE/MIT} file.


use std::borrow::Cow;
use std::fmt;
use std::path::Path;

use utils::*;
use self::LanguageType::*;

serializable_enum! {
    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub enum LanguageType {
        /// ActionScript
        ActionScript,
        /// Assembly
        Assembly,
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
        /// FortranLegacy
        FortranLegacy,
        /// FortranModern
        FortranModern,
        /// Go
        Go,
        /// Haskell
        Haskell,
        /// Html
        Html,
        /// Idris
        Idris,
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
        /// Ruby
        Ruby,
        /// RubyHtml
        RubyHtml,
        /// Rust
        Rust,
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
        /// __Total
        __Total,
    }
    LanguageTypeVisitor
}

#[derive(Debug)]
pub enum Error {
    Parse(String),
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl_as_ref_from_str! {
    LanguageType {
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
    Error::Parse
}


impl LanguageType {
    pub fn name(&self) -> &'static str {
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

    pub fn from_extension<P: AsRef<Path>>(entry: P) -> Option<Self> {
        if let Some(extension) = get_extension(entry) {
            match &*extension {
                "as" => Some(ActionScript),
                "bash" | "sh" => Some(Bash),
                "bat" | "btm" | "cmd" => Some(Batch),
                "c" | "ec" | "pgc" => Some(C),
                "cc" | "cpp" | "cxx" | "c++" | "pcc" => Some(Cpp),
                "cfc" => Some(ColdFusionScript),
                "cfm" => Some(ColdFusion),
                "clj" => Some(Clojure),
                "coffee" => Some(CoffeeScript),
                "cs" => Some(CSharp),
                "csh" => Some(CShell),
                "css" => Some(Css),
                "d" => Some(D),
                "dart" => Some(Dart),
                "dts" | "dtsi" => Some(DeviceTree),
                "el" | "lisp" | "lsp" => Some(Lisp),
                "erl" | "hrl" => Some(Erlang),
                "f" | "for" | "ftn" | "f77" | "pfo" => Some(FortranLegacy),
                "f03" | "f08" | "f90" | "f95" => Some(FortranModern),
                "go" => Some(Go),
                "h" => Some(CHeader),
                "hh" | "hpp" | "hxx" => Some(CppHeader),
                "hs" => Some(Haskell),
                "html" => Some(Html),
                "idr" | "lidr" => Some(Idris),
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
                "rs" | "in" => Some(Rust),
                "s" => Some(Assembly),
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
                _ => None,
            }
        } else {
            None
        }
    }
}

impl From<String> for LanguageType {
    fn from(from: String) -> Self {
        match &*from {
            "ActionScript" => ActionScript,
            "Assembly" => Assembly,
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
            "FortranLegacy" => FortranLegacy,
            "FortranModern" => FortranModern,
            "Go" => Go,
            "Haskell" => Haskell,
            "Html" => Html,
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
            "Ruby" => Ruby,
            "RubyHtml" => RubyHtml,
            "Rust" => Rust,
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
