use log::Level::Trace;

use super::language_type::LanguageType;

/// Tracks the syntax of the language as well as the current state in the file.
/// Current has what could be consider three types of mode.
/// - `plain` mode: This is the normal state, blanks are counted as blanks,
///   string literals can trigger `string` mode, and comments can trigger
///   `comment` mode.
/// - `string` mode: This when the state machine is current inside a string
///   literal for a given language, comments cannot trigger `comment` mode while
///   in `string` mode.
/// - `string` mode: This when the state machine is current inside a comment
///   for a given language, strings cannot trigger `string` mode while in
///   `comment` mode.
#[derive(Clone, Debug)]
pub(crate) struct SyntaxCounter {
    pub(crate) allows_nested: bool,
    pub(crate) doc_quotes: &'static [(&'static str, &'static str)],
    pub(crate) is_fortran: bool,
    pub(crate) line_comments: &'static [&'static str],
    pub(crate) multi_line_comments: &'static [(&'static str, &'static str)],
    pub(crate) nested_comments: &'static [(&'static str, &'static str)],
    pub(crate) quote: Option<&'static str>,
    pub(crate) quote_is_doc_quote: bool,
    pub(crate) quotes: &'static [(&'static str, &'static str)],
    pub(crate) stack: Vec<&'static str>,
}

impl SyntaxCounter {
    pub(crate) fn new(language: LanguageType) -> Self {
        Self {
            allows_nested: language.allows_nested(),
            doc_quotes: language.doc_quotes(),
            is_fortran: language.is_fortran(),
            line_comments: language.line_comments(),
            multi_line_comments: language.multi_line_comments(),
            nested_comments: language.nested_comments(),
            quote_is_doc_quote: false,
            quotes: language.quotes(),
            stack: Vec::with_capacity(1),
            quote: None,
        }
    }

    #[inline]
    pub(crate) fn important_syntax(&self) -> impl Iterator<Item = &str> {
        self.quotes
            .iter()
            .map(|(s, _)| *s)
            .chain(self.doc_quotes.iter().map(|(s, _)| *s))
            .chain(self.multi_line_comments.iter().map(|(s, _)| *s))
            .chain(self.nested_comments.iter().map(|(s, _)| *s))
    }

    #[inline]
    pub(crate) fn start_of_comments(&self) -> impl Iterator<Item = &&str> {
        self.line_comments
            .iter()
            .chain(self.multi_line_comments.iter().map(|(s, _)| s))
            .chain(self.nested_comments.iter().map(|(s, _)| s))
    }

    #[inline]
    pub(crate) fn parse_line_comment(&self, window: &[u8]) -> bool {
        if self.quote.is_some() || !self.stack.is_empty() {
            return false;
        }

        for comment in self.line_comments {
            if window.starts_with(comment.as_bytes()) {
                trace!("Start {:?}", comment);
                return true;
            }
        }

        false
    }

    #[inline]
    pub(crate) fn parse_quote(&mut self, window: &[u8]) -> Option<usize> {
        if !self.stack.is_empty() {
            return None;
        }

        for &(start, end) in self.doc_quotes {
            if window.starts_with(start.as_bytes()) {
                trace!("Start Doc {:?}", start);
                self.quote = Some(end);
                self.quote_is_doc_quote = true;
                return Some(start.len());
            }
        }

        for &(start, end) in self.quotes {
            if window.starts_with(start.as_bytes()) {
                trace!("Start {:?}", start);
                self.quote = Some(end);
                self.quote_is_doc_quote = false;
                return Some(start.len());
            }
        }

        None
    }

    /// Returns whether the syntax is currently in plain mode.
    pub(crate) fn is_plain_mode(&self) -> bool {
        self.quote.is_none() && self.stack.is_empty()
    }

    /// Returns whether the syntax is currently in string mode.
    pub(crate) fn _is_string_mode(&self) -> bool {
        self.quote.is_some()
    }

    /// Returns whether the syntax is currently in comment mode.
    pub(crate) fn _is_comment_mode(&self) -> bool {
        !self.stack.is_empty()
    }

    #[inline]
    pub(crate) fn parse_multi_line_comment(&mut self, window: &[u8]) -> Option<usize> {
        if self.quote.is_some() {
            return None;
        }

        let iter = self.multi_line_comments.iter().chain(self.nested_comments);
        for &(start, end) in iter {
            if window.starts_with(start.as_bytes()) {
                if self.stack.is_empty()
                    || self.allows_nested
                    || self.nested_comments.contains(&(start, end))
                {
                    self.stack.push(end);

                    if log_enabled!(Trace) && self.allows_nested {
                        trace!("Start nested {:?}", start);
                    } else {
                        trace!("Start {:?}", start);
                    }
                }

                return Some(start.len());
            }
        }

        None
    }

    #[inline]
    pub(crate) fn parse_end_of_quote(&mut self, window: &[u8]) -> Option<usize> {
        if let Some(quote) = self.quote {
            if window.starts_with(quote.as_bytes()) {
                let quote = self.quote.take().unwrap();
                trace!("End {:?}", quote);
                Some(quote.len())
            } else if window.starts_with(br"\") {
                // Tell the state machine to skip the next character because it
                // has been escaped.
                Some(2)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn parse_end_of_multi_line(&mut self, window: &[u8]) -> Option<usize> {
        if self
            .stack
            .last()
            .map_or(false, |l| window.starts_with(l.as_bytes()))
        {
            let last = self.stack.pop().unwrap();
            if log_enabled!(Trace) && self.stack.is_empty() {
                trace!("End {:?}", last);
            } else {
                trace!("End {:?}. Still in comments.", last);
            }

            Some(last.len())
        } else {
            None
        }
    }
}
