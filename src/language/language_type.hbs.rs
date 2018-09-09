// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the APACHE2.0/MIT licence that can be
// found in the LICENCE-{APACHE/MIT} file.

use std::borrow::Cow;
use std::fmt;
use std::path::Path;
use std::fs::File;
use std::io::{self, Read, BufRead, BufReader};
use std::str::FromStr;

use encoding_rs_io::DecodeReaderBytes;
use log::Level::Trace;
use ignore::DirEntry;

use utils::fs as fsutils;
use self::LanguageType::*;
use stats::Stats;

struct Comments {
    allows_nested: bool,
    line_comments: &'static [&'static str],
    multi_line_comments: &'static [(&'static str, &'static str)],
    nested_comments: &'static [(&'static str, &'static str)],
    quotes: &'static [(&'static str, &'static str)],
}

#[cfg_attr(feature = "io", derive(Deserialize, Serialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LanguageType {
    {{~#each languages}}
        {{~@key}},
    {{/each}}
}

impl LanguageType {

    pub(crate) fn blank_allows_nested() -> bool {
        false
    }

    pub(crate) fn blank_line_comments() -> &'static [&'static str] {
        &[]
    }

    pub(crate) fn blank_multi_line_comments()
        -> &'static [(&'static str, &'static str)]
    {
        &[]
    }

    pub(crate) fn blank_quotes() -> &'static [(&'static str, &'static str)] {
        &[]
    }

    pub(crate) fn c_allows_nested() -> bool {
        Self::blank_allows_nested()
    }

    pub(crate) fn c_line_comments() -> &'static [&'static str] {
        &["//"]
    }

    pub(crate) fn c_multi_line_comments()
        -> &'static [(&'static str, &'static str)]
    {
        &[("/*", "*/")]
    }

    pub(crate) fn c_quotes() -> &'static [(&'static str, &'static str)] {
        &[("\"", "\"")]
    }

    pub(crate) fn func_allows_nested() -> bool {
        Self::blank_allows_nested()
    }

    pub(crate) fn func_line_comments() -> &'static [&'static str] {
        Self::blank_line_comments()
    }

    pub(crate) fn func_multi_line_comments()
        -> &'static [(&'static str, &'static str)]
    {
        &[("(*", "*)")]
    }

    pub(crate) fn func_quotes() -> &'static [(&'static str, &'static str)] {
        Self::c_quotes()
    }

    pub(crate) fn hash_allows_nested() -> bool {
        Self::blank_allows_nested()
    }

    pub(crate) fn hash_line_comments() -> &'static [&'static str] {
        &["#"]
    }

    pub(crate) fn hash_multi_line_comments()
        -> &'static [(&'static str, &'static str)]
    {
        Self::blank_multi_line_comments()
    }

    pub(crate) fn hash_quotes() -> &'static [(&'static str, &'static str)] {
        Self::blank_quotes()
    }

    pub(crate) fn haskell_allows_nested() -> bool {
        true
    }

    pub(crate) fn haskell_line_comments() -> &'static [&'static str] {
        &["--"]
    }

    pub(crate) fn haskell_multi_line_comments()
        -> &'static [(&'static str, &'static str)]
    {
        &[("{-", "-}")]
    }

    pub(crate) fn haskell_quotes() -> &'static [(&'static str, &'static str)] {
        Self::blank_quotes()
    }

    pub(crate) fn html_allows_nested() -> bool {
        Self::blank_allows_nested()
    }

    pub(crate) fn html_line_comments() -> &'static [&'static str] {
        Self::blank_line_comments()
    }

    pub(crate) fn html_multi_line_comments()
        -> &'static [(&'static str, &'static str)]
    {
        &[("<!--", "-->")]
    }

    pub(crate) fn html_quotes() -> &'static [(&'static str, &'static str)] {
        Self::c_quotes()
    }

    pub(crate) fn pro_allows_nested() -> bool {
        Self::blank_allows_nested()
    }

    pub(crate) fn pro_line_comments() -> &'static [&'static str] {
        &["%"]
    }

    pub(crate) fn pro_multi_line_comments()
        -> &'static [(&'static str, &'static str)]
    {
        &[("/*", "*/")]
    }

    pub(crate) fn pro_quotes() -> &'static [(&'static str, &'static str)] {
        Self::c_quotes()
    }

    /// Returns the display name of a language.
    ///
    /// ```
    /// # use tokei::*;
    /// let bash = LanguageType::Bash;
    ///
    /// assert_eq!(bash.name(), "BASH");
    /// ```
    pub fn name(self) -> &'static str {
        match self {
            {{~#each languages}}
                {{@key}} =>
                {{#if this.name}}
                    "{{~name}}"
                {{else}}
                    "{{~@key}}"
                {{~/if}},
            {{~/each}}
        }
    }

    pub(crate) fn is_blank(self) -> bool {
        match self {
            {{#each languages}}
                {{#if this.blank}}
                    {{@key}} => true,
                {{/if}}
            {{/each}}
            _ => false,
        }
    }

    pub(crate) fn is_fortran(self) -> bool {
        self == LanguageType::FortranModern ||
        self == LanguageType::FortranLegacy
    }

    /// Provides every variant in a Vec
    pub fn list() -> Vec<Self> {
        return vec! [
            {{#each languages}}
                {{@key}},
            {{~/each}}
        ]
    }

    /// Returns the single line comments of a language.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::Rust;
    /// assert_eq!(lang.line_comments(), &["//"]);
    /// ```
    pub fn line_comments(self) -> &'static [&'static str] {
        match self {
            {{#each languages}}
                {{~@key}} =>
                    {{#if this.line_comment}}
                        &[
                            {{~#each this.line_comment}}
                                "{{~this}}",
                            {{~/each}}
                        ],
                    {{else}}
                        {{#if this.base}}
                            Self::{{this.base}}_line_comments(),
                        {{else}}
                            Self::blank_line_comments(),
                        {{~/if}}
                    {{~/if}}
            {{~/each}}
        }
    }

    /// Returns the single line comments of a language.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::Rust;
    /// assert_eq!(lang.multi_line_comments(), &[("/*", "*/")]);
    /// ```
    pub fn multi_line_comments(self) -> &'static [(&'static str, &'static str)]
    {
        match self {
            {{#each languages}}
                {{~@key}} =>
                    {{#if this.multi_line}}
                        &[
                            {{~#each this.multi_line}}
                                (
                                {{~#each this}}
                                    "{{~this}}",
                                {{~/each}}
                                ),
                            {{~/each}}
                        ],
                    {{else}}
                        {{#if this.base}}
                            Self::{{this.base}}_multi_line_comments(),
                        {{else}}
                            Self::blank_multi_line_comments(),
                        {{~/if}}
                    {{~/if}}
            {{~/each}}
        }
    }


    /// Returns whether the language allows nested multi line comments.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::Rust;
    /// assert!(lang.allows_nested());
    /// ```
    pub fn allows_nested(self) -> bool {
        match self {
            {{#each languages}}
                {{~@key}} =>
                    {{~#if this.base}}
                        {{~#if this.nested}}
                            true
                        {{else}}
                                Self::{{this.base}}_allows_nested()
                        {{~/if}}
                    {{else}}
                        {{~#if this.nested}}
                            true
                        {{else}}
                            false
                        {{~/if}}
                    {{~/if}},
            {{~/each}}
        }
    }

    /// Returns what nested comments the language has. (Currently only D has
    /// any of this type.)
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::D;
    /// assert_eq!(lang.nested_comments(), &[("/+", "+/")]);
    /// ```
    pub fn nested_comments(self) -> &'static [(&'static str, &'static str)]
    {
        match self {
            {{#each languages}}
                {{~@key}} => &[
                    {{~#each this.nested_comments}}
                    (
                        {{~#each this}} "{{this}}", {{~/each}}
                    ),
                    {{~/each}}
                ],
            {{~/each}}
        }
    }

    /// Returns the quotes of a language.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::Rust;
    /// assert_eq!(lang.quotes(), &[("r#\"", "\"#"), ("#\"", "\"#"), ("\"", "\"")]);
    /// ```
    pub fn quotes(self) -> &'static [(&'static str, &'static str)] {
        match self {
            {{#each languages}}
                {{~@key}} =>
                    {{#if this.quotes}}
                        &[
                            {{~#each this.quotes}}
                                (
                                {{~#each this}}
                                    "{{this}}",
                                {{~/each}}
                                ),
                            {{~/each}}
                        ],
                    {{else}}
                        {{#if this.base}}
                            Self::{{this.base}}_quotes(),
                        {{else}}
                            Self::blank_quotes(),
                        {{~/if}}
                    {{~/if}}
            {{~/each}}
        }
    }

    /// Get language from a file path. May open and read the file.
    ///
    /// ```no_run
    /// # use tokei::*;
    /// let rust = LanguageType::from_path("./main.rs");
    ///
    /// assert_eq!(rust, Some(LanguageType::Rust));
    /// ```
    pub fn from_path<P: AsRef<Path>>(entry: P) -> Option<Self> {
        let entry = entry.as_ref();

        if let Some(filename) = fsutils::get_filename(&entry) {
            match &*filename {
                {{~#each languages}}
                    {{~#if this.filenames}}
                        {{~#each this.filenames}}
                            "{{~this}}" {{~#unless @last}} | {{~/unless}}
                        {{~/each}}
                            => return Some({{~@key}}),
                    {{~/if}}
                {{~/each}}
                _ => ()
            }
        }

        let extension = fsutils::get_extension(&entry);
        let filetype = extension.as_ref()
            .map(|s| &**s)
            .or_else(|| get_filetype_from_shebang(&entry));

        if let Some(extension) = filetype {
            match extension {
                {{~#each languages}}
                    {{~#if this.extensions}}
                        {{~#each this.extensions}}
                            "{{~this}}" {{~#unless @last}} | {{~/unless}}
                        {{~/each}}
                            => Some({{~@key}}),
                    {{~/if}}
                {{~/each}}
                extension => {
                    warn!("Unknown extension: {}", extension);
                    None
                },
            }
        } else {
            None
        }
    }

    /// Parses a given `DirEntry` using the `LanguageType`. Returning `Stats`
    /// on success.
    pub fn parse(self, entry: DirEntry) -> io::Result<Stats> {
        let text = {
            let f = File::open(entry.path())?;
            let mut s = String::new();
            let mut reader = DecodeReaderBytes::new(f);

            reader.read_to_string(&mut s)?;
            s
        };

        self.parse_from_str(entry, &text)
    }

    /// Parses the text provided. Returning `Stats` on success.
    pub fn parse_from_str(self, entry: DirEntry, text: &str)
        -> io::Result<Stats>
    {

        let lines = text.lines();
        let mut stats = Stats::new(entry.path().to_owned());

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
    fn parse_basic(comments: &Comments, line: &str, stats: &mut Stats)
        -> bool
    {

        let mut iter = comments.quotes.iter()
            .chain(comments.multi_line_comments)
            .chain(comments.nested_comments);

        if !iter.any(|(s, _)| line.contains(s)) {
            trace!("Determined to be skippable");
            if comments.line_comments.iter().any(|s| line.starts_with(s)) {
                stats.comments += 1;
                trace!("Determined to be comment. So far: {} lines", stats.comments);
            } else {
                stats.code += 1;
                trace!("Determined to be code. So far: {} lines", stats.code);
            }

            trace!("{}", line);
            true
        } else {
            false
        }
    }

    #[inline]
    fn parse_lines<'a>(self,
                    lines: impl IntoIterator<Item=&'a str>,
                    mut stats: Stats)
        -> Stats
    {
        let mut stack: Vec<&'static str> = Vec::with_capacity(1);
        let mut quote: Option<&'static str> = None;
        let comments = Comments {
            allows_nested: self.allows_nested(),
            line_comments: self.line_comments(),
            multi_line_comments: self.multi_line_comments(),
            nested_comments: self.nested_comments(),
            quotes: self.quotes(),
        };

        for line in lines {

            if line.chars().all(char::is_whitespace) {
                stats.blanks += 1;
                trace!("Blank line. So far: {}", stats.blanks);
                continue;
            }

            // FORTRAN has a rule where it only counts as a comment if it's the
            // first character in the column, so removing starting whitespace
            // could cause a miscount.
            let line = if self.is_fortran() { line } else { line.trim() };
            let mut ended_with_comments = false;
            let mut had_code = stack.is_empty();
            let mut skip = 0;
            macro_rules! skip {
                ($skip:expr) => { {
                    skip = $skip - 1;
                } }
            }

            if quote.is_none() &&
               stack.is_empty() &&
               Self::parse_basic(&comments, line, &mut stats)
            {
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

                if let Some(true) = stack.last()
                    .and_then(|l| Some(window.starts_with(l.as_bytes())))
                    {
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
                    for comment in comments.line_comments {
                        if window.starts_with(comment.as_bytes()) {
                            trace!(r#"Start of "{}"."#, comment);
                            break 'window;
                        }
                    }

                    for &(start, end) in comments.quotes {
                        if window.starts_with(start.as_bytes()) {
                            quote = Some(end);
                            trace!(r#"Start of "{}"."#, start);
                            skip!(start.len());
                            continue 'window;
                        }
                    }
                }

                for &(start, end) in comments.nested_comments {
                    if window.starts_with(start.as_bytes()) {
                        stack.push(end);
                        trace!(r#"Start of "{}"."#, start);
                        skip!(start.len());
                        continue 'window;
                    }
                }

                for &(start, end) in comments.multi_line_comments {
                    if window.starts_with(start.as_bytes()) {
                        if comments.allows_nested || stack.is_empty() {
                            if log_enabled!(Trace) && comments.allows_nested {
                                trace!(r#"Start of nested "{}"."#, start);
                            } else {
                                trace!(r#"Start of "{}"."#, start);
                            }

                            stack.push(end);
                        }

                        skip!(start.len());
                        continue 'window;
                    }
                }
            }

            let starts_with_comment = comments.multi_line_comments.iter()
                .map(|&(s, _)| s)
                .chain(comments.line_comments.iter().map(|s| *s))
                .chain(comments.nested_comments.iter().map(|&(s, _)| s))
                .any(|comment| line.starts_with(comment));

            trace!("{}", line);

            if ((!stack.is_empty() || ended_with_comments) && !had_code) ||
                (starts_with_comment && quote.is_none())
                {
                    stats.comments += 1;
                    trace!("Determined to be comment. So far: {} lines", stats.comments);
                    trace!("Did it have code?: {}", had_code);
                } else {
                    stats.code += 1;
                    trace!("Determined to be code. So far: {} lines", stats.code);
                }
        }

        stats.lines = stats.blanks + stats.code + stats.comments;
        stats
    }
}

impl FromStr for LanguageType {
    type Err = &'static str;

    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match &*from {
            {{~#each languages}}
                {{~#if this.name}}
                    "{{~this.name}}"
                {{else}}
                    "{{~@key}}"
                {{~/if}}
                    => Ok({{~@key}}),
            {{~/each}}
            _ => Err("Language not found, please use `-l` to see all available\
                     languages."),
        }
    }
}

impl fmt::Display for LanguageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}


impl<'a> From<LanguageType> for Cow<'a, LanguageType> {
    fn from(from: LanguageType) -> Self {
        Cow::Owned(from)
    }
}

impl<'a> From<&'a LanguageType> for Cow<'a, LanguageType> {
    fn from(from: &'a LanguageType) -> Self {
        Cow::Borrowed(from)
    }
}


/// This is for getting the file type from the first line of a file
pub fn get_filetype_from_shebang(file: &Path) -> Option<&'static str>
{
    let file = match File::open(file) {
        Ok(file) => file,
        _ => return None,
    };

    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let _ = buf.read_line(&mut line);

    let mut words = line.split_whitespace();
    match words.next() {
        Some("#!/bin/sh") => Some("sh"),
        Some("#!/bin/csh") => Some("csh"),
        Some("#!/usr/bin/perl") => Some("pl"),
        Some("#!/usr/bin/env") => {
            if let Some(word) = words.next() {
                match word {
                    {{~#each languages}}
                        {{~#if this.env}}
                            {{~#each this.env}}
                                "{{~this}}"
                                {{~#unless @last}}
                                    |
                                {{~/unless}}
                            {{~/each}}
                                => Some("{{this.extensions.[0]}}"),
                        {{~/if}}
                    {{~/each}}
                    env => {
                        warn!("Unknown environment: {:?}", env);
                        None
                    }
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust() {
        assert_eq!(LanguageType::Rust.allows_nested(), true);
    }
}
