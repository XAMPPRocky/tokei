# Tokei

[![GitHub License](https://img.shields.io/github/license/Aaronepower/tokei.svg)](http://github.com/Aaronepower/tokei/blob/master/LICENSE)
[![](https://img.shields.io/github/release/Aaronepower/tokei.svg)](https://github.com/Aaronepower/tokei/releases/tag/1.1.1/)
[![](https://img.shields.io/travis/Aaronepower/tokei.svg)](https://travis-ci.org/Aaronepower/tokei)
[![](https://img.shields.io/github/downloads/Aaronepower/tokei/latest/total.svg)](https://github.com/Aaronepower/tokei/releases/)
[![](https://img.shields.io/github/issues-raw/Aaronepower/tokei.svg)](http://github.com/Aaronepower/tokei/issues)

A blazingly fast CLOC(Count Lines Of Code) program, written in Rust.

## Installation

### Automatic
If you have [`cargo 0.6.0>=`](https://www.rust-lang.org/downloads.html) installed just run the `cargo install` command.

```
$ cargo install tokei
```

### Manual

#### Fedora 64 bit
Install rust and cargo from either the [official page](https://www.rust-lang.org) or use a copr repo such as [Rust](https://copr.fedoraproject.org/coprs/phnxrbrn/rust/)

`$ dnf copr enable phnxrbrn/tokei`

`$ dnf install tokei`


#### Other
`$ git clone https://github.com/Aaronepower/tokei.git`

`$ cd tokei`

`$ cargo build --release`

##### Linux
`# sudo mv target/release/tokei /usr/local/bin`

##### OSX
`# sudo mv <your_download_location>/tokei /usr/local/bin/tokei`

##### Windows
- Create a folder for tokei
- search for `env`
- open "edit your enviroment variables"
- edit `PATH`
- paste folder path to the end of the string ie: `<path_stuff_here>;C:/tokei;`

## Usage

To use tokei, use must add it to your path. Then you can call tokei like so
`$ tokei ./path/to/code`

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
BASH
Batch
C
C++
ColdFusion CFScript
ColdFusion
Clojure
CoffeeScript
C#
CSS
D
Dart
LISP
FORTRAN Legacy
FORTRAN Modern
Go
C Header
C++ Header
Haskell
HTML
JAI
Java
Julia
JavaScript
JSON
JSX
LESS
Objective-C
Markdown  
Objective-C++
Pascal
PHP
Perl
Python
R
Ruby
Ruby HTML
Rust
Sass
SQL
TeX
Swift
TOML
TypeScript
XML
YAML
```

## Common issues

If you get errors like the following, it is mostly like due to having folders with paths that too long. For example NPM<3.0.0 `node_modules` generates long path files, either use the `exclude` argument, or in the case of NPM, update to >3.0.0
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

This application is free software: you can redistribute it and/or modify
it under the terms of the MIT License.

See LICENSE for more information.
