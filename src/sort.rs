use std::{borrow::Cow, str::FromStr};

use serde::de::{self, Deserialize, Deserializer};

/// Used for sorting languages.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Sort {
    /// Sort by number blank lines.
    Blanks,
    /// Sort by number comments lines.
    Comments,
    /// Sort by number code lines.
    Code,
    /// Sort by number files lines.
    Files,
    /// Sort by number of lines.
    Lines,
}

impl FromStr for Sort {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.eq_ignore_ascii_case("blanks") {
            Sort::Blanks
        } else if s.eq_ignore_ascii_case("comments") {
            Sort::Comments
        } else if s.eq_ignore_ascii_case("code") {
            Sort::Code
        } else if s.eq_ignore_ascii_case("files") {
            Sort::Files
        } else if s.eq_ignore_ascii_case("lines") {
            Sort::Lines
        } else {
            return Err(format!("Unsupported sorting option: {}", s));
        })
    }
}

impl<'de> Deserialize<'de> for Sort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

impl<'a> From<Sort> for Cow<'a, Sort> {
    fn from(from: Sort) -> Self {
        Cow::Owned(from)
    }
}

impl<'a> From<&'a Sort> for Cow<'a, Sort> {
    fn from(from: &'a Sort) -> Self {
        Cow::Borrowed(from)
    }
}
