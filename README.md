# Tokei
A blazingly fast CLOC(Count Lines Of Code) program, written in Rust.


# Usage

To use tokei, use must add it to your path.

## OSX
`sudo mv <your_download_location> /usr/local/bin/tokei`

## Windows

- Create a folder for tokei
- search for `env` 
- open "edit your enviroment variables"
- edit PATH
- paste folder path to the end of the string ie: `<path_stuf_here>;C:/tokei;`

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
