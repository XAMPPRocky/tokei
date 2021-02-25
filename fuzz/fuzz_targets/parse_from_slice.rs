use arbitrary::Arbitrary;
use std::str;

use tokei::{Config, LanguageType};

#[derive(Arbitrary, Debug)]
pub struct FuzzInput<'a> {
    lang: LanguageType,
    data: &'a [u8],
}

// The first byte of data is used to select a language; remaining input is parsed
// If check_total is true, asserts that the parsed stats pass a basic sanity test
pub fn parse_from_slice(input: FuzzInput, check_total: bool) {
    let config = &Config {
        treat_doc_strings_as_comments: Some(true),
        ..Config::default()
    };

    // check that parsing doesn't panic
    let stats = input.lang.parse_from_slice(input.data, config);

    if check_total {
        // verify that the parsed total lines is not more than the total occurences of \n and \r\n.
        // if/when all of the current discrepancies are fixed, we could make this stronger by checking it is equal.
        if let Ok(s) = str::from_utf8(input.data) {
            assert!(
            stats.lines() <= s.lines().count(),
            "{} got more total lines ({}) than str::lines ({}). Code: {}, Comments: {}, Blanks: {}. File contents (as UTF-8):\n{}",
            input.lang.name(),
            stats.lines(),
            s.lines().count(),
            stats.code,
            stats.comments,
            stats.blanks,
            s
        )
        };
    }
}
