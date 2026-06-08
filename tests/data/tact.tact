// 20 lines 12 code 4 comments 4 blanks

import "@stdlib/deploy"; // comment

/* comment */
fun global() {
    let str: String = "\n \r \t \u1234 \xFF";

    // comment
    while (true) { // comment
        if /* comment */ (true) { /* comment */ }
    }
}

// "quoted"
struct St { /* " */
    field1: Int; // /*
    field2: Int as uint128;
    field3: Int; // */
}
