use language::Language;

/// This is used to catch lines like "let x = 5; /* Comment */"
pub fn handle_multi_line(line: &str,
                         language: &Language,
                         stack: &mut Vec<&'static str>,
                         quote: &mut Option<&'static str>) {
    let mut chars = line.chars();
    let mut cont = false;
    let nested_is_empty = language.nested_comments.is_empty();

    'window: loop {
        let window = chars.as_str();
        if window.is_empty() {
            break;
        }
        chars.next();

        // Prevents counting overlaps like /*/*
        if cont {
            cont = false;
            continue;
        }

        let mut end = false;

        if let &mut Some(quote_str) = quote {
            if window.starts_with("\\") {
                cont = true;
                continue;
            } else if window.starts_with(quote_str) {
                end = true;
            }
        }

        if end {
            if let &mut Some(quote_str) = quote {
                *quote = None;

                if quote_str.chars().count() == 1 {
                    cont = true
                }
                continue;
            }
        }

        if quote.is_some() {
            continue;
        }

        let mut pop = false;
        if let Some(last) = stack.last() {
            if window.starts_with(last) {
                pop = true;
            }
        }

        if pop {
            stack.pop();
            cont = true;
            continue;
        }


        if stack.is_empty() {
            for &(start, end) in &language.quotes {
                if window.starts_with(start) {
                    *quote = Some(end);
                    cont = true;
                    continue 'window;
                }
            }
        }


        for comment in &language.line_comment {
            if window.starts_with(comment) {
                break 'window;
            }
        }

        for &(start, end) in &language.nested_comments {
            if window.starts_with(start) {
                stack.push(end);
                cont = true;
                continue 'window;
            }
        }

        for &(start, end) in &language.multi_line {
            if window.starts_with(start) {
                if language.nested && nested_is_empty {
                    stack.push(end);
                } else if stack.len() == 0 {
                    stack.push(end);
                }
                cont = true;
                continue 'window;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use language::Language;

    #[test]
    fn both_comments_in_line() {
        let mut stack = vec![];
        let mut quote = None;
        let language = Language::new_c();
        handle_multi_line("Hello /* /* */ World", &language, &mut stack, &mut quote);
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn comment_hidden_in_single() {
        let mut stack = vec![];
        let mut quote = None;
        let language = Language::new_c();
        handle_multi_line("Hello World // /*", &language, &mut stack, &mut quote);
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn comment_start() {
        let mut stack = vec![];
        let mut quote = None;
        let language = Language::new_c();
        handle_multi_line("/*Hello World", &language, &mut stack, &mut quote);
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn both_comments_in_line_nested() {
        let mut stack = vec![];
        let mut quote = None;
        let language = Language::new_func().nested();
        handle_multi_line("Hello (* (* *) World", &language, &mut stack, &mut quote);
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn comments_of_uneven_length() {
        let mut stack = vec![];
        let mut quote = None;
        let language = Language::new(vec![], vec![("\\<open>", "\\<close>")]).nested();
        handle_multi_line("Hello \\<open> \\<open> \\<close> World",
                          &language,
                          &mut stack,
                          &mut quote);
        assert_eq!(stack.len(), 1);
    }
}
