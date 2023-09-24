# Contributing to Tokei

- [Language Addition](#language-addition)
- [Bug Reports](#bug-reports)

# Language Addition

Currently, Tokei generates languages from the [`languages.json`](languages.json)
file. JSON was chosen to make it easy to add new languages and change code
structure without changing large data structures. Here, we will go over the
properties of a language in `languages.json` through examples.

```json
"JavaScript": {
      "line_comment": ["//"],
      "multi_line_comments": [["/*", "*/"]],
      "quotes": [["\\\"", "\\\""], ["'", "'"], ["`", "`"]],
      "extensions": ["js", "mjs"]
},
```

Above is the JavaScript's definition. The first thing that needs to be defined
is the key. The key's format should be same as [Rust's enum style]. As this key
will be used in an enum for identifying the language. For a lot of languages,
this also works for showing the language when we print to the screen.

However, there are some languages whose names don't work with the enum style.
For example, `JSON` is usually shown in all caps, but that doesn't fit in Rust's
enum style. So we have an additional optional field called `name` which defines
how the language should look when displayed to the user.

```json
"Json": {
    "name": "JSON",
    //...
},
```

For defining comments, there are a few properties. The most commonly used
property is `line_comment` which defines single line comments. These are comments
which don't continue onto the next line. Here is an example in Rust:

```rust
let x = 5; // default x position
let y = 0; // default y position
```

The `line_comment` property expects an array of strings, as some languages have
multiple syntaxes for defining a single line comment. For example, `PHP` allows
both `#` and `//` for single line comments.

```json
"Php": {
    "line_comment": [
        "#",
        "//"
    ],
    //...
},
```

For defining comments that also have an ending syntax, there is the `multi_line`
property. An example for such comments in Rust:

```rust
let x = /* There is a reason
    for this comment, I swear! */
    10;
```

The `verbatim_quotes` property expects an array of strings, as some languages
have multiple syntaxes for defining verbatim strings. A verbatim string
in the context of Tokei is a string literal that can have unescaped `"`s. For example [`CSharp`](https://docs.microsoft.com/en-us/dotnet/csharp/programming-guide/strings/#regular-and-verbatim-string-literals)

```json
"CSharp": {
  "verbatim_quotes": [
    [
      "@\\\"",
      "\\\""
    ]
  ],
  //...
},
```

```csharp
const string BasePath = @"C:\";
```

Some languages have a single, standard filename with no extension
like `Makefile` or `Dockerfile`. These can be defined with the
`filenames` property:

```json
"Makefile": {
    "filenames": [
        "makefile"
    ],
    "extensions": [
        "makefile",
        "mak",
        "mk"
    ]
},
```

Filenames should be all-lowercase, whether or not the filename
typically has capital letters included.

Note that filenames will **override** extensions. With the
following definition, a file named `CMakeLists.txt` will be
detected as a `CMake` file, not a `Text` file.

```json
"Text": {
    "extensions": [
        "txt"
    ]
},
"CMake": {
    "filenames": [
        "cmakelists.txt"
    ]
},
```

# Tests

A test file is required for language additions. The file should
contain every variant comments and quotes, as well as a comment
at the top of the file containing the manually verified lines,
code, comments, blanks in the following format:

```
NUM lines NUM code NUM comments NUM blanks
```

### Example

In Rust for example, the first line should look like the following:

```rust
//! 39 lines 32 code 2 comments 5 blanks
```

The comment should use the syntax of the language you're testing.
A good example of a test file is [`tests/data/rust.rs`](tests/data/rust.rs).

```rust
//! 48 lines 36 code 6 comments 6 blanks
//! ```rust
//! fn main () {
//!     // Comment
//!
//!     println!("Hello World!");
//! }
//! ```

/* /**/ */
fn main() {
    let start = r##"/*##\"
\"##;
    // comment
    loop {
        if x.len() >= 2 && x[0] == '*' && x[1] == '/' { // found the */
            break;
        }
    }
}

fn foo<'a, 'b>(name: &'b str) {
    let this_ends = "a \"test/*.";
    call1();
    call2();
    let this_does_not = /* a /* nested */ comment " */
        "*/another /*test
            call3();
            */";
}

fn foobar() {
    let does_not_start = // "
        "until here,
        test/*
        test"; // a quote: "
    let also_doesnt_start = /* " */
        "until here,
        test,*/
        test"; // another quote: "
}

fn foo() {
    let a = 4; // /*
    let b = 5;
    let c = 6; // */
}


```

# Bug Reports

Please include the error message and a minimum working example
including the file or file structure.

````
This file crashes the program:

<filename>
```
<file/file structure>
```
````

[Rust's enum style]: https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md#general-naming-conventions
