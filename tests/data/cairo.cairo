//! 51 lines 32 code 13 comments 6 blanks
//! ```rust
//! fn main () {
//!     // Comment
//!
//!     println!("Hello World!");
//! }
//! ```

/// The main function
fn main() {
    let x: ByteArray = "\"/*##\"\"##\'\'";
    // comment
    loop {
        if x.len() >= 2 && x[0] == '*' && x[1] == '/' { // found the */
            break;
        }
    }
}

fn foo<T, +Drop<T>>(name: T) {
    let this_ends = 'a "\'test/"*.';
    call1();
    call2();
    let this_does_not =  // a // nested // comment " //
    ///"*/another /*test
    call3();
//*/";
}

fn call1() {}
fn call2() {}
fn call3() {}

fn foobar() {
    let does_not_start: ByteArray =  // "
    "until here,
        test/*
        test"; // a quote: "
    let also_doesnt_start =
        /// " */
        'until here,
        test,'; // another quote: "
}

fn foo2() {
    let a = 4; // ///
    let b = '5';
    let c = 6; // ///
}

