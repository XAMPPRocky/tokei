use std::sync::Arc;

use aho_corasick::AhoCorasick;
use dashmap::DashMap;
use grep_searcher::LineStep;
use log::Level::Trace;
use once_cell::sync::Lazy;

use super::embedding::{
    RegexCache, RegexFamily, ENDING_LF_BLOCK_REGEX, ENDING_MARKDOWN_REGEX, END_SCRIPT, END_STYLE,
    END_TEMPLATE,
};
use crate::LanguageType::LinguaFranca;
use crate::{stats::CodeStats, utils::ext::SliceExt, Config, LanguageType};

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
    pub(crate) lf_embedded_language: Option<LanguageType>,
}

#[derive(Clone, Debug)]
pub(crate) struct FileContext {
    pub(crate) language: LanguageContext,
    pub(crate) stats: CodeStats,
    pub(crate) end: usize,
}

impl FileContext {
    pub fn new(language: LanguageContext, end: usize, stats: CodeStats) -> Self {
        Self {
            language,
            stats,
            end,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum LanguageContext {
    Html {
        language: LanguageType,
    },
    LinguaFranca,
    Markdown {
        balanced: bool,
        language: LanguageType,
    },
    Rust,
}

#[derive(Clone, Debug)]
pub(crate) struct SharedMatchers {
    pub language: LanguageType,
    pub allows_nested: bool,
    pub doc_quotes: &'static [(&'static str, &'static str)],
    pub important_syntax: AhoCorasick,
    #[allow(dead_code)]
    pub any_comments: &'static [&'static str],
    pub is_fortran: bool,
    pub is_literate: bool,
    pub line_comments: &'static [&'static str],
    pub any_multi_line_comments: &'static [(&'static str, &'static str)],
    pub multi_line_comments: &'static [(&'static str, &'static str)],
    pub nested_comments: &'static [(&'static str, &'static str)],
    pub string_literals: &'static [(&'static str, &'static str)],
    pub verbatim_string_literals: &'static [(&'static str, &'static str)],
}

impl SharedMatchers {
    pub fn new(language: LanguageType) -> Arc<Self> {
        static MATCHERS: Lazy<DashMap<LanguageType, Arc<SharedMatchers>>> = Lazy::new(DashMap::new);

        MATCHERS
            .entry(language)
            .or_insert_with(|| Arc::new(Self::init(language)))
            .value()
            .clone()
    }

    pub fn init(language: LanguageType) -> Self {
        fn init_corasick(pattern: &[&'static str]) -> AhoCorasick {
            AhoCorasick::builder()
                .match_kind(aho_corasick::MatchKind::LeftmostLongest)
                .start_kind(aho_corasick::StartKind::Unanchored)
                .prefilter(true)
                .kind(Some(aho_corasick::AhoCorasickKind::DFA))
                .build(pattern)
                .unwrap()
        }

        Self {
            language,
            allows_nested: language.allows_nested(),
            doc_quotes: language.doc_quotes(),
            is_fortran: language.is_fortran(),
            is_literate: language.is_literate(),
            important_syntax: init_corasick(language.important_syntax()),
            any_comments: language.any_comments(),
            line_comments: language.line_comments(),
            multi_line_comments: language.multi_line_comments(),
            any_multi_line_comments: language.any_multi_line_comments(),
            nested_comments: language.nested_comments(),
            string_literals: language.quotes(),
            verbatim_string_literals: language.verbatim_quotes(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum AnalysisReport {
    /// No child languages were found, contains a boolean representing whether
    /// the line ended with comments or not.
    Normal(bool),
    ChildLanguage(FileContext),
}

impl SyntaxCounter {
    pub(crate) fn new(language: LanguageType) -> Self {
        Self {
            shared: SharedMatchers::new(language),
            quote_is_doc_quote: false,
            quote_is_verbatim: false,
            stack: Vec::with_capacity(1),
            lf_embedded_language: None,
            quote: None,
        }
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

    pub(crate) fn get_lf_target_language(&self) -> LanguageType {
        // in case the target declaration was not found, default it to that language
        const DEFAULT_LANG: LanguageType = LinguaFranca;
        self.lf_embedded_language.unwrap_or(DEFAULT_LANG)
    }

    #[inline]
    pub(crate) fn parse_line_comment(&self, window: &[u8]) -> bool {
        if self.quote.is_some() || !self.stack.is_empty() {
            false
        } else if let Some(comment) = self
            .shared
            .line_comments
            .iter()
            .find(|c| window.starts_with(c.as_bytes()))
        {
            trace!("Start {:?}", comment);
            true
        } else {
            false
        }
    }

    /// Try to see if we can determine what a line is from examining the whole
    /// line at once. Returns `true` if successful.
    pub(crate) fn try_perform_single_line_analysis(
        &self,
        line: &[u8],
        stats: &mut crate::stats::CodeStats,
    ) -> bool {
        if !self.is_plain_mode() {
            false
        } else if line.trim().is_empty() {
            stats.blanks += 1;
            trace!("Blank No.{}", stats.blanks);
            true
        } else if self.shared.important_syntax.is_match(line) {
            false
        } else {
            trace!("^ Skippable");

            if self.shared.is_literate
                || self
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

            true
        }
    }

    pub(crate) fn perform_multi_line_analysis(
        &mut self,
        lines: &[u8],
        start: usize,
        end: usize,
        config: &Config,
    ) -> AnalysisReport {
        let mut ended_with_comments = false;
        let mut skip = 0;
        macro_rules! skip {
            ($skip:expr) => {{
                skip = $skip - 1;
            }};
        }

        let regex_cache = RegexCache::build(self.shared.language, lines, start, end);

        for i in start..end {
            if skip != 0 {
                skip -= 1;
                continue;
            }

            let window = &lines[i..];

            if window.trim().is_empty() {
                break;
            }

            ended_with_comments = false;
            let is_end_of_quote_or_multi_line = self
                .parse_end_of_quote(window)
                .or_else(|| self.parse_end_of_multi_line(window));

            if let Some(skip_amount) = is_end_of_quote_or_multi_line {
                ended_with_comments = true;
                skip!(skip_amount);
                continue;
            } else if self.quote.is_some() {
                continue;
            }

            if let Some(child) = self.parse_context(lines, i, end, config, &regex_cache) {
                return AnalysisReport::ChildLanguage(child);
            }

            let is_quote_or_multi_line = self
                .parse_quote(window)
                .or_else(|| self.parse_multi_line_comment(window));

            if let Some(skip_amount) = is_quote_or_multi_line {
                skip!(skip_amount);
                continue;
            }

            if self.parse_line_comment(window) {
                ended_with_comments = true;
                break;
            }
        }

        AnalysisReport::Normal(ended_with_comments)
    }

    /// Performs a set of heuristics to determine whether a line is a comment or
    /// not. The procedure is as follows.
    ///
    /// - Yes/No: Counted as Comment
    ///
    /// 1. Check if we're in string mode
    ///  1. Check if string literal is a doc string and whether tokei has
    ///     been configured to treat them as comments.
    ///     - Yes: When the line starts with the doc string or when we are
    ///            continuing from a previous line.
    ///  - No: The string is a normal string literal or tokei isn't
    ///        configured to count them as comments.
    /// 2. If we're not in string mode, check if we left it this on this line.
    ///    - Yes: When we found a doc quote and we started in comments.
    /// 3. Yes: When the whole line is a comment e.g. `/* hello */`
    /// 4. Yes: When the previous line started a multi-line comment.
    /// 5. Yes: When the line starts with a comment.
    /// 6. No: Any other input.
    pub(crate) fn line_is_comment(
        &self,
        line: &[u8],
        config: &crate::Config,
        _ended_with_comments: bool,
        started_in_comments: bool,
    ) -> bool {
        let trimmed = line.trim();
        let whole_line_is_comment = || {
            self.shared
                .line_comments
                .iter()
                .any(|c| trimmed.starts_with(c.as_bytes()))
                || self
                    .shared
                    .any_multi_line_comments
                    .iter()
                    .any(|(start, end)| {
                        trimmed.starts_with(start.as_bytes()) && trimmed.ends_with(end.as_bytes())
                    })
        };
        let starts_with_comment = || {
            let quote = match self.stack.last() {
                Some(q) => q,
                _ => return false,
            };

            self.shared
                .any_multi_line_comments
                .iter()
                .any(|(start, end)| end == quote && trimmed.starts_with(start.as_bytes()))
        };

        // `Some(true)` in order to respect the current configuration.
        #[allow(clippy::if_same_then_else)]
        if self.quote.is_some() {
            if self.quote_is_doc_quote && config.treat_doc_strings_as_comments == Some(true) {
                self.quote.map_or(false, |q| line.starts_with(q.as_bytes()))
                    || (self.quote.is_some())
            } else {
                false
            }
        } else if self
            .shared
            .doc_quotes
            .iter()
            .any(|(_, e)| line.contains_slice(e.as_bytes()))
            && started_in_comments
        {
            true
        } else if (whole_line_is_comment)() {
            true
        } else if started_in_comments {
            true
        } else {
            (starts_with_comment)()
        }
    }

    #[inline]
    pub(crate) fn parse_context(
        &mut self,
        lines: &[u8],
        start: usize,
        end: usize,
        config: &Config,
        regex_cache: &RegexCache,
    ) -> Option<FileContext> {
        use std::str::FromStr;

        // static TYPE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"type="(.*)".*>"#).unwrap());
        if self.quote.is_some() || !self.stack.is_empty() {
            return None;
        }

        match regex_cache.family()? {
            RegexFamily::Markdown(md) => {
                if !lines[start..end].contains_slice(b"```") {
                    return None;
                }

                let opening_fence = md.starts_in_range(start, end)?;
                let start_of_code = opening_fence.end();
                let closing_fence = ENDING_MARKDOWN_REGEX.find(&lines[start_of_code..]);
                if let Some(m) = &closing_fence {
                    trace!("{:?}", String::from_utf8_lossy(m.as_bytes()));
                }
                let end_of_code = closing_fence
                    .map_or_else(|| lines.len(), |fence| start_of_code + fence.start());
                let end_of_code_block =
                    closing_fence.map_or_else(|| lines.len(), |fence| start_of_code + fence.end());
                let balanced = closing_fence.is_some();
                let identifier = &opening_fence.as_bytes().trim()[3..];

                let language = identifier
                    .split(|&b| b == b',')
                    .find_map(|s| LanguageType::from_str(&String::from_utf8_lossy(s)).ok())?;
                trace!(
                    "{} BLOCK: {:?}",
                    language,
                    String::from_utf8_lossy(&lines[start_of_code..end_of_code])
                );
                let stats =
                    language.parse_from_slice(lines[start_of_code..end_of_code].trim(), config);

                Some(FileContext::new(
                    LanguageContext::Markdown { balanced, language },
                    end_of_code_block,
                    stats,
                ))
            }
            RegexFamily::Rust => {
                let rest = &lines[start..];
                let comment_syntax = if rest.trim_start().starts_with(b"///") {
                    b"///"
                } else if rest.trim_start().starts_with(b"//!") {
                    b"//!"
                } else {
                    return None;
                };

                let mut stepper = LineStep::new(b'\n', start, lines.len());
                let mut markdown = Vec::new();
                let mut end_of_block = lines.len();

                while let Some((start, end)) = stepper.next(lines) {
                    if lines[start..].trim().starts_with(comment_syntax) {
                        trace!("{}", String::from_utf8_lossy(&lines[start..end]));
                        let line = lines[start..end].trim_start();
                        let stripped_line = &line[3.min(line.len())..];
                        markdown.extend_from_slice(stripped_line);
                        end_of_block = end;
                    } else {
                        end_of_block = start;
                        break;
                    }
                }

                trace!("Markdown found: {:?}", String::from_utf8_lossy(&markdown));
                let doc_block = LanguageType::Markdown.parse_from_slice(markdown.trim(), config);

                Some(FileContext::new(
                    LanguageContext::Rust,
                    end_of_block,
                    doc_block,
                ))
            }
            RegexFamily::LinguaFranca(lf) => {
                let opening_fence = lf.starts_in_range(start, end)?;
                let start_of_code = opening_fence.end();
                let closing_fence = ENDING_LF_BLOCK_REGEX.find(&lines[start_of_code..]);
                let end_of_code = closing_fence
                    .map_or_else(|| lines.len(), |fence| start_of_code + fence.start());

                let block_contents = &lines[start_of_code..end_of_code];
                trace!("LF block: {:?}", String::from_utf8_lossy(block_contents));
                let stats = self.get_lf_target_language().parse_from_slice(
                    block_contents.trim_first_and_last_line_of_whitespace(),
                    config,
                );
                trace!("-> stats: {:?}", stats);

                Some(FileContext::new(
                    LanguageContext::LinguaFranca,
                    end_of_code,
                    stats,
                ))
            }
            RegexFamily::HtmlLike(html) => {
                if let Some(mut captures) = html.start_script_in_range(start, end) {
                    let start_of_code = captures.next().unwrap().end();
                    let closing_tag = END_SCRIPT.find(&lines[start_of_code..])?;
                    let end_of_code = start_of_code + closing_tag.start();
                    let language = captures
                        .next()
                        .and_then(|m| {
                            LanguageType::from_mime(&String::from_utf8_lossy(m.as_bytes().trim()))
                        })
                        .unwrap_or(LanguageType::JavaScript);
                    let script_contents = &lines[start_of_code..end_of_code];
                    if script_contents.trim().is_empty() {
                        return None;
                    }

                    let stats = language.parse_from_slice(
                        script_contents.trim_first_and_last_line_of_whitespace(),
                        config,
                    );
                    Some(FileContext::new(
                        LanguageContext::Html { language },
                        end_of_code,
                        stats,
                    ))
                } else if let Some(mut captures) = html.start_style_in_range(start, end) {
                    let start_of_code = captures.next().unwrap().end();
                    let closing_tag = END_STYLE.find(&lines[start_of_code..])?;
                    let end_of_code = start_of_code + closing_tag.start();
                    let language = captures
                        .next()
                        .and_then(|m| {
                            LanguageType::from_str(
                                &String::from_utf8_lossy(m.as_bytes().trim()).to_lowercase(),
                            )
                            .ok()
                        })
                        .unwrap_or(LanguageType::Css);
                    let style_contents = &lines[start_of_code..end_of_code];
                    if style_contents.trim().is_empty() {
                        return None;
                    }

                    let stats = language.parse_from_slice(
                        style_contents.trim_first_and_last_line_of_whitespace(),
                        config,
                    );
                    Some(FileContext::new(
                        LanguageContext::Html { language },
                        end_of_code,
                        stats,
                    ))
                } else if let Some(mut captures) = html.start_template_in_range(start, end) {
                    let start_of_code = captures.next().unwrap().end();
                    let closing_tag = END_TEMPLATE.find(&lines[start_of_code..])?;
                    let end_of_code = start_of_code + closing_tag.start();
                    let language = captures
                        .next()
                        .and_then(|m| {
                            LanguageType::from_str(
                                &String::from_utf8_lossy(m.as_bytes().trim()).to_lowercase(),
                            )
                            .ok()
                        })
                        .unwrap_or(LanguageType::Html);

                    let template_contents = &lines[start_of_code..end_of_code];
                    if template_contents.trim().is_empty() {
                        return None;
                    }
                    let stats = language.parse_from_slice(
                        template_contents.trim_first_and_last_line_of_whitespace(),
                        config,
                    );
                    Some(FileContext::new(
                        LanguageContext::Html { language },
                        end_of_code,
                        stats,
                    ))
                } else {
                    None
                }
            }
        }
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

    #[inline]
    pub(crate) fn parse_end_of_quote(&mut self, window: &[u8]) -> Option<usize> {
        #[allow(clippy::if_same_then_else)]
        if self._is_string_mode() && window.starts_with(self.quote?.as_bytes()) {
            let quote = self.quote.take().unwrap();
            trace!("End {:?}", quote);
            Some(quote.len())
        } else if !self.quote_is_verbatim && window.starts_with(br"\\") {
            Some(2)
        } else if !self.quote_is_verbatim
            && window.starts_with(br"\")
            && self
                .shared
                .string_literals
                .iter()
                .any(|(start, _)| window[1..].starts_with(start.as_bytes()))
        {
            // Tell the state machine to skip the next character because it
            // has been escaped if the string isn't a verbatim string.
            Some(2)
        } else {
            None
        }
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
    pub(crate) fn parse_end_of_multi_line(&mut self, window: &[u8]) -> Option<usize> {
        if self
            .stack
            .last()
            .map_or(false, |l| window.starts_with(l.as_bytes()))
        {
            let last = self.stack.pop().unwrap();

            if log_enabled!(Trace) {
                if self.stack.is_empty() {
                    trace!("End {:?}", last);
                } else {
                    trace!("End {:?}. Still in comments.", last);
                }
            }

            Some(last.len())
        } else {
            None
        }
    }
}
