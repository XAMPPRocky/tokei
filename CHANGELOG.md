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
