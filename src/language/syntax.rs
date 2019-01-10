use log::Level::Trace;

use super::language_type::LanguageType;

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
        self.quotes.into_iter()
            .map(|(s, _)| *s)
            .chain(self.doc_quotes.into_iter().map(|(s, _)| *s))
            .chain(self.multi_line_comments.into_iter().map(|(s, _)| *s))
            .chain(self.nested_comments.into_iter().map(|(s, _)| *s))
    }

    #[inline]
    pub(crate) fn start_of_comments(&self) -> impl Iterator<Item = &&str> {
        self.line_comments.into_iter()
            .chain(self.multi_line_comments.into_iter().map(|(s, _)| s))
            .chain(self.nested_comments.into_iter().map(|(s, _)| s))
    }

    #[inline]
    pub(crate) fn parse_line_comment(&self, window: &[u8]) -> bool {
        if self.quote.is_some() || !self.stack.is_empty() {
            return false
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
            return None
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

    #[inline]
    pub(crate) fn parse_multi_line_comment(&mut self, window: &[u8])
        -> Option<usize>
    {
        if self.quote.is_some() {
            return None
        }

        let iter = self.multi_line_comments.into_iter()
                                          .chain(self.nested_comments);
        for &(start, end) in iter {
            if window.starts_with(start.as_bytes()) {
                if self.stack.is_empty() ||
                   self.allows_nested ||
                   self.nested_comments.contains(&(start, end))
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
    pub(crate) fn parse_end_of_quote(&mut self, window: &[u8]) -> Option<usize>
    {
        if self.quote.map_or(false, |q| window.starts_with(q.as_bytes())) {
            let quote = self.quote.take().unwrap();
            trace!("End {:?}", quote);
            Some(quote.len())
        } else if window.starts_with(br"\") {
            // Tell the state machine to skip the next character because it has
            // been escaped.
             Some(2)
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn parse_end_of_multi_line(&mut self, window: &[u8])
        -> Option<usize>
    {
        if self.stack.last().map_or(false, |l| window.starts_with(l.as_bytes()))
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

