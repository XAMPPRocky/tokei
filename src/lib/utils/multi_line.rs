use std::cmp;

/// This is used to catch lines like "let x = 5; /* Comment */"
pub fn has_trailing_comments(line: &str, language: &Language) -> Vec<&'static str> {
    let line = slice_to_single(line, language);
    let mut is_in_comments = 0u64;
    let mut start = None;
    let mut stack = vec![];

    for &(comment, comment_end) in &language.multi_line {
        start = line.find(comment).and_then(|x| cmp::min(x, start.unwrap_or(x)));

        // This should short circuit 99% of languages.
        if start.is_none() && !language.nested && language.multi_line.len() == 1 {
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
    }

    let start = match start {
        Some(pos) => pos,
        None => return stack,
    };

    let mut chars = line[start..].chars();
    let mut cont = false;
    loop {
        let window = chars.as_str();

        // Prevents counting overlaps like /*/*
        if cont {
            cont = false;
            continue;
        }

        if let Some(last) = stack.last() {
            if window.starts_with(last) {
                stack.pop();
                cont = true;
                continue;
            }
        }

        for &(comment, comment_end) in &language.multi_line {
            if window.starts_with(comment) {
                if nested {
                    stack.push(comment_end);
                } else if stack.len() == 0 {
                    stack.push(comment_end);
                }
                cont = true;
                continue;
            }
        }

        if chars.next().is_none() {
            break;
        }
    }

    stack
}

#[inline]
fn slice_to_single(line: &str, language: &language) -> &str {
    if !language.single.is_empty() {
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
