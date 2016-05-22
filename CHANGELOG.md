# 2.1.0
Tokei, can now output results in vsrious formats(_cbor, json, yaml_)
Conversely tokei can now take in results in those formats, and add them to the current run.
Premilarily support for nested comments(_currently only rust supports it_)
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
