use arbitrary::Arbitrary;
use std::str;

use tokei::{Config, LanguageType};

#[derive(Arbitrary, Debug)]
pub struct FuzzInput<'a> {
    lang: LanguageType,
    treat_doc_strings_as_comments: bool,
    data: &'a [u8],
}

// The first byte of data is used to select a language; remaining input is parsed
// If check_total is true, asserts that the parsed stats pass a basic sanity test
pub fn parse_from_slice(input: FuzzInput, check_total: bool) {
    let config = &Config {
        treat_doc_strings_as_comments: Some(input.treat_doc_strings_as_comments),

        // these options don't impact the behaviour of parse_from_slice:
        columns: None,
        hidden: None,
        no_ignore: None,
        no_ignore_parent: None,
        no_ignore_dot: None,
        no_ignore_vcs: None,
        sort: None,
        types: None,
        for_each_fn: None,
    };

    // check that parsing doesn't panic
    let stats = input.lang.parse_from_slice(input.data, config);

    if check_total {
        // verify that the parsed total lines is not more than the total occurrences of \n and \r\n.
        // if/when all of the current discrepancies are fixed, we could make this stronger by checking it is equal.
        if let Ok(s) = str::from_utf8(input.data) {
            assert!(
            stats.lines() <= s.lines().count(),
            "{} got more total lines ({}) than str::lines ({}). Code: {}, Comments: {}, Blanks: {}. treat_doc_strings_as_comments: {}. File contents (as UTF-8):\n{}",
            input.lang.name(),
            stats.lines(),
            s.lines().count(),
            stats.code,
            stats.comments,
            input.treat_doc_strings_as_comments,
            stats.blanks,
            s
        )
        };
    }
}
