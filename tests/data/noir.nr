//! 39 lines 25 code 11 comments 3 blanks
//! ```noir
//! fn main() {
//!     // Comment
//!
//!     println("Hello World!");
//! }
//! ```

/*  */
fn main(x: [Field; 3]) {
    assert_eq(1, 0, f"format string");
    // comment
    if (x.len() >= 2) & (x[0] == 2) & (x[1] == 3) {
        // found the */
        println(foo)
    }
    println(r#""#);
    println(
        r##"
        foo
    "##,
    );
}

fn foo() {
    let this_ends = "a \"test/*.";
    call1();
    call2();
    let this_does_not = /* a  nested  comment " */ "*/another /*test
            call3();
            */";
}

fn foo() {
    let a = 4; // /*
    let b = 5;
    let c = 6; // */
}
