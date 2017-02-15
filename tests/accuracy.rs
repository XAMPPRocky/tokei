#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate tokei;
extern crate ignore;

use std::io::Read;
use std::fs::File;

use regex::Regex;
use tokei::Languages;

lazy_static! {
    static ref LINES: Regex = Regex::new(r"\d+ lines").unwrap();
    static ref CODE: Regex = Regex::new(r"\d+ code").unwrap();
    static ref COMMENTS: Regex = Regex::new(r"\d+ comments").unwrap();
    static ref BLANKS: Regex = Regex::new(r"\d+ blanks").unwrap();
}

macro_rules! get_digit {
    ($regex:expr, $text:expr) => {{
        let matched = $regex.find(&$text).expect("Couldn't find category");
        matched.as_str().split_whitespace()
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    }}
}

include!(concat!(env!("OUT_DIR"), "/tests.rs"));
