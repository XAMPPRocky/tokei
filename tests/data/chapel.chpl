// chapel 24 lines 9 code 9 comments 6 blanks

// Tidy line comment

/* Tidy block
   comment.
*/

// Cheeky line comments /*
// */

/* Cheeky // block comments */

// Caculate a factorial
proc factorial(n: int): int {
    var x = 1; // this will eventually be returned
    for i in 1..n {
        x *= i;
    }
    return x;
}

writeln("// this isn't a comment");
writeln('/* this is also not a comment */');
