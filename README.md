# Tokei ([時計](https://en.wiktionary.org/wiki/%E6%99%82%E8%A8%88))

[![GitHub License](https://img.shields.io/github/license/Aaronepower/tokei.svg)](http://github.com/Aaronepower/tokei/blob/master/LICENSE)
[![](https://img.shields.io/github/release/Aaronepower/tokei.svg)](https://github.com/Aaronepower/tokei/releases/tag/1.1.1/)
[![](https://img.shields.io/travis/Aaronepower/tokei.svg)](https://travis-ci.org/Aaronepower/tokei)
[![](https://img.shields.io/github/downloads/Aaronepower/tokei/latest/total.svg)](https://github.com/Aaronepower/tokei/releases/)
[![](https://img.shields.io/github/issues-raw/Aaronepower/tokei.svg)](http://github.com/Aaronepower/tokei/issues)

A blazingly fast CLOC(Count Lines Of Code) program, written in Rust.

## Canonical Source
The canonical source of this repo is hosted on [GitLab](https://gitlab.com/Aaronepower/tokei). If you have a GitLab account, please make your issues, and pull requests there. However if you don't have one, please feel free to make the issue on [GitHub](https://github.com/Aaronepower/tokei).

## Installation

### Automatic
If you have [`cargo 0.6.0>=`](https://www.rust-lang.org/downloads.html) installed just run the `cargo install` command.

```bash
$ cargo install tokei
```

### Manual

#### Fedora 64 bit
Install rust and cargo from either the [official page](https://www.rust-lang.org) or use a copr repo such as [Rust](https://copr.fedoraproject.org/coprs/phnxrbrn/rust/)
```bash
$ dnf copr enable phnxrbrn/tokei
$ dnf install tokei
```

#### Other
```bash
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

## Usage

To use tokei, use must add it to your path. Then you can call tokei like so

```bash
$ tokei ./path/to/code
```

## Options
```
Tokei 1.3.0
Aaron P. <theaaronepower@gmail.com>
A quick CLOC (Count Lines Of Code) tool

USAGE:
        Tokei [FLAGS] [OPTIONS] <input>... [--]

FLAGS:
    -f, --files        Will print out the files found only recommended for debugging purposes
    -h, --help         Prints help information
    -l, --languages    prints out supported languages and their extensions
    -V, --version      Prints version information

OPTIONS:
    -e, --exclude <exclude>    Will ignore all files and directories containing the word ie --exclude node_modules
    -s, --sort <sort>          Will sort based on a certain column ie --sort=files will sort by file count. [values: files total blanks code commments]

ARGS:
    input...    The input file(s)/directory(ies)

```

## Supported Languages

If there is a language that you want added submit a pull request with the following information

- Name of language
- Most common file extension
- The comment syntax (Does it have block comments? is it the same as C?)

```
ActionScript
Assembly
BASH
Batch
C
C Header
C Shell
C#
C++
C++ Header
CSS
Clojure
CoffeeScript
ColdFusion
ColdFusion CFScript
D
Dart
Device Tree
FORTRAN Legacy
FORTRAN Modern
Go
HTML
Haskell
JAI
JSON
JSX
Java
JavaScript
Julia
LD Script
LESS
LISP
Lua
Makefile
Markdown
OCaml
Objective C
Objective C++
PHP
Pascal
Perl
Plain Text
Polly
Python
R
Ruby
Ruby HTML
Rust
SQL
Sass
Standard ML
Swift
TOML
TeX
TypeScript
XML
YAML
```

## Common issues

If you get errors like the following, it is mostly like due to having folders with paths that too long. For example NPM<3.0.0 `node_modules` generates long path files, which causes problems on windows. You can exclude these paths with the`exclude` argument, or in the case of NPM, update to >3.0.0, and update your codebase dependencies.
```
"The system cannot find the path specified.\r\n"
```
```
thread <main> has overflowed its stack
Illegal instruction: 4
```

## Copyright and License
(C) Copyright 2015 by Aaron Power and contributors

See CONTRIBUTORS.md for a full list of contributors.

Tokei is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENCE-APACHE](./LICENCE-APACHE), [LICENCE-MIT](./LICENCE-MIT) for more information.
