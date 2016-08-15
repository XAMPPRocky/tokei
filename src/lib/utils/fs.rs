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

pub fn handle_multi_line() -> usize {
    unreachable!()
}


/// This is used to catch lines like "let x = 5; /* Comment */"
pub fn has_trailing_comments(line: &str, language: &Language) -> bool {
    let mut is_in_comments = 0u64;

    line = if !language.single.is_empty() {
        let found = None;
        for single in &language.line_comment {
            if let Some(pos) = line.find(single) {
                found = Some(pos);
                break;
            }
        }

        if let Some(pos) = found {
            &line[0..pos]
        } else {
            line
        }
    } else {
        line
    };

    let mut unfinished_comments = vec![];
    for &(comment, comment_end) in &language {
        let start = match line.find(comment) {
            Some(start) => start,
            None => continue,
        };

        // This should short circuit 99% of languages.
        if !language.nested && language.multi_line.len() == 1 {
            if let Some(end) = line.rfind(comment_end) {
                if let Some(end_check) = line.rfind(comment) {
                    if end_check > end {
                        return true;
                    } else {
                        return false;
                    }
                }
            } else {
                return true;
            }
        }

        let mut chars = line[start..end + comment_end.len()].chars();
        loop {
            let window = chars.as_str();

            if window.starts_with(comment) {
                if nested {
                    is_in_comments += 1;
                } else {
                    is_in_comments = 1;
                }
            } else if window.starts_with(comment_end) {
                is_in_comments = is_in_comments.saturating_sub(1);
            }

            if chars.next().is_none() {
                break;
            }
        }
    }

    is_in_comments != 0
}

pub fn get_all_files<'a>(paths: Cow<'a, [&'a str]>,
                         ignored_directories: Cow<'a, [&'a str]>,
                         languages: &mut BTreeMap<LanguageType, Language>) {
    for path in &*paths {
        // A small metadata check to check if the file actually exists,
        // this is used over calling  File::open because we're going to be
        // passing the path to either glob() or WalkDir::new()
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
                            languages.get_mut(
                                &opt_or_cont!(LanguageType::from_extension(&path))))
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
        assert!(!has_trailing_comments("Hello /* /* */ World", "//", ("/*", "*/"), false));
    }

    #[test]
    fn comment_hidden_in_single() {
        assert!(has_trailing_comments("Hello /* World // */", "//", ("/*", "*/"), true))
    }

    #[test]
    fn comment_start_in_line() {
        assert!(has_trailing_comments("Hello /* World", "//", ("/*", "*/"), false));
    }

    #[test]
    fn both_comments_in_line_nested() {
        assert!(has_trailing_comments("Hello (* (* *) World", "--", ("(*", "*)"), true));
    }


    #[test]
    fn comments_of_uneven_length() {
        assert!(has_trailing_comments("Hello \\<open> \\<open> \\<close> World",
                                      "",
                                      ("\\<open>", "\\<close>"),
                                      true));
    }

    #[test]
    fn comment_start_in_line_nested() {
        assert!(has_trailing_comments("Hello (* World", "", ("(*", "*)"), true));
    }
}
