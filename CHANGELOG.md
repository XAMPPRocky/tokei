# 10.1.1

- Fixed `.tokeignore` always working even when `--no-ignore` is present.
- Updated dependencies

**Added languages**

- @erikaxel Gherkin (Cucumber)

# 10.1.0

- Added `cjsx` extension to CoffeeScript.
- Tokei will now recognise files with `#!/usr/bin/env ruby` as Ruby.
- Updated dependencies.
- Tokei now uses `crossbeam` channels over `std::mpsc`, which should have a
  noticeable performance improvement on large repos.
- Improved documentation for `libtokei`.

**Added languages**

- @lzybkr PowerShell
- @turbo MoonScript
- @dtolnay Thrift
- @Tranzystorek FlatBuffers
- @NieDzejkob Emojicode
- @DanteFalzone0 HolyC
- @sci4me Odin
- @fkarg Rusty Object Notation (RON)

# 10.0.0

- Fixed minor parsing bugs.
- Width is now limited to 80 unless you use the `--files` flag.
- Added the `mjs` extension to JavaScript.
- Added the `tpp` extension to C++.
- You can now disable Tokei's git ignore detection, similar to ripgrep. See
  `--help` for options.
- You can now add a `.tokeignore` file to your project to specify file paths
  for tokei to always ignore. This file uses the same syntax as `.gitignore`.
- Improved Pascal representation

**Added languages**

- @hobofan solidity
- @stefanmaric GraphQL
- @jhpratt PostCSS
- @evitalis RPM
- @alexmaco Pony
- @yjhmelody WASM, LLVM, Pest
- @XAMPPRocky ASN.1

# 9.0.0

- Tokei now has config files. You can now specify some commonly used arguments
  in a `.tokeirc`/`tokei.toml`. Namely `columns` to set the default column
  output, `types` to filter your count to just a single set of languages, and
  `treat_doc_strings_as_comments` which is a new option that allows you to
  specify whether to treat doc strings such as `"""` in Python as comments
  or code.
  The config files can be specified in two places, the current directory tokei
  is running in and your [system configuration
  directory](//docs.rs/tokei/struct.Config.html#method.from_config_files). The
  priority of options is as follows
  `CLI > <current_directory> > <configuration_directory>`.
- Tokei is now available on [Conda](https://anaconda.org/conda-forge/tokei).
- [Tokei's README has been translated
  to chinese.](https://github.com/chinanf-boy/tokei-zh#tokei-)
- `LanguageType` now implements `Hash`.
- Tokei now batches it's console output, this should result in a small
  performance boost.
- There is now a `--columns` argument for manually setting tokei's output width.
- The `--sort` argument is now case-insensitive.
- Tokei will now mark languages who's files failed to parse correctly as
  potentially inaccurate.
- Due to a bug in trust-ci `x86_64-unknown-netbsd` versions are will not be
  available in GitHub releases. (You will still be able to install from source.)
- Due to toml-rs's lacking enum support the TOML output option has
  been disabled.

**Added languages**

- @t-richards Liquid
- @diaphore Added the `.glsl` extension to GLSL.
- @ahmedelgabri Twig
- @pmoura Logtalk
- @alekratz Perl, Not Quite Perl
- @XAMPPRocky Automake, .NET Resource, HLSL, INI, Unreal Plugin,
  Unreal Project, Unreal Shader, Unreal Shader Header, Unreal Markdown,
  Visual Basic, Visual Studio Solution, Visual Studio Project, Xcode Config,
- @TheMrNomis SWIG
- @xnorme Added the `.vhdl` extension to VHDL

# 8.0.0

- A language's comments, and quotes are now available through the `LanguageType`
  enum.
- You can filter by language using the `-t/--type` option. e.g. `tokei -t "Rust,C"`
  will print only Rust and C files.
- Tokei now understands terminal width and will expand to fit it. (Thanks
  to @Veykril)
- Added [comparison](./COMPARISON.md) document to compare Tokei to other
  code counters.
- Updated dependencies

**Added languages**

- @BrandonBoone VB6, VBScript, XSLT
- @ialpert BrightScript
- @PJB3005 Dream Maker
- @schmee edn

# 7.0.3

Made various optimisations, up to 65% faster in some cases.

**Added languages**

- @DenialAdams Added Forsyth-Edwards-Notation (FEN)
- @DjebbZ Added ClojureC
- @grimm26 Added HCL/Terraform

# 7.0.2

- Updated dependencies.
- Changed how compilied serialization formats are handled.
- Fixed minor parser inaccuracies.
- Tokei should now recognise more python files from their shebang.

**Added languages**

- @ignatenko Added Meson
- @sprang Added Scheme
- @fengcms Added Vue
- @mark.knol Added Haxe
- @rleungx Added ABAP, COBOL, and Groovy
- @tiehuis Added Zig
- @murielsilveira Added Mint
- @notramo Added Elvish Shell and Kakoune
- @aatxe Added Racket
- @kamilchm Added ReasonML
- @cyplp Added XSL

# 7.0.1

- Updated dependencies

# 7.0.0

- Fixed parsing corner cases
- Changed storage of comments and quotes from `Vec` to static slices.
- Added tracing for debugging single files. Not recommended for use on
  multiple file
- Updated `log`

# 6.1.0

- Fixed inaccuracies relating to the end comment being smaller than start
  comment.

**Added languages**

- @mattico Added Xaml
- @weakish Added Ceylon
- @theduke Added tsx extension to typescript
- @vmchale Added Hamlet, Cassius, Lucius, Cabal, Nix, Happy, Alex, and Madlang
- @notramo Added Crystal

# 6.0.2

- Now can recognise file languages based on their filename.

**Added Languages:**

- @kazimuth CMake, Dockerfile, Rakefile, Scons

# 6.0.1

- Multiple exclude flags now allowed.

**Added Languages:**

- @seiks Added Fish Shell
- @XAMPPRocky Added Module-Definition
- @tbu- Added Vala

# 6.0.0

- Reworked internals
- Now uses serde*derive(\_and thusly requires rust v1.15*)
- Now has better file based testing

**Added languages:**

- @tuncer Added Ur/Web
- @svisser Added PureScript
- @tjodden Add some common extensions for HTML, C++ and Makefile
- @xd009642 Added VHDL

# 5.0.0

- Optimised internals

**Added languages:**

- @GungnirInd Added GDScript
- @tuncer Differentiate between sh and Bash, Added Cogent, F\*, F#
- @pthariensflame Added Agda

# 4.5.0

- Added Regex based hueristics so more expensive multi line handling isn't used
  if there are no multi line comments in the file.
- Now uses the `ignore` crate for getting files. Which now also makes
  determining language from path/file parallelised
- File counting used to only be parallelised per language, now it is also
  parallelised per file per language.
- Updated homepage, and documentation links
- @rmbreak Tokei will now not add directories with `foo.bar` like syntax
  to a language.
- @Michael-F-Bryan tokei will now exit gracefully when a feature is missing
  instead of panicing

**Added languages:**

- @hauleth Added Elixir support

# 4.4.0

- Simplified language definitions, now consolidated into a single JSON file.
- Fixed regression where lines and files weren't sorted.
- @llogiq : made clippy fixes
- @lligo : Added long verbose name

**Added languages:**

- @little-dude : Tcl(_tcl_)
- @svenstaro : GLSL(_vert, tesc, tese, geom, frag, comp_)
- @not-fl3 : Elm(_elm_)

**Changes to existing languages:**

- @xpayn : Added `pm` extension to Perl.

# 4.3.0

- @lligo : Tokei no longer panics on non-character-boundary when printing file names.
- Fixed regression where no comment style files(_json, markdown_) weren't counted.
- Tokei can now handle files in different encodings.(_using the [encoding](https://crates.io/crates/encoding) library_)
- Tokei now prints errors instead of sliently skipping them.
- Tokei can now print unused extensions using `-v` option.

**Added languages:**

- Asp(_asa, asp_)
- Asp.NET(_asax, ascx, asmx, aspx, master, sitemap, webinfo_)
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
- Clojure - Removed `#`
- Forth - `( Comment)` style comments need a space after the opening paren
- Haskell - Has nested comments
- Idris - Has nested comments
- Jai - Has nested block comments
- Julia - Has nested block comments
- Kotlin - Has nested block comments
- Pascal - Pascal should be multiline from `{` or `(*` to `}` or `*)`
- Perl - Perl5 and earlier for multiline comments need `=pod` to `=cut`.
- Swift - Has nested block comments

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

Change in the output format [PR #35](https://github.com/XAMPPRocky/tokei/pull/35)

Moved `.sc` from Lisp to Scala.

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
