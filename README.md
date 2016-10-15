# Tokei ([時計](https://en.wiktionary.org/wiki/%E6%99%82%E8%A8%88))

[![](https://img.shields.io/travis/Aaronepower/tokei.svg)](https://travis-ci.org/Aaronepower/tokei)
[![](https://img.shields.io/crates/d/tokei.svg)](https://crates.io/crates/tokei)
[![](https://img.shields.io/github/issues-raw/Aaronepower/tokei.svg)](http://github.com/Aaronepower/tokei/issues)

Tokei is a program that allows you to count code, quickly.

## [Documentation](https://crates.fyi/crates/tokei/3.0.0/index.html)

## Table of Contents

- [Canonical Source](#canonical-source)
- [Installation](#installation)
- [How to use Tokei](#how-to-use-tokei)
- [Options](#options)
- [Supported Languages](#supported-languages)
- [Changelog](CHANGELOG.md)
- [Common Issues](#common-issues)
- [Copyright](#copyright)


## Canonical Source
The canonical source of this repo is hosted on [GitHub](https://github.com/Aaronepower/tokei). If you have a GitHub account, please make your issues, and pull requests there.

## Installation

### Automatic
If you have [`cargo 0.6.0>=`](https://www.rust-lang.org/downloads.html) installed just run the `cargo install` command.

```shell
$ cargo install tokei
```

### Manual

#### Fedora 64 bit
Install rust and cargo from either the [official page](https://www.rust-lang.org) or use a copr repo such as [Rust](https://copr.fedoraproject.org/coprs/phnxrbrn/rust/)
```shell
$ dnf copr enable phnxrbrn/tokei
$ dnf install tokei
```

#### Other
```shell
$ git clone https://github.com/Aaronepower/tokei.git
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
- open "edit your enviroment variables"
- edit `PATH`
- append folder path to the end of the string ie: `<path_stuff_here>;C:/tokei/;`

## How to use Tokei

#### Basic usage

This is the basic way to use tokei. Which will report on the code in `./foo` and all subfolders.

```shell
$ tokei ./foo
```

#### Multiple folders
To have tokei report on multiple folders in the same call simply add a comma, or a space followed by another path.

```shell
$ tokei ./foo ./bar ./baz
```
```shell
$ tokei ./foo, ./bar, ./baz
```

#### Excluding folders
The `--exclude` option accepts a comma-separated list of strings. Any file or directory containing a term will be ignored:

```shell
$ tokei ./foo --exclude node_modules,.cache,tmp target
```

#### Sorting output
By default tokei sorts alphabetically by language name, however using `--sort` tokei can also sort by any of the columns.
`blanks, code, comments, lines`

```shell
$ tokei ./foo --sort code
```

#### Outputing file statistics
By default tokei only outputs the total of the languages, and using `--files` flag tokei can also output individual file statistics.

```shell
$ tokei ./foo --files
```

#### Outputting into different formats
Tokei normally outputs into a nice human readable format designed for the terminal. 
There is also using the `--output` option various other formats that are more useful for bringing the data into another program.

**Current supported formats**
- JSON `--output json`
- YAML `--output yaml`
- TOML `--output toml`
- CBOR `--output cbor`

```shell
$ tokei ./foo --output json
```

#### Reading in stored formats
Tokei can also take in the outputted formats added the previous results to it's current run.
Tokei can take either a path to a file, the format passed in as a value to the option, or from stdin.

```shell
$ tokei ./foo --input ./stats.json
```

## Options
```
Tokei 4.3.0
Aaron P. <theaaronepower@gmail.com>
Count Code, Quickly.

USAGE:
    Tokei [FLAGS] [OPTIONS] <input>...

FLAGS:
    -f, --files        Will print out statistics on individual files.
    -h, --help         Prints help information
    -l, --languages    Prints out supported languages and their extensions.
    -V, --version      Prints version information
    -v                 Set verbose output level: 1 for File IO errors 2: for unknown extensions

OPTIONS:
    -e, --exclude <exclude>     Ignore all files & directories containing the word.
    -i, --input <file_input>    Gives statistics from a previous tokei run. Can be given a file path, or "stdin" to
                                read from stdin.
    -o, --output <output>       Outputs Tokei in a specific format. [values: cbor, json, toml, yaml]
    -s, --sort <sort>           Will sort based on column [values: files, lines, blanks, code, comments]

ARGS:
    <input>...    The input file(s)/directory(ies)
```

## Supported Languages

If there is a language that you want added submit a pull request with the following information

- Name of language
- File Extension
- The comment syntax (_Does it have block comments? is it the same as C?_)

```
ActionScript
Ada
Assembly
ASP
ASP.Net
Autoconf
BASH
Batch
C
C Header
Clojure
CoffeeScript
ColdFusion
ColdFusion CFScript
Coq
C++
C++ Header
C#
C Shell
CSS
D
Dart
Device Tree
Erlang
Forth
FORTRAN Legacy
FORTRAN Modern
GLSL
Go
Handlebars
Haskell
HTML
HEX
Idris
Intel HEX
Isabelle
JAI
Java
JavaScript
Julia
JSON
JSX
Kotlin
Lean
LESS
LD Script
LISP
Lua
Makefile
Markdown
Mustache
Nim
Objective C
Objective C++
OCaml
Oz
Pascal
Perl
Polly
PHP
Protocol Buffers
Prolog
Python
QCL
R
Razor
Ruby
Ruby HTML
Rust
ReStructuredText
Sass
Scala
Standard ML
SQL
Swift
TeX
Plain Text
TOML
TypeScript
Vim Script
Unreal Script
Wolfram
XML
YAML
Zsh
```

## Common issues

### Tokei says I have a lot of D code, but I know there is no D code!
This is likely due to `gcc` generating `.d` files. Until the D people decide on a different file extension, you can always exclude `.d` files using the `-e --exclude` flag like so

```
$ tokei . -e .d
```

## Copyright and License
(C) Copyright 2015 by Aaron Power and contributors

See CONTRIBUTORS.md for a full list of contributors.

Tokei is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENCE-APACHE](./LICENCE-APACHE), [LICENCE-MIT](./LICENCE-MIT) for more information.
