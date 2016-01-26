# Contributing to Tokei

* [Language Addition](#language-addition)
* [Bug Reports](#bug-reports)

# Language Addition
In order to properly add a language to count, there is a couple of key pieces needed.

* Name of the language
* Any file extensions associated with the language
* The comment syntax
  - Does it have multiple single line comment symbols?
  - Does it only contain single line comments? Or only multi-line comments?
  - Is just C style comments? `/* */, //`

# Bug Reports
Please include the error message, and a minimum working example including the file, or file structure.

```
This file crashes the program.

<filename>
\`\`\`
\`\`\`
```