#![allow(clippy::trivial_regex)]

use crate::LanguageType;
use once_cell::sync::Lazy;
use regex::bytes::Regex;

pub static START_SCRIPT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"<script(?:.*type="(.*)")?.*?>"#).unwrap());
pub static END_SCRIPT: Lazy<Regex> = Lazy::new(|| Regex::new(r#"</script>"#).unwrap());

pub static START_STYLE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"<style(?:.*lang="(.*)")?.*?>"#).unwrap());
pub static END_STYLE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"</style>"#).unwrap());

pub static START_TEMPLATE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"<template(?:.*lang="(.*)")?.*?>"#).unwrap());
pub static END_TEMPLATE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"</template>"#).unwrap());

pub static STARTING_MARKDOWN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"```\S+\s"#).unwrap());
pub static ENDING_MARKDOWN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"```\s?"#).unwrap());

pub static STARTING_LF_BLOCK_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\{="#).unwrap());
pub static ENDING_LF_BLOCK_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"=}"#).unwrap());

/// A memory of a regex matched.
/// The values provided by `Self::start` and `Self::end` are in the same space as the
/// start value supplied to `RegexCache::build`
pub struct Capture<'a> {
    start: usize,
    text: &'a [u8],
}

impl Capture<'_> {
    #[inline(always)]
    fn start(&self) -> usize {
        self.start
    }
    #[inline(always)]
    pub fn end(&self) -> usize {
        self.start + self.text.len()
    }
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        self.text
    }
}

impl<'a> std::fmt::Debug for Capture<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Capture")
            .field("start", &self.start)
            .field("end", &self.end())
            .field("text", &String::from_utf8_lossy(self.text))
            .finish()
    }
}

pub(crate) struct RegexCache<'a> {
    inner: Option<RegexFamily<'a>>,
}

/// Embedding regexes are similar between different sets of languages.
/// `RegexFamily` records both which family the language belongs to,
/// as well as the actual matches
pub(crate) enum RegexFamily<'a> {
    HtmlLike(HtmlLike<'a>),
    LinguaFranca(SimpleCapture<'a>),
    Markdown(SimpleCapture<'a>),
    Rust,
}

pub(crate) struct HtmlLike<'a> {
    start_script: Option<Box<[Capture<'a>]>>,
    start_style: Option<Box<[Capture<'a>]>>,
    start_template: Option<Box<[Capture<'a>]>>,
}

pub(crate) struct SimpleCapture<'a> {
    starts: Option<Box<[Capture<'a>]>>,
}

impl<'a> HtmlLike<'a> {
    pub fn start_script_in_range<'this>(
        &'this self,
        start: usize,
        end: usize,
    ) -> Option<impl Iterator<Item = &'this Capture<'a>>> {
        filter_range(self.start_script.as_ref()?, start, end)
    }

    pub fn start_style_in_range<'this>(
        &'this self,
        start: usize,
        end: usize,
    ) -> Option<impl Iterator<Item = &'this Capture<'a>>> {
        filter_range(self.start_style.as_ref()?, start, end)
    }

    pub fn start_template_in_range<'this>(
        &'this self,
        start: usize,
        end: usize,
    ) -> Option<impl Iterator<Item = &'this Capture<'a>>> {
        filter_range(self.start_template.as_ref()?, start, end)
    }
}

impl<'a> SimpleCapture<'a> {
    pub fn starts_in_range<'this>(
        &'this self,
        start: usize,
        end: usize,
    ) -> Option<&'this Capture<'a>> {
        filter_range(self.starts.as_ref()?, start, end).and_then(|mut it| it.next())
    }

    fn make_capture(
        regex: &Regex,
        lines: &'a [u8],
        start: usize,
        end: usize,
    ) -> Option<SimpleCapture<'a>> {
        let capture = SimpleCapture {
            starts: save_captures(regex, lines, start, end),
        };

        if capture.starts.is_some() {
            Some(capture)
        } else {
            None
        }
    }
}

fn filter_range<'dataset, 'cap>(
    dataset: &'dataset [Capture<'cap>],
    start: usize,
    end: usize,
) -> Option<impl Iterator<Item = &'dataset Capture<'cap>>> {
    let pos = dataset
        .binary_search_by_key(&start, |cap| cap.start())
        .ok()?;

    if pos >= dataset.len() || dataset[pos].end() > end {
        None
    } else {
        Some(
            dataset[pos..]
                .iter()
                .take_while(move |cap| cap.end() <= end),
        )
    }
}

impl<'a> RegexCache<'a> {
    /// Returns the language family for which regexes were matched, if any
    pub(crate) fn family(&self) -> Option<&RegexFamily> {
        self.inner.as_ref()
    }

    /// Tries to memoize any matches of embedding regexes that occur within lines[start..end]
    /// for the given language. Any `Capture` values eventually recovered will use the same
    /// zero for their start as the given `start` argument.
    pub(crate) fn build(lang: LanguageType, lines: &'a [u8], start: usize, end: usize) -> Self {
        let inner = match lang {
            LanguageType::Markdown | LanguageType::UnrealDeveloperMarkdown => {
                SimpleCapture::make_capture(&STARTING_MARKDOWN_REGEX, lines, start, end)
                    .map(RegexFamily::Markdown)
            }
            LanguageType::Rust => Some(RegexFamily::Rust),
            LanguageType::LinguaFranca => {
                SimpleCapture::make_capture(&STARTING_LF_BLOCK_REGEX, lines, start, end)
                    .map(RegexFamily::LinguaFranca)
            }
            LanguageType::Html
            | LanguageType::RubyHtml
            | LanguageType::Svelte
            | LanguageType::Vue
            | LanguageType::GlimmerJs
            | LanguageType::GlimmerTs => {
                let html = HtmlLike {
                    start_script: save_captures(&START_SCRIPT, lines, start, end),
                    start_style: save_captures(&START_STYLE, lines, start, end),
                    start_template: save_captures(&START_TEMPLATE, lines, start, end),
                };

                if html.start_script.is_some()
                    || html.start_style.is_some()
                    || html.start_template.is_some()
                {
                    Some(RegexFamily::HtmlLike(html))
                } else {
                    None
                }
            }
            _ => None,
        };
        Self { inner }
    }
}

fn save_captures<'a>(
    regex: &Regex,
    lines: &'a [u8],
    start: usize,
    end: usize,
) -> Option<Box<[Capture<'a>]>> {
    let v: Vec<_> = regex
        .captures(&lines[start..end])?
        .iter()
        .flatten()
        .map(|cap| Capture {
            start: start + cap.start(),
            text: cap.as_bytes(),
        })
        .collect();

    if v.is_empty() {
        None
    } else {
        Some(v.into())
    }
}
