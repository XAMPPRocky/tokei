use std::{
    borrow::Cow,
    fmt,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    config::Config,
    language::syntax::SyntaxCounter,
    stats::Stats,
    utils::{ext::SliceExt, fs as fsutils},
};

use encoding_rs_io::DecodeReaderBytesBuilder;
use grep_searcher::LineIter;
use rayon::prelude::*;

use self::LanguageType::*;

include!(concat!(env!("OUT_DIR"), "/language_type.rs"));

impl LanguageType {
    /// Parses a given `Path` using the `LanguageType`. Returning `Stats`
    /// on success and giving back ownership of PathBuf on error.
    pub fn parse(self, path: PathBuf, config: &Config) -> Result<Stats, (io::Error, PathBuf)> {
        let text = {
            let f = match File::open(&path) {
                Ok(f) => f,
                Err(e) => return Err((e, path)),
            };
            let mut s = Vec::new();
            let mut reader = DecodeReaderBytesBuilder::new().build(f);

            if let Err(e) = reader.read_to_end(&mut s) {
                return Err((e, path));
            }
            s
        };

        Ok(self.parse_from_slice(path, &text, config))
    }

    /// Parses the text provided. Returns `Stats` on success.
    pub fn parse_from_str<A: AsRef<str>>(self, path: PathBuf, text: A, config: &Config) -> Stats {
        self.parse_from_slice(path, text.as_ref().as_bytes(), config)
    }

    /// Parses the text provided. Returning `Stats` on success.
    pub fn parse_from_slice<A: AsRef<[u8]>>(
        self,
        path: PathBuf,
        text: A,
        config: &Config,
    ) -> Stats {
        let text = text.as_ref();
        let lines = LineIter::new(b'\n', text);
        let mut stats = Stats::new(path);
        let syntax = SyntaxCounter::new(self);

        if self.is_blank() {
            let count = lines.count();
            stats.lines = count;
            stats.code = count;
            stats
        } else if let Some(end) = syntax
            .shared
            .important_syntax
            .earliest_find(text)
            .and_then(|m| {
                // Get the position of the last line before the important
                // syntax.
                text[..=m.start()]
                    .into_iter()
                    .rev()
                    .position(|&c| c == b'\n')
                    .filter(|&p| p != 0)
                    .map(|p| m.start() - p)
            })
        {
            let (skippable_text, rest) = text.split_at(end + 1);
            let lines = LineIter::new(b'\n', skippable_text);
            let is_fortran = syntax.shared.is_fortran;
            let comments = syntax.shared.line_comments;

            let (mut stats, (code, comments, blanks)) = rayon::join(
                move || self.parse_lines(config, LineIter::new(b'\n', rest), stats, syntax),
                move || {
                    lines
                        .par_bridge()
                        .map(|line| {
                            // FORTRAN has a rule where it only counts as a comment if it's the
                            // first character in the column, so removing starting whitespace
                            // could cause a miscount.
                            let line = if is_fortran { line } else { line.trim() };
                            trace!("{}", String::from_utf8_lossy(line));

                            if line.trim().is_empty() {
                                (0, 0, 1)
                            } else if comments.iter().any(|c| line.starts_with(c.as_bytes())) {
                                (0, 1, 0)
                            } else {
                                (1, 0, 0)
                            }
                        })
                        .reduce(|| (0, 0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2))
                },
            );

            stats.code += code;
            stats.comments += comments;
            stats.blanks += blanks;
            stats.lines += code + comments + blanks;
            stats
        } else {
            self.parse_lines(config, lines, stats, syntax)
        }
    }

    #[inline]
    fn parse_lines<'a>(
        self,
        config: &Config,
        lines: impl IntoIterator<Item = &'a [u8]>,
        mut stats: Stats,
        mut syntax: SyntaxCounter,
    ) -> Stats {
        for line in lines {
            // FORTRAN has a rule where it only counts as a comment if it's the
            // first character in the column, so removing starting whitespace
            // could cause a miscount.
            let line = if syntax.shared.is_fortran {
                line
            } else {
                line.trim()
            };
            trace!("{}", String::from_utf8_lossy(line));

            if line.trim().is_empty() {
                stats.blanks += 1;
                trace!("Blank No.{}", stats.blanks);
                continue;
            } else if syntax.is_plain_mode() && !syntax.shared.important_syntax.is_match(line) {
                trace!("^ Skippable");

                if syntax
                    .shared
                    .line_comments
                    .iter()
                    .any(|c| line.starts_with(c.as_bytes()))
                {
                    stats.comments += 1;
                    trace!("Comment No.{}", stats.comments);
                } else {
                    stats.code += 1;
                    trace!("Code No.{}", stats.code);
                }

                continue;
            }

            let had_multi_line = !syntax.stack.is_empty();
            let mut ended_with_comments = false;
            let mut skip = 0;
            macro_rules! skip {
                ($skip:expr) => {{
                    skip = $skip - 1;
                }};
            }

            'window: for i in 0..line.len() {
                if skip != 0 {
                    skip -= 1;
                    continue;
                }

                ended_with_comments = false;
                let window = &line[i..];

                let is_end_of_quote_or_multi_line = syntax
                    .parse_end_of_quote(window)
                    .or_else(|| syntax.parse_end_of_multi_line(window));

                if let Some(skip_amount) = is_end_of_quote_or_multi_line {
                    ended_with_comments = true;
                    skip!(skip_amount);
                    continue;
                } else if syntax.quote.is_some() {
                    continue;
                }

                let is_quote_or_multi_line = syntax
                    .parse_quote(window)
                    .or_else(|| syntax.parse_multi_line_comment(window));

                if let Some(skip_amount) = is_quote_or_multi_line {
                    skip!(skip_amount);
                    continue;
                }

                if syntax.parse_line_comment(window) {
                    ended_with_comments = true;
                    break 'window;
                }
            }

            trace!("{}", String::from_utf8_lossy(line));

            let is_comments = ((!syntax.stack.is_empty() || ended_with_comments) && had_multi_line)
                || (
                    // If we're currently in a comment or we just ended
                    // with one.
                    syntax.shared.any_comments.is_match(line) && syntax.quote.is_none()
                )
                || ((
                        // If we're currently in a doc string or we just ended
                        // with one.
                        syntax.quote.is_some() ||
                        syntax.shared.doc_quotes.iter().any(|(s, _)| line.starts_with(s.as_bytes()))
                    ) &&
                    // `Some(true)` is import in order to respect the current
                    // configuration.
                    config.treat_doc_strings_as_comments == Some(true) &&
                    syntax.quote_is_doc_quote);

            if is_comments {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_allows_nested() {
        assert!(LanguageType::Rust.allows_nested());
    }
}
