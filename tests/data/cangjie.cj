/* 50 lines 34 code 8 comments 8 blanks */

/* /* we can nest block*/ comments in cangjie */
main(): Unit {
    let start: String = "/*"

    var x: Int64 = 1
    x += 2 // end of line comment */
}

func foo(): Unit {
    let esc: String = "\"/*escaped quotes in a string and block comment*/\"";
    let next_line: String = """
		*/ /*string on new line
		continued to another line
		bar();
		*/"
"""

    /* Block comment
    // line comment in a block comment
    end block comment*/
    let late_start: String = // "
        ##"wow\
		that's pretty neat"##

    let late_start2: String = /* " */
        "*/ still just a string" // but this is a line comment
}

func foobar(): Unit {
    let a: Int64 = 4 // /*
    let b: Int64 = 5
    let c: Int64 = 6 // */
}

/* \
 / comment
\*/
struct Point {
    let x: Int64
    let y: Int64
    let z: Int64

    public init(x: Int64, y: Int64, z: Int64) {
        this.x = x
        this.y = y
        this.z = z
    }
}
