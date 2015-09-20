# Tokei
A blazingly fast CLOC(Count Lines Of Code) program, written in Rust.


# Usage

To use tokei, use must add it to your path. Then you can call tokei like so
`$ tokei ./path/to/code`

## OSX
`# sudo mv <your_download_location>/tokei /usr/local/bin/tokei`

## Windows

- Create a folder for tokei
- search for `env` 
- open "edit your enviroment variables"
- edit `PATH`
- paste folder path to the end of the string ie: `<path_stuff_here>;C:/tokei;`

# Options
``` 
Aaron P. <theaaronepower@gmail.com>
A quick CLOC (Count Lines Of Code) tool

USAGE:
  tokei [FLAGS] [OPTIONS] [--] <input>...

FLAGS:
    -h, --help         Prints help information
    -l, --languages    prints out supported languages and their extensions
    -V, --version      Prints version information

OPTIONS:
    -e, --exclude <exclude>...    Will ignore all files and directories containing the word ie --exclude node_modules
    -s, --sort <sort>             Will sort based on a certain column ie --sort=files will sort by file count.

ARGS:
    input...    The input file(s)/directory(ies)

```

# Supported Languages

If there is a language that you want added submit a pull request with the following information

- Name of language
- Most common file extension
- The comment syntax (Does it have block comments? is it the same as C?)

```
ActionScript              (as)
C                         (c)
ColdFusion CFScript       (cfc)
ColdFusion                (cfm)
Clojure                   (clj)
CoffeeScript              (coffee)
C++                       (cpp)
C#                        (cs)
CSS                       (css)
D                         (d)
Dart                      (dart)
LISP                      (el)
Go                        (go)
C Header                  (h)
C++ Header                (hpp)
Haskell                   (hs)
HTML                      (html)
Java                      (java)
JavaScript                (js)
JSON                      (json)
JSX                       (jsx)
Objective-C               (m)
Objective-C++             (mm)
Pascal                    (pas)
PHP                       (php)
Perl                      (pl)
Python                    (py)
R                         (r)
Ruby                      (rb)
Ruby HTML                 (rhtml)
Rust                      (rs)
Sass                      (sass)
BASH                      (sh)
SQL                       (sql)
Swift                     (swift)
TypeScript                (ts)
XML                       (xml)
YAML                      (yml)
```

# Common issues

If you get errors like the following, it is mostly like due to having folders with paths that too long. For example NPM<3.0.0 `node_modules` generates long path files, either use the `exclude` argument, or in the case of NPM, update to >3.0.0
```
"The system cannot find the path specified.\r\n"
```
```
thread <main> has overflowed its stack
Illegal instruction: 4
```
