// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the APACHE2.0/MIT licence that can be
// found in the LICENCE-{APACHE/MIT} file.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

use glob::glob;
use walkdir::{WalkDir, WalkDirIterator};

use language::{Language, LanguageType};
use language::LanguageType::*;

/// This is used to catch lines like "let x = 5; /* Comment */"
pub fn has_trailing_comments(line: &str,
                             comment: &'static str,
                             comment_end: &'static str,
                             nested: bool)
                             -> bool {
    let mut in_comments: usize = 0;
    for chars in line.chars().collect::<Vec<char>>().windows(comment.len()) {
        let window = {
            let mut window = String::new();
            for ch in chars {
                window.push(*ch);
            }
            window
        };

        if window == comment {
            if nested {
                in_comments += 1;
            } else {
                in_comments = 1;
            }
            continue;
        } else if window == comment_end {
            if nested && in_comments != 0 {
                in_comments -= 1;
            } else {
                in_comments = 0;
            }
            continue;
        }
    }
    in_comments != 0
}

pub fn get_all_files<'a>(paths: Cow<'a, [&'a str]>,
                         ignored_directories: Cow<'a, [&'a str]>,
                         languages: &mut BTreeMap<LanguageType, Language>) {
    for path in &*paths {
        // A small metadata check to check if the file actually exists, this is used over calling
        // File::open because we're going to be passing the path to either glob() or WalkDir::new()
        if let Err(_) = Path::new(path).metadata() {
            if let Ok(paths) = glob(path) {
                'path: for path in paths {
                    let path = rs_or_cont!(path);

                    for ig in &*ignored_directories {
                        if opt_or_cont!(path.to_str()).contains(ig) {
                            continue 'path;
                        }
                    }
                    let mut language = if opt_or_cont!(path.to_str()).contains("Makefile") {
                        languages.get_mut(&Makefile).unwrap()
                    } else {
                        opt_or_cont!(
                            languages.get_mut(&opt_or_cont!(LanguageType::from_extension(&path))))
                    };

                    language.files.push(path.to_owned());
                }
            }
        } else {
            let walker = WalkDir::new(path).into_iter().filter_entry(|entry| {
                for ig in &*ignored_directories {
                    if entry.path().to_str().unwrap().contains(&*ig) {
                        return false;
                    }
                }
                true
            });

            for entry in walker {
                let entry = rs_or_cont!(entry);

                let mut language = if opt_or_cont!(entry.path().to_str()).contains("Makefile") {
                    languages.get_mut(&Makefile).unwrap()
                } else {
                    opt_or_cont!(
                        languages.get_mut(&opt_or_cont!(LanguageType::from_extension(entry.path())))
                        )
                };

                language.files.push(entry.path().to_owned());
            }
        }
    }
}

pub fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
    let path = path.as_ref();
    match path.extension() {
        Some(extension_os) => {
            match extension_os.to_str() {
                Some(extension) => Some(extension.to_lowercase()),
                None => None,
            }
        }
        None => {
            match get_filetype_from_shebang(path) {
                // Using String::from here because all file extensions from
                // get_filetype_from_shebang are guaranteed to be lowercase.
                Some(extension) => Some(String::from(extension)),
                None => None,
            }
        }
    }

}
/// This is for getting the file type from the first line of a file
pub fn get_filetype_from_shebang<P: AsRef<Path>>(file: P) -> Option<&'static str> {
    let file = match File::open(file) {
        Ok(file) => file,
        _ => return None,
    };
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let _ = buf.read_line(&mut line);

    let mut words = line.split_whitespace();
    match words.next() {
        Some("#!/bin/sh") => Some("sh"),
        Some("#!/bin/csh") => Some("csh"),
        Some("#!/usr/bin/perl") => Some("pl"),
        Some("#!/usr/bin/env") => {
            match words.next() {
                Some("python") | Some("python2") | Some("python3") => Some("py"),
                Some("sh") => Some("sh"),
                _ => None,
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_comments_in_line() {
        assert!(!has_trailing_comments("Hello /* /* */ World", "/*", "*/", false));
    }

    #[test]
    fn comment_start_in_line() {
        assert!(has_trailing_comments("Hello /* World", "/*", "*/", false));
    }

    #[test]
    fn both_comments_in_line_nested() {
        assert!(has_trailing_comments("Hello (* (* *) World", "(*", "*)", true));
    }

    #[test]
    fn comment_start_in_line_nested() {
        assert!(has_trailing_comments("Hello (* World", "(*", "*)", true));
    }
}
