// 16 lines 9 code 3 comments 4 blanks

// Some example settings
#set document(title: "a title", author: "an author")
#set page(numbering: "1 / 1", number-align: center)
#set par(justify: true)
#set text(size: 13pt, lang: "fr") // with a trailing comment
#set heading(numbering: "1.1") /* with another trailing comment */

#let foo = "multiline
string"

#let bar = "singleline string"

/* comment */ /* nested /* comment */ */
#lorem(50)
