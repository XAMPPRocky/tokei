// 8 lines 6 code 1 comments 1 blanks
fn quote(s: &str) -> bool {
    s.starts_with('"')
}

fn lifetime<'a>(x: &'a str) -> &'a str {
    x
}
