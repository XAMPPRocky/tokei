extern crate ignore;
extern crate regex;
extern crate tokei;

use std::fs;

use once_cell::sync::Lazy;
use regex::Regex;
use tokei::{Config, Languages};

static LINES: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+ lines").unwrap());
static CODE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+ code").unwrap());
static COMMENTS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+ comments").unwrap());
static BLANKS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+ blanks").unwrap());

macro_rules! get_digit {
    ($regex:expr, $text:expr) => {{
        let matched = $regex.find(&$text).expect("Couldn't find category");
        matched
            .as_str()
            .split_whitespace()
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    }};
}

mod config {
    use tokei::*;

    /*
    #[test]
    fn extension_change() {
        use std::collections::HashMap;
        let mut languages = Languages::new();
        let config = Config {
            languages: {
                let mut map = HashMap::new();
                let mut config = LanguageConfig::new();
                config.extensions(vec![String::from("cpp")]);
                map.insert(LanguageType::C, config);

                Some(map)
            },
            ..Config::default()
        };

        languages.get_statistics(&["tests/data/cpp.cpp"], &[], &config);

        if languages.len() != 1 {
            panic!("wrong languages detected: expected just C, found {:?}",
                   languages.into_iter().collect::<Vec<_>>());
        }

        let (name, _) = languages.into_iter().next().unwrap();

        assert_eq!(LanguageType::C, name);
    }
    */

    #[test]
    fn treating_comments_as_code() {
        let mut languages = Languages::new();
        let config = Config {
            treat_doc_strings_as_comments: Some(true),
            ..Config::default()
        };

        languages.get_statistics(&["tests/data/python.py"], &[], &config);

        if languages.len() != 1 {
            panic!(
                "wrong languages detected: expected just Python, found {:?}",
                languages.into_iter().collect::<Vec<_>>()
            );
        }

        let (_, language) = languages.into_iter().next().unwrap();

        assert_eq!(language.lines(), 15);
        assert_eq!(language.blanks, 3);
        assert_eq!(language.comments, 7);
        assert_eq!(language.code, 5);
    }
}

mod no_comments {
    use tokei::*;

    #[test]
    fn bruno() {
        let mut languages = Languages::new();
        let config = Config::default();

        languages.get_statistics(&["tests/no_comments/bruno.bru"], &[], &config);

        if languages.len() != 1 {
            panic!(
                "wrong languages detected: expected just Bruno, found {:?}",
                languages.into_iter().collect::<Vec<_>>()
            );
        }

        let (_, language) = languages.into_iter().next().unwrap();

        assert_eq!(language.lines(), 27);
        assert_eq!(language.blanks, 6);
        assert_eq!(language.comments, 0);
        assert_eq!(language.code, 21);

        let scripts = &language.children[&LanguageType::JavaScript][0];
        assert_eq!(scripts.stats.lines(), 12);
        assert_eq!(scripts.stats.blanks, 1);
        assert_eq!(scripts.stats.comments, 0);
        assert_eq!(scripts.stats.code, 11);
    }
}

include!(concat!(env!("OUT_DIR"), "/tests.rs"));
