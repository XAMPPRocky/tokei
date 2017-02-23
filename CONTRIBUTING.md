# Contributing to Tokei

* [Language Addition](#language-addition)
* [Bug Reports](#bug-reports)

# Language Addition
Currently tokei generates languages from the [`languages.json`](languages.json)
file. JSON was decided to make it easy to add new languages, and change code
structure without changing large data structures. Here we will go over the
properties of a language in `languages.json`, through examples.

```
"JavaScript":{
    "base":"c",
    "quotes":[
        [
            "\\\"",
            "\\\""
        ],
        [
            "'",
            "'"
        ],
        [
            "`",
            "`"
        ]
    ],
    "extensions":[
        "js"
    ]
},
```

Above is the JavaScript's definition. The first thing that needs to be defined
is the key, the keys format should be same as 
[Rust's enum style](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md#general-naming-conventions).
As this key will be used in an enum for identifying the language. For a lot of 
language's this also works for showing the language when we print to the screen. 
However there are some languages whose names don't work with the enum style.
For example `JSON` is usually shown in all caps, but that doesn't fit in Rust's
enum style. So we have an additional optional field called `name`, which defines
how the language should look when displayed to the user.

```json
"Json" {
    "name": "JSON",
```

For defining comments has a few properties: firstly is the most commonly used
`single` property which defines single line comments. Comments which don't
continue onto the next line.

```rust
let x = 5; // default x position
let y = 0; // default y position
```

The `single` property expects an array of strings, as some languages have 
multiple syntaxes for defining a a single line comment. For example `PHP` allows
both `#` and `//` as comments.

```json
"Php": {
    "single": [
        "#",
        "//"
    ]
```

For defining comments that also have a ending syntax, there is the `multi_line`
property.

```
let x = /* There is a reason
    for this comment I swear */
    10;
```

A lot of languages have the same commenting syntax usually inheriting from the 
authors previous language or preferred language. In order to avoid code reuse
tokei's languages have a `base` property which says to use a common comment
syntax. 

* Name of the language
* Any file extensions associated with the language
* The comment syntax
  - Does it have multiple single line comment symbols?
  - Does it only contain single line comments? Or only multi-line comments?
  - Is just C style comments? `/* */, //`

Some languages have a single, standard filename, like Makefile or Dockerfile.
These can be defined with the `filenames` property:

```json
"Makefile":{
    "filenames":[
        "makefile"
    ],
    "extensions":[
        "makefile",
        "mak",
        "mk"
    ]
}
```

Filenames should be all-lowercase, whether or not the filename typically has capital letters included.

Note that filenames will *override* extensions, so with the following
configuration:

```json
"Toml":{
    "extensions": [
        "toml"
    ]
},
"Cargo":{
    "filenames": [
        "cargo.toml"
    ]
}

A file named `Cargo.toml` will be detected as a Cargo file, not a TOML file.

```

# Bug Reports
Please include the error message, and a minimum working example including the file, or file structure.

```
This file crashes the program.

<filename>
\`\`\`
\`\`\`
```
