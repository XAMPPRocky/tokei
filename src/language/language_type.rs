include!(concat!(env!("OUT_DIR"), "/language_type.rs"));

struct Syntax {
    is_fortran: bool,
    allows_nested: bool,
    line_comments: &'static [&'static str],
    multi_line_comments: &'static [(&'static str, &'static str)],
    nested_comments: &'static [(&'static str, &'static str)],
    quotes: &'static [(&'static str, &'static str)],
}

impl Syntax {
    fn new(language: LanguageType) -> Self {
        Self {
            is_fortran: language.is_fortran(),
            allows_nested: language.allows_nested(),
            line_comments: language.line_comments(),
            multi_line_comments: language.multi_line_comments(),
            nested_comments: language.nested_comments(),
            quotes: language.quotes(),
        }
    }

    #[inline]
    fn important_syntax(&self) -> impl Iterator<Item = &str> {
        self.quotes.into_iter()
            .map(|(s, _)| *s)
            .chain(self.multi_line_comments.into_iter().map(|(s, _)| *s))
            .chain(self.nested_comments.into_iter().map(|(s, _)| *s))
    }

    #[inline]
    fn start_of_comments(&self) -> impl Iterator<Item = &&str> {
        self.line_comments.into_iter()
            .chain(self.multi_line_comments.into_iter().map(|(s, _)| s))
            .chain(self.nested_comments.into_iter().map(|(s, _)| s))
    }
}

impl LanguageType {
    /// Parses a given `Path` using the `LanguageType`. Returning `Stats`
    /// on success.
    pub fn parse(self, path: PathBuf) -> io::Result<Stats> {
        let text = {
            let f = File::open(&path)?;
            let mut s = String::new();
            let mut reader = DecodeReaderBytes::new(f);

            reader.read_to_string(&mut s)?;
            s
        };

        self.parse_from_str(path, &text)
    }

    /// Parses the text provided. Returning `Stats` on success.
    pub fn parse_from_str(self, path: PathBuf, text: &str)
        -> io::Result<Stats>
    {

        let lines = text.lines();
        let mut stats = Stats::new(path);

        let stats = if self.is_blank() {
            let count = lines.count();
            stats.lines = count;
            stats.code = count;
            stats
        } else {
            self.parse_lines(lines, stats)
        };

        Ok(stats)
    }

    /// Attempts to parse the line as simply as possible if there are no multi
    /// line comments or quotes. Returns `bool` indicating whether it was
    /// successful or not.
    #[inline]
    fn parse_basic(syntax: &Syntax, line: &str, stats: &mut Stats) {
        trace!("Determined to be skippable");
        if syntax.line_comments.iter().any(|s| line.starts_with(s)) {
            stats.comments += 1;
            trace!("Determined to be comment. So far: {} lines", stats.comments);
        } else {
            stats.code += 1;
            trace!("Determined to be code. So far: {} lines", stats.code);
        }

        trace!("{}", line);
    }

    #[inline]
    fn parse_lines<'a>(self,
                    lines: impl IntoIterator<Item=&'a str>,
                    mut stats: Stats)
        -> Stats
    {
        let mut stack: Vec<&'static str> = Vec::with_capacity(1);
        let mut quote: Option<&'static str> = None;
        let syntax = Syntax::new(self);

        for line in lines {

            if line.chars().all(char::is_whitespace) {
                stats.blanks += 1;
                trace!("Blank line. So far: {}", stats.blanks);
                continue;
            }

            // FORTRAN has a rule where it only counts as a comment if it's the
            // first character in the column, so removing starting whitespace
            // could cause a miscount.
            let line = if syntax.is_fortran { line } else { line.trim() };
            let mut ended_with_comments = false;
            let mut no_previous_multi_line = stack.is_empty();
            let mut skip = 0;
            macro_rules! skip {
                ($skip:expr) => {{
                    skip = $skip - 1;
                }}
            }

            if quote.is_none() &&
               no_previous_multi_line &&
               !syntax.important_syntax().any(|s| line.contains(s))
            {
                Self::parse_basic(&syntax, line, &mut stats);
                continue;
            }


            'window: for i in 0..line.len() {
                if skip != 0 {
                    skip -= 1;
                    continue;
                }

                ended_with_comments = false;
                let line = line.as_bytes();
                let window = &line[i..];

                if let Some(quote_str) = quote {
                    if window.starts_with(br"\") {
                        skip = 1;
                    } else if window.starts_with(quote_str.as_bytes()) {
                        quote = None;
                        trace!(r#"End of "{}"."#, quote_str);
                        skip!(quote_str.len());
                    }
                    continue;
                }

                if stack.last().map_or(false, |l| window.starts_with(l.as_bytes())) {
                    let last = stack.pop().unwrap();
                    ended_with_comments = true;

                    if log_enabled!(Trace) && stack.is_empty() {
                        trace!(r#"End of "{}"."#, last);
                    } else {
                        trace!(r#"End of "{}". Still in comments."#, last);
                    }

                    skip!(last.len());
                    continue;
                }

                if stack.is_empty() {
                    for comment in syntax.line_comments {
                        if window.starts_with(comment.as_bytes()) {
                            trace!(r#"Start of "{}"."#, comment);
                            break 'window;
                        }
                    }

                    for &(start, end) in syntax.quotes {
                        if window.starts_with(start.as_bytes()) {
                            quote = Some(end);
                            trace!(r#"Start of "{}"."#, start);
                            skip!(start.len());
                            continue 'window;
                        }
                    }
                }

                for &(start, end) in syntax.nested_comments {
                    if window.starts_with(start.as_bytes()) {
                        stack.push(end);
                        trace!(r#"Start of "{}"."#, start);
                        skip!(start.len());
                        continue 'window;
                    }
                }

                for &(start, end) in syntax.multi_line_comments {
                    if window.starts_with(start.as_bytes()) {
                        if syntax.allows_nested || stack.is_empty() {
                            stack.push(end);

                            if log_enabled!(Trace) && syntax.allows_nested {
                                trace!(r#"Start of nested "{}"."#, start);
                            } else {
                                trace!(r#"Start of "{}"."#, start);
                            }

                        }

                        skip!(start.len());
                        continue 'window;
                    }
                }
            }

            trace!("{}", line);

            if ((!stack.is_empty() || ended_with_comments) && !no_previous_multi_line) ||
                (syntax.start_of_comments().any(|comment| line.starts_with(comment)) &&
                 quote.is_none())
            {
                stats.comments += 1;
                trace!("Determined to be comment. So far: {} lines", stats.comments);
                trace!("Did the previous line have a multi line?: {}", no_previous_multi_line);
            } else {
                stats.code += 1;
                trace!("Determined to be code. So far: {} lines", stats.code);
            }
        }

        stats.lines = stats.blanks + stats.code + stats.comments;
        stats
    }
}

