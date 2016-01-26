// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

extern crate glob;

use std::fs;
use std::fs::metadata;
use self::glob::glob;

pub fn contains_comments(file: &str, comment: &str, comment_end: &str) -> bool {
    let mut in_comments: usize = 0;
    'window: for chars in file.chars().collect::<Vec<char>>().windows(comment.len()) {
        let section = {
            let mut section = String::new();
            for ch in chars {
                section.push(*ch);
            }
            section
        };

        if section == comment {
            in_comments += 1;
            continue 'window;
        } else if section == comment_end {
            if in_comments != 0 {
                in_comments -= 1;
            }
            continue 'window;
        }
    }
    in_comments != 0
}

pub fn get_all_files(path: String, ignored_directories: &[String]) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    if let Ok(result) = metadata(&path) {
        if result.is_dir() {
            let dir = match fs::read_dir(&path) {
                Ok(val) => val,
                Err(_) => return Vec::new(),
            };
            'file: for entry in dir {
                let entry = unwrap_rs_cont!(entry);
                let file_path = entry.path();
                let file_str = unwrap_opt_cont!(file_path.to_str());
                let file_string = file_str.to_owned();
                let path_metadata = unwrap_rs_cont!(metadata(file_str));

                for ignored_directory in ignored_directories {
                    if file_str.contains(ignored_directory) {
                        continue 'file;
                    }
                }

                if path_metadata.is_dir() {
                    for file in get_all_files(file_string, ignored_directories) {
                        files.push(file);
                    }
                } else if path_metadata.is_file() {
                    files.push(file_string);
                }
            }
        } else {
            files.push(path);
        }
    } else {
        let glob = glob(&path);
        match glob {
            Ok(iter) => {
                for path_buf in iter {
                    let file_path = unwrap_opt_cont!(unwrap_rs_cont!(path_buf).as_path().to_str())
                                        .to_owned();
                    files.push(file_path);
                }
            }
            Err(error) => {
                panic!("The path provided wasn't valid. PATH:{:?}, error:{:?}",
                       path,
                       error);
            }
        }
    }

    files
}

#[allow(dead_code, unused_imports)]
mod tests {
    use super::*;
    #[test]
    fn comment_start_in_quotes() {
        assert!(contains_comments("Hello \"/*\" World", "/*", "*/"));
    }

    #[test]
    fn both_comments_in_quotes() {
        assert!(!contains_comments("Hello \"/**/\" World", "/*", "*/"));
    }

    #[test]
    fn both_comments_in_line() {
        assert!(!contains_comments("Hello /**/ World", "/*", "*/"));
    }

    #[test]
    fn comment_start_in_line() {
        assert!(contains_comments("Hello /* World", "/*", "*/"));
    }
}
