* 16 lines 6 code 7 comments 3 blanks
* This is a comment
**** Any number of * symbol

use "foo.dta", replace
gen x = 1*2
gen x2 = 1/2
/*
Here's a comment block
*/

if c(username) == "foobar" {
    global FOO 1
}

// Finally another symbol for comment