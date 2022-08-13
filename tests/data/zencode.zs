// 21 lines 9 code 7 comments 5 blanks

// This is a single line comment.
/* This is a multiline comment on a single line. */
/*
  This is a multiline comment.
*/

var str = "/*";
var arr = [str, @"wysiwyg", '\"'];

for item in arr {
    print(item); // Found the */
}

// Comment with quote "

var badStr = // Comment before value
    "\"";
badStr = // Another comment before value
    @'zen';
