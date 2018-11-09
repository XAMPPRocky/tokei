use std::borrow::Cow;
use std::fmt;
use std::path::Path;
use std::io;
use std::str::FromStr;

use self::LanguageType::*;
use stats::Stats;

use super::syntax::SyntaxCounter;
use FileAccess;

include!(concat!(env!("OUT_DIR"), "/language_type.rs"));

impl LanguageType {
    /// Parses a given `Path` using the `LanguageType`. Returning `Stats`
    /// on success and giving back ownership of PathBuf on error.
    pub fn parse<'a, F>(self, file_access: F) -> io::Result<Stats>
        where F: FileAccess<'a>
    {
        let text = file_access.read_to_string()?;
        Ok(self.parse_from_str(file_access.name(), &text))
    }

    /// Parses the text provided. Returning `Stats` on success.
    pub fn parse_from_str<'a>(self, name: Cow<'a, str>, text: &str) -> Stats
    {
        let lines = text.lines();
        let mut stats = Stats::new(name.to_string());

        if self.is_blank() {
            let count = lines.count();
            stats.lines = count;
            stats.code = count;
            stats
        } else {
            self.parse_lines(lines, stats)
        }
    }

    /// Attempts to parse the line as simply as possible if there are no multi
    /// line comments or quotes. Returns `bool` indicating whether it was
    /// successful or not.
    #[inline]
    fn parse_basic(self, syntax: &SyntaxCounter, line: &str, stats: &mut Stats)
        -> bool
    {
        if syntax.quote.is_some() ||
           !syntax.stack.is_empty() ||
           syntax.important_syntax().any(|s| line.contains(s))
        {
            return false;
        }

        if syntax.line_comments.into_iter()
                               .any(|s| line.as_bytes()
                                            .starts_with(s.as_bytes()))
        {
            stats.comments += 1;
            trace!("Comment No.{}", stats.comments);
        } else {
            stats.code += 1;
            trace!("Code No.{}", stats.code);
        }

        trace!("{}", line);
        trace!("^ Skippable.");

        true
    }

    #[inline]
    fn parse_lines<'a>(self,
                    lines: impl IntoIterator<Item=&'a str>,
                    mut stats: Stats)
        -> Stats
    {
        let mut syntax = SyntaxCounter::new(self);

        for line in lines {

            if line.chars().all(char::is_whitespace) {
                stats.blanks += 1;
                trace!("Blank No.{}", stats.blanks);
                continue;
            }

            // FORTRAN has a rule where it only counts as a comment if it's the
            // first character in the column, so removing starting whitespace
            // could cause a miscount.
            let line = if syntax.is_fortran { line } else { line.trim() };
            let mut ended_with_comments = false;
            let mut had_multi_line = !syntax.stack.is_empty();
            let mut skip = 0;
            macro_rules! skip {
                ($skip:expr) => {{
                    skip = $skip - 1;
                }}
            }

            if self.parse_basic(&syntax, line, &mut stats) {
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

                let is_end_of_quote_or_multi_line =
                    syntax.parse_end_of_quote(window)
                    .or_else(|| syntax.parse_end_of_multi_line(window));

                if let Some(skip_amount) = is_end_of_quote_or_multi_line {
                    ended_with_comments = true;
                    skip!(skip_amount);
                    continue;
                }

                let is_quote_or_multi_line = syntax.parse_quote(window)
                    .or_else(|| syntax.parse_multi_line_comment(window));

                if let Some(skip_amount) = is_quote_or_multi_line {
                    skip!(skip_amount);
                    continue;
                }

                if syntax.parse_line_comment(window) {
                    break 'window;
                }

            }

            trace!("{}", line);

            if ((!syntax.stack.is_empty() || ended_with_comments) && had_multi_line) ||
                (syntax.start_of_comments().any(|comment| line.starts_with(comment)) &&
                 syntax.quote.is_none())
            {
                stats.comments += 1;
                trace!("Comment No.{}", stats.comments);
                trace!("Was the Comment stack empty?: {}", !had_multi_line);
            } else {
                stats.code += 1;
                trace!("Code No.{}", stats.code);
            }
        }

        stats.lines = stats.blanks + stats.code + stats.comments;
        stats
    }
}

