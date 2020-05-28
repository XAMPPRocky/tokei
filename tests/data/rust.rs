// 41 lines 33 code 3 comments 5 blanks

/* /**/ */
fn main() {
    let start = r##"/*##\"
\"##;
    // comment
    loop {
        if x.len() >= 2 && x[0] == '*' && x[1] == '/' { // found the */
            break;
        }
    }
}

fn foo<'a, 'b>(name: &'b str) {
    let this_ends = "a \"test/*.";
    call1();
    call2();
    let this_does_not = /* a /* nested */ comment " */
        "*/another /*test
            call3();
            */";
}

fn foobar() {
    let does_not_start = // "
        "until here,
        test/*
        test"; // a quote: "
    let also_doesnt_start = /* " */
        "until here,
        test,*/
        test"; // another quote: "
}

fn foo() {
    let a = 4; // /*
    let b = 5;
    let c = 6; // */
}

