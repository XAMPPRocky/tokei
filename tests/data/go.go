// 37 lines 24 code 6 comments 7 blanks

// Package main is a test file.
package main

import (
	"errors"
)

/* /**/
func main() {
	start := "/*"

	for {
		if len(start) >= 2 && start[1] == '*' && start[0] == '/' { // found the */
			break
		}
	}

	if err := Foo(42, start); err != nil {
		panic(err)
	}
}

// Foo is a function. /* nested comment */
func Foo(
	// first
	a int,
	s string, /* second */
) (err error) {
	m := `a
multiline
string`
	return errors.New(m)
}

// end of file
