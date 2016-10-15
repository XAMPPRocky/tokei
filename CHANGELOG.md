# 4.3.0

- @lligo : Tokei no longer panics on non-character-boundary when printing file names.
- Fixed regression where no comment style files(_json, markdown_) weren't counted.
- Tokei can now handle files in different encodings.(_using the [encoding](https://crates.io/crates/encoding) library_)
- Tokei now prints errors instead of sliently skipping them.
- Tokei can now print unused extensions using `-v` option.


**Added languages:**
- Asp(_asa, asp_)
- Asp.NET(_asax, ascx, asmx, aspx, master, sitemap, webinfo_)
- GLSL(_vert, tesc, tese, geom, frag, comp_)
- Hex(_hex_)
- Intel Hex(_ihex_)
- ReStructuredText(_rst_)
- Razor(_cshtml_)

**Changes to existing languages Thanks to @mwilli20 :**
- Another Ada extension(_pad_)
- Assembly - Uses `' '` or `" "` and added another extension(_asm_)
- Bash - Uses `' '` or `" "`
- Batch - They don't use quotes for strings, added `::`
- Cold Fusion - Uses `' '` or `" "`
- D - Uses `" "` or
- Dart - Uses `" "` or `' '` or `""" """` or `''' '''`
- Forth - Uses `" "` but new, doesn't have a preset
- Fortrans - Use `" "` or `' '`
- Idris - Uses `" "` or `""" """`
- Julia - Uses `" "` or `""" """`
- Kotlin - Uses `" "` or `""" """`
- Lisp - Comments can be nested
- Moustache - Uses `" "` or `' '`
- Nim - Uses `" "` or `""" """`
- Pascal - Uses `' '`
- Perl - Uses `" "` or `' '`
- Php - Uses `" "` or `' '`
- Python - Uses `" "` or `' '` or `""" """` or `''' '''`
- Ruby - Uses `" "` or `' '`
- Sass - Uses `" "` or `' '`
- Sql - Uses `' '`
- Toml - Uses `" "` or `' '` or `""" """` or `''' '''`
- Typescript - Uses `" "` or `' '` or
- Vimscript - Uses `" "` or `' '`
- Yaml - Uses `" "` or `' '`
- Zsh - Uses `" "` or `' '`
-  Clojure - Removed `#`
- Forth   - `( Comment)` style comments need a space after the opening paren
- Haskell - Has nested comments
- Idris - Has nested comments
- Jai     - Has nested block comments
- Julia   - Has nested block comments
- Kotlin  - Has nested block comments
- Pascal  - Pascal should be multiline from `{` or `(*` to `}` or `*)`
- Perl    - Perl5 and earlier for multiline comments need `=pod` to `=cut`.
- Swift   - Has nested block comments

### Tokei's code count
```
-------------------------------------------------------------------------------
 Language            Files        Lines         Code     Comments       Blanks 
-------------------------------------------------------------------------------
 Rust                   13         2413         1596          601          216 
-------------------------------------------------------------------------------
 |ib\language\languages.rs          693          420          197           76 
 |anguage\language_type.rs          500          386          102           12 
 .\src\main.rs                      314          256           17           41 
 |lib\language\language.rs          356          166          166           24 
 .\src\lib\utils\fs.rs              129          107            9           13 
 |\lib\utils\multi_line.rs          149           89           39           21 
 .\src\lib\utils\macros.rs           59           50            3            6 
 .\src\lib\stats.rs                  63           45           12            6 
 .\src\lib\lib.rs                    76           25           47            4 
 .\src\lib\build.rs                  31           23            0            8 
 .\src\lib\sort.rs                   28           19            6            3 
 .\src\lib\language\mod.rs           11            6            3            2 
 .\src\lib\utils\mod.rs               4            4            0            0 
-------------------------------------------------------------------------------
 Markdown                4          492          492            0            0 
-------------------------------------------------------------------------------
 .\README.md                        252          252            0            0 
 .\CHANGELOG.md                     202          202            0            0 
 .\CONTRIBUTING.md                   25           25            0            0 
 .\CONTRIBUTORS.md                   13           13            0            0 
-------------------------------------------------------------------------------
 YAML                    2           70           67            3            0 
-------------------------------------------------------------------------------
 .\cli.yml                           53           50            3            0 
 .\.travis.yml                       17           17            0            0 
-------------------------------------------------------------------------------
 TOML                    1           80           65            0           15 
-------------------------------------------------------------------------------
 .\Cargo.toml                        80           65            0           15 
-------------------------------------------------------------------------------
 Autoconf                1            9            7            1            1 
-------------------------------------------------------------------------------
 .\src\lib\lib.rs.in                  9            7            1            1 
-------------------------------------------------------------------------------
 Total                  21         3064         2227          605          232 
-------------------------------------------------------------------------------
```

# 4.2.0

Tokei is now more precise, and shouldn't ever panic.
Tokei now handles comments in quotes and more precise nested comments properly.
Fixes #53

### Tokei's code count.

```
-------------------------------------------------------------------------------
 Language            Files        Lines         Code     Comments       Blanks
-------------------------------------------------------------------------------
 Rust                   13         2303         1487          594          222
-------------------------------------------------------------------------------
 |ib\language\languages.rs          682          401          198           83
 |anguage\language_type.rs          467          359           96           12
 .\src\main.rs                      302          243           17           42
 |lib\language\language.rs          356          166          166           24
 .\src\lib\utils\fs.rs              116           95            9           12
 |\lib\utils\multi_line.rs          156           93           41           22
 .\src\lib\stats.rs                  54           36           12            6
 .\src\lib\build.rs                  31           23            0            8
 .\src\lib\lib.rs                    69           22           43            4
 .\src\lib\utils\macros.rs           27           20            3            4
 .\src\lib\sort.rs                   28           19            6            3
 .\src\lib\language\mod.rs           11            6            3            2
 .\src\lib\utils\mod.rs               4            4            0            0
-------------------------------------------------------------------------------
 YAML                    2           68           65            3            0
-------------------------------------------------------------------------------
 .\cli.yml                           49           46            3            0
 .\.travis.yml                       19           19            0            0
-------------------------------------------------------------------------------
 TOML                    1           71           58            0           13
-------------------------------------------------------------------------------
 .\Cargo.toml                        71           58            0           13
-------------------------------------------------------------------------------
 Autoconf                1            9            7            1            1
-------------------------------------------------------------------------------
 .\src\lib\lib.rs.in                  9            7            1            1
-------------------------------------------------------------------------------
 Total                  17         2451         1617          598          236
-------------------------------------------------------------------------------
```

# 4.1.0

Tokei is now **~40%** faster.

**Added languages**

- Ada
- Forth


# 4.0.0

Tokei now has a minimal version without `serde` for faster compilation.

Updated various dependencies.

Internal dependencies removed.

## Regressions
- CBOR is not supported till it supports `serde 0.8`

**Added languages**

- Handlebars


# 3.0.0
Tokei is now available as a library.

Tokei now has a lot more tests.

Tokei now supports TOML

Fixed #41

Fixed #44

Fixed #45


# 2.1.0
Tokei, can now output results in various formats(_cbor, json, yaml_)

Conversely tokei can now take in results in those formats, and add them to the current run.

Premilarily support for nested comments(_currently only supported for rust_)

Change in the output format [PR #35](https://github.com/Aaronepower/tokei/pull/35)

Moved  `.sc` from Lisp to Scala.

Internals changed to allow for multiple multi line comment formats.

**Added languages:**
- Isabelle

# 2.0.0

Major rewrite, now parallelized.
Can now support sorting files.
Added a progress message for when it is counting files.
Fixed #29

**Added languages:**
- Coq
- Erlang
- Kotlin
- Idris
- Nim
- Oz
- Prolog
- Qcl
- Scala
- Unreal Script
- Wolfram

# 1.6.0

Added file counting.

# 1.5.0

Added Shebang support.

**Added languages:**
- Assembly
- LD Scripts
- Device Trees
- Makefiles
- Plain Text
- C Shell

# 1.4.1

Changed the formatting so tokei looks nice for consoles of 80 column width.


# 1.4.0

Changed from handmade recursive file opening to [walkdir](https://github.com/BurntSushi/walkdir)
