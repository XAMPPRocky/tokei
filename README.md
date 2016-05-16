# Tokei ([時計](https://en.wiktionary.org/wiki/%E6%99%82%E8%A8%88))

[![GitHub License](https://img.shields.io/github/license/Aaronepower/tokei.svg)](http://github.com/Aaronepower/tokei/blob/master/LICENSE)
[![](https://img.shields.io/github/release/Aaronepower/tokei.svg)](https://github.com/Aaronepower/tokei/releases/tag/1.1.1/)
[![](https://img.shields.io/travis/Aaronepower/tokei.svg)](https://travis-ci.org/Aaronepower/tokei)
[![](https://img.shields.io/crates/d/tokei.svg)](https://crates.io/crates/tokei)
[![](https://img.shields.io/github/issues-raw/Aaronepower/tokei.svg)](http://github.com/Aaronepower/tokei/issues)

A blazingly fast CLOC(Count Lines Of Code) program, written in Rust.

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

## Usage

To use tokei, use must add it to your path. Then you can call tokei like so

```shell
$ tokei ./path/to/code
```

## Options
```
Tokei 2.0.0
Aaron P. <theaaronepower@gmail.com>
Count code, quickly.

USAGE:
    Tokei [FLAGS] [OPTIONS] <input>...

FLAGS:
    -f, --files        Will print out the files found only recommended for debugging purposes
    -h, --help         Prints help information
    -l, --languages    prints out supported languages and their extensions
    -V, --version      Prints version information

OPTIONS:
    -e, --exclude <exclude>    Will ignore all files and directories containing the word ie --exclude node_modules
    -s, --sort <sort>          Will sort based on a certain column ie --sort=files will sort by file count. [values: files, total, blanks, code, commments]

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
Assembly
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
FORTRAN Legacy
FORTRAN Modern
Go
Haskell
HTML
Idris
JAI
Java
JavaScript
Julia
JSON
JSX
Kotlin
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
Ruby
Ruby HTML
Rust
Sass
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

## Tokei says I have a lot of D code, but I know there is no D code!
This is likely due to `gcc` generating `.d` files. Until the D people decide on a different file extension, you can always exclude `.d` files using the `-e --exclude` flag like so

```
$ tokei . -e .d
```

## Copyright and License
(C) Copyright 2015 by Aaron Power and contributors

See CONTRIBUTORS.md for a full list of contributors.

Tokei is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENCE-APACHE](./LICENCE-APACHE), [LICENCE-MIT](./LICENCE-MIT) for more information.
