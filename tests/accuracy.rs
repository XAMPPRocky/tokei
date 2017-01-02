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
        let (begin, end) = $regex.find(&$text).expect("Couldn't find category");
        $text[begin..end].split_whitespace()
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    }}
}

#[test]
fn languages() {
    use ignore::Walk;
    let walker = Walk::new("./tests/data/").filter(|p| {
        match p {
            &Ok(ref p) => !p.metadata().unwrap().is_dir(),
            _ => false,
        }
    });
    for path in walker {
        let path = path.unwrap();
        let path = path.path().to_str().unwrap();
        let mut languages = Languages::new();
        languages.get_statistics(vec![path], vec![]);
        let mut contents = String::new();
        File::open(path).unwrap().read_to_string(&mut contents).unwrap();


        for (name, language) in languages {
            assert_eq!(get_digit!(LINES, contents), language.lines);
            println!("{} LINES MATCH", name);
            assert_eq!(get_digit!(CODE, contents), language.code);
            println!("{} CODE MATCH", name);
            assert_eq!(get_digit!(COMMENTS, contents), language.comments);
            println!("{} COMMENTS MATCH", name);
            assert_eq!(get_digit!(BLANKS, contents), language.blanks);
            println!("{} BLANKS MATCH", name);
        }
    }
}
