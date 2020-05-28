use std::sync::Arc;

use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use dashmap::DashMap;
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
/// - `comment` mode: This when the state machine is current inside a comment
///   for a given language, strings cannot trigger `string` mode while in
///   `comment` mode.
#[derive(Clone, Debug)]
pub(crate) struct SyntaxCounter {
    pub(crate) shared: Arc<SharedMatchers>,
    pub(crate) quote: Option<&'static str>,
    pub(crate) quote_is_doc_quote: bool,
    pub(crate) stack: Vec<&'static str>,
    pub(crate) quote_is_verbatim: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct SharedMatchers {
    pub allows_nested: bool,
    pub doc_quotes: &'static [(&'static str, &'static str)],
    pub important_syntax: AhoCorasick<u16>,
    pub any_comments: AhoCorasick<u16>,
    pub is_fortran: bool,
    pub line_comments: &'static [&'static str],
    pub multi_line_comments: &'static [(&'static str, &'static str)],
    pub nested_comments: &'static [(&'static str, &'static str)],
    pub string_literals: &'static [(&'static str, &'static str)],
    pub verbatim_string_literals: &'static [(&'static str, &'static str)],
}

impl SharedMatchers {
    pub fn new(language: LanguageType) -> Arc<Self> {
        lazy_static::lazy_static! {
            pub(crate) static ref MATCHERS: DashMap<LanguageType, Arc<SharedMatchers>> = DashMap::new();
        }

        MATCHERS
            .entry(language)
            .or_insert_with(|| Arc::new(Self::init(language)))
            .value()
            .clone()
    }

    pub fn init(language: LanguageType) -> Self {
        fn init_corasick(pattern: &[&'static str], anchored: bool) -> AhoCorasick<u16> {
            let mut builder = AhoCorasickBuilder::new();
            builder
                .anchored(anchored)
                .byte_classes(false)
                .dfa(true)
                .prefilter(true);
            builder.build_with_size(pattern).unwrap()
        }

        Self {
            allows_nested: language.allows_nested(),
            doc_quotes: language.doc_quotes(),
            is_fortran: language.is_fortran(),
            important_syntax: init_corasick(language.important_syntax(), false),
            any_comments: init_corasick(language.start_any_comments(), true),
            line_comments: language.line_comments(),
            multi_line_comments: language.multi_line_comments(),
            nested_comments: language.nested_comments(),
            string_literals: language.quotes(),
            verbatim_string_literals: language.verbatim_quotes(),
        }
    }
}

impl SyntaxCounter {
    pub(crate) fn new(language: LanguageType) -> Self {
        Self {
            shared: SharedMatchers::new(language),
            quote_is_doc_quote: false,
            quote_is_verbatim: false,
            stack: Vec::with_capacity(1),
            quote: None,
        }
    }

    #[inline]
    pub(crate) fn parse_line_comment(&self, window: &[u8]) -> bool {
        if self.quote.is_some() || !self.stack.is_empty() {
            return false;
        }

        if let Some(comment) = self
            .shared
            .line_comments
            .iter()
            .find(|c| window.starts_with(c.as_bytes()))
        {
            trace!("Start {:?}", comment);
            return true;
        }

        false
    }

    #[inline]
    pub(crate) fn parse_quote(&mut self, window: &[u8]) -> Option<usize> {
        if !self.stack.is_empty() {
            return None;
        }

        if let Some((start, end)) = self
            .shared
            .doc_quotes
            .iter()
            .find(|(s, _)| window.starts_with(s.as_bytes()))
        {
            trace!("Start Doc {:?}", start);
            self.quote = Some(end);
            self.quote_is_verbatim = false;
            self.quote_is_doc_quote = true;
            return Some(start.len());
        }

        if let Some((start, end)) = self
            .shared
            .verbatim_string_literals
            .iter()
            .find(|(s, _)| window.starts_with(s.as_bytes()))
        {
            trace!("Start verbatim {:?}", start);
            self.quote = Some(end);
            self.quote_is_verbatim = true;
            self.quote_is_doc_quote = false;
            return Some(start.len());
        }

        if let Some((start, end)) = self
            .shared
            .string_literals
            .iter()
            .find(|(s, _)| window.starts_with(s.as_bytes()))
        {
            trace!("Start {:?}", start);
            self.quote = Some(end);
            self.quote_is_verbatim = false;
            self.quote_is_doc_quote = false;
            return Some(start.len());
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

        let iter = self
            .shared
            .multi_line_comments
            .iter()
            .chain(self.shared.nested_comments);
        for &(start, end) in iter {
            if window.starts_with(start.as_bytes()) {
                if self.stack.is_empty()
                    || self.shared.allows_nested
                    || self.shared.nested_comments.contains(&(start, end))
                {
                    self.stack.push(end);

                    if log_enabled!(Trace) && self.shared.allows_nested {
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
        if window.starts_with(self.quote?.as_bytes()) {
            let quote = self.quote.take().unwrap();
            trace!("End {:?}", quote);
            Some(quote.len())
        } else if window.starts_with(br"\") && !self.quote_is_verbatim {
            // Tell the state machine to skip the next character because it
            // has been escaped if the string isn't a verbatim string.
            Some(2)
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
