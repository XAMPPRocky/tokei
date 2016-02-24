// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

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
