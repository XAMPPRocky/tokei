# Tokei ([時計](https://en.wiktionary.org/wiki/%E6%99%82%E8%A8%88))
[![Mean Bean CI](https://github.com/XAMPPRocky/tokei/workflows/Mean%20Bean%20CI/badge.svg)](https://github.com/XAMPPRocky/tokei/actions?query=workflow%3A%22Mean+Bean+CI%22)
[![crates.io](https://img.shields.io/crates/d/tokei.svg)](https://crates.io/crates/tokei)
[![Help Wanted](https://img.shields.io/github/issues/XAMPPRocky/tokei/help%20wanted?color=green)](https://github.com/XAMPPRocky/tokei/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
[![Lines Of Code](https://tokei.rs/b1/github/XAMPPRocky/tokei?category=code)](https://github.com/XAMPPRocky/tokei)
[![Documentation](https://docs.rs/tokei/badge.svg)](https://docs.rs/tokei/)

Tokei is a program that displays statistics about your code. Tokei will show number of files, total lines within those files and code, comments, and blanks grouped by language.

### Translations
- [中文](https://github.com/chinanf-boy/tokei-zh#支持的语言)

## Example
```terminal
-------------------------------------------------------------------------------
 Language            Files        Lines         Code     Comments       Blanks
-------------------------------------------------------------------------------
 Dockerfile              1           64           36           14           14
 JSON                    1         1919         1919            0            0
 Markdown                5         1952         1952            0            0
 Rust                   19         2922         2093          412          417
 Shell                   4          138          102            8           28
 TOML                    1           78           66            0           12
 YAML                    1           80           60            8           12
-------------------------------------------------------------------------------
 Total                  32         7153         6228          442          483
-------------------------------------------------------------------------------
```

## [Documentation](https://docs.rs/tokei)

## Table of Contents

- [Features](#features)
- [Installation](#installation)
    - [Automatic](#automatic)
        - [Arch Linux](#arch-linux)
        - [Cargo](#cargo)
        - [Conda](#conda)
        - [Fedora](#fedora)
        - [FreeBSD](#freebsd)
        - [Homebrew](#homebrew)
        - [Nix/NixOS](#nixnixos)
    - [Manual](#manual)
- [How to use Tokei](#how-to-use-tokei)
- [Options](#options)
- [Badges](#badges)
- [Plugins](#plugins)
- [Supported Languages](#supported-languages)
- [Changelog](CHANGELOG.md)
- [Common Issues](#common-issues)
- [Canonical Source](#canonical-source)
- [Copyright and License](#copyright-and-license)

## Features

- Tokei is **very fast**, check out our [comparison](./COMPARISON.md) document
  to see how Tokei's speed compares to others.

- Tokei is **accurate**, Tokei correctly handles multi line comments,
  nested comments, and not counting comments that are in strings. Providing an
  accurate code statistics.

- Tokei has huge range of languages, supporting over **150** languages, and
  their various extensions.

- Tokei can output in multiple formats(**CBOR**, **JSON**, **TOML**, **YAML**)
  allowing Tokei's output to be easily stored, and reused. These can also be
  reused in tokei combining a previous run's statistics with another set.

- Tokei is available on **Mac**, **Linux**, and **Windows**. See [installation
  instructions](#installation) for how to get Tokei on your platform.

- Tokei is also a **library** allowing you to easily integrate it with other
  projects.

## Installation

### Automatic

#### Arch Linux
```shell
$ pacman -S tokei
```

#### Cargo
```shell
$ cargo install tokei
```

#### Conda
```shell
$ conda install -c conda-forge tokei
```

#### Fedora
```shell
$ sudo dnf install tokei
```

#### OpenSUSE
```shell
$ sudo zypper install tokei
```

#### FreeBSD
```shell
$ pkg install tokei
```

#### MacOS (Homebrew)
```shell
$ brew install tokei
```

#### Nix/NixOS
```shell
$ nix-env -i tokei
```

### In a container

Launch the Docker container by supplying the path as a volume in read only mode:

```shell
$ docker run -v ~/Development/code/myproject/foo:/data:ro mbologna/docker-tokei
```
or
```shell
$ docker run -v ~/Development/code/myproject/foo:/data:ro mbologna/docker-tokei tokei --sort lines
```

### Manual
You can download prebuilt binaries in the
[releases section](https://github.com/XAMPPRocky/tokei/releases), or create
from source.
```shell
$ git clone https://github.com/XAMPPRocky/tokei.git
$ cd tokei
$ cargo build --release
```
##### Linux
```
# sudo mv target/release/tokei /usr/local/bin
```
##### OSX
```
# sudo mv target/release/tokei /usr/local/bin/tokei
```
##### Windows
- Create a folder for tokei
- search for `env`
- open "edit your environment variables"
- edit `PATH`
- append folder path to the end of the string ie: `<path_stuff_here>;C:/tokei/;`

## How to use Tokei

#### Basic usage

This is the basic way to use tokei. Which will report on the code in `./foo`
and all subfolders.

```shell
$ tokei ./foo
```

#### Multiple folders
To have tokei report on multiple folders in the same call simply add a comma,
or a space followed by another path.

```shell
$ tokei ./foo ./bar ./baz
```
```shell
$ tokei ./foo, ./bar, ./baz
```

#### Excluding folders
Tokei will respect all `.gitignore` and `.ignore` files, and you can use
the `--exclude` option to exclude any additional files. The `--exclude` flag has
the same semantics as `.gitignore`.

```shell
$ tokei ./foo --exclude *.rs
```

#### Sorting output
By default tokei sorts alphabetically by language name, however using `--sort`
tokei can also sort by any of the columns.

`blanks, code, comments, lines`

```shell
$ tokei ./foo --sort code
```

#### Outputting file statistics
By default tokei only outputs the total of the languages, and using `--files`
flag tokei can also output individual file statistics.

```shell
$ tokei ./foo --files
```

#### Outputting into different formats
Tokei normally outputs into a nice human readable format designed for terminals.
There is also using the `--output` option various other formats that are more
useful for bringing the data into another program.

**Note:** This version of tokei was compiled without any serialization formats, to enable serialization, reinstall
tokei with the features flag.

```shell
  ALL:
  cargo install tokei --features all

  JSON:
  cargo install tokei --features json

  CBOR:
  cargo install tokei --features cbor

  YAML:
  cargo install tokei --features yaml

  TOML:
  cargo install tokei --features toml
```

**Currently supported formats**
- JSON `--output json`
- YAML `--output yaml`
- TOML `--output toml`
- CBOR `--output cbor`

```shell
$ tokei ./foo --output json
```

#### Reading in stored formats
Tokei can also take in the outputted formats added in the previous results to it's
current run. Tokei can take either a path to a file, the format passed in as a
value to the option, or from stdin.

```shell
$ tokei ./foo --input ./stats.json
```

## Options

```
USAGE:
    tokei [FLAGS] [OPTIONS] [--] [input]...

FLAGS:
    -f, --files               Will print out statistics on individual files.
    -h, --help                Prints help information
        --hidden              Count hidden files.
    -l, --languages           Prints out supported languages and their extensions.
        --no-ignore           Don't respect ignore files.
        --no-ignore-parent    Don't respect ignore files in parent directories.
        --no-ignore-vcs       Don't respect VCS ignore files.
    -V, --version             Prints version information
    -v, --verbose             Set log output level:
                                          1: to show unknown file extensions,
                                          2: reserved for future debugging,
                                          3: enable file level trace. Not recommended on multiple files

OPTIONS:
    -c, --columns <columns>       Sets a strict column width of the output, only available for terminal output.
    -e, --exclude <exclude>...    Ignore all files & directories containing the word.
    -i, --input <file_input>      Gives statistics from a previous tokei run. Can be given a file path, or "stdin" to
                                  read from stdin.
    -o, --output <output>         Outputs Tokei in a specific format. Compile with additional features for more format
                                  support. [possible values: cbor, json, yaml]
    -s, --sort <sort>             Sort languages based on column [possible values: files, lines, blanks, code, comments]
    -t, --type <types>            Filters output by language type, seperated by a comma. i.e. -t=Rust,Markdown

ARGS:
    <input>...    The input file(s)/directory(ies) to be counted.
```

## Badges
Tokei has support for badges. For example
[![](https://tokei.rs/b1/github/XAMPPRocky/tokei)](https://github.com/XAMPPRocky/tokei).

```
[![](https://tokei.rs/b1/github/XAMPPRocky/tokei)](https://github.com/XAMPPRocky/tokei).
```

Tokei's URL scheme is as follows.

```
https://tokei.rs/b1/{host: values: github|gitlab}/{Repo Owner eg: XAMPPRocky}/{Repo name eg: tokei}
```

By default the badge will show the repo's LoC(_Lines of Code_), you can also
specify for it to show a different category, by using the `?category=` query
string. It can be either `code`, `blanks`, `files`, `lines`, `comments`,
Example show total lines:

```
[![](https://tokei.rs/b1/github/XAMPPRocky/tokei?category=lines)](https://github.com/XAMPPRocky/tokei).
```

The server code hosted on tokei.rs is in [XAMPPRocky/tokei_rs](https://github.com/XAMPPRocky/tokei_rs)

## Plugins
Thanks to contributors tokei is now available as a plugin for some text editors.

- [Vim](https://github.com/vmchale/tokei-vim) by [vmchale](https://github.com/vmchale/)

## Supported Languages

If there is a language that you want added, feel free to submit a pull request
with the following information. If you're unsure have a look at
[`languages.json`](./languages.json) to see how other languages are defined.

- Name of language
- File Extension(s)
- The comment syntax (_Does it have block comments? is it the same as C?_)
- The string literal syntax

```
ABAP
ActionScript
Ada
Agda
Alex
ASP
ASP.NET
Assembly
AssemblyGAS
AutoHotKey
Autoconf
Automake
BASH
Batch
BrightScript
C
C Header
CMake
C#
C Shell
Cabal
Cassius
Ceylon
Clojure
ClojureC
ClojureScript
COBOL
CoffeeScript
Cogent
ColdFusion
ColdFusion CFScript
Coq
C++
C++ Header
Crystal
CSS
D
Dart
Device Tree
Dockerfile
.NET Resource
Dream Maker
Edn
Emacs Lisp
Elixir
Elm
Elvish
Emacs Dev Env
Erlang
FEN
F#
Fish
Forth
FORTRAN Legacy
FORTRAN Modern
F*
GDScript
Gherkin (Cucumber)
GLSL
Go
Groovy
Hamlet
Handlebars
Happy
Haskell
Haxe
HCL
HEX
HLSL
HolyC
HTML
Idris
INI
Intel HEX
Isabelle
JAI
Java
JavaScript
JSON
JSX
Julia
Julius
Kakoune script
Kotlin
Lean
LESS
LD Script
Liquid
Lisp
Logtalk
Lua
Lucius
Madlang
Makefile
Markdown
Meson
Mint
Module-Definition
MSBuild
Mustache
Nim
Nix
Not Quite Perl
OCaml
Objective-C
Objective-C++
Org
Oz
PSL Assertion
Pascal
Perl
Perl6
PHP
Polly
Processing
Prolog
Protocol Buffers
PureScript
Python
QCL
QML
R
Racket
Rakefile
Razor
ReStructuredText
Ruby
Ruby HTML
Rust
SRecode Template
Sass
Scala
Scheme
Scons
Shell
Standard ML (SML)
Solidity
Specman e
Spice Netlist
SQL
SVG
Swift
SWIG
SystemVerilog
TCL
TeX
Plain Text
TOML
Twig
TypeScript
Unreal Markdown
Unreal Plugin
Unreal Project
Unreal Script
Unreal Shader
Unreal Shader Header
Ur/Web
Ur/Web Project
VB6
VBScript
Vala
Verilog
Verilog Args File
VHDL
Vim Script
Visual Basic
Visual Studio Project
Visual Studio Solution
Vue
Wolfram
XSL
XAML
Xcode Config
XML
Xtend
YAML
Zig
Zsh
```

## Common issues

### Tokei says I have a lot of D code, but I know there is no D code!
This is likely due to `gcc` generating `.d` files. Until the D people decide on
a different file extension, you can always exclude `.d` files using the
`-e --exclude` flag like so

```
$ tokei . -e *.d
```

## Canonical Source
The canonical source of this repo is hosted on
[GitHub](https://github.com/XAMPPRocky/tokei). If you have a GitHub account,
please make your issues, and pull requests there.

## Copyright and License
(C) Copyright 2015 by XAMPPRocky and contributors

See [the graph](https://github.com/XAMPPRocky/tokei/graphs/contributors) for a full list of contributors.

Tokei is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENCE-APACHE](./LICENCE-APACHE), [LICENCE-MIT](./LICENCE-MIT) for more information.
