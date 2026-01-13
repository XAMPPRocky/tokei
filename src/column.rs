use std::{borrow::Cow, str::FromStr};

use serde::de::{self, Deserialize, Deserializer};

/// Used to reorder columns
#[derive(Clone, Copy, Debug)]
pub enum Column {
    /// Indicates the number of files of this type.
    Files,
    /// Indicates the total number of lines of text.
    Lines,
    /// Indicates the total number of lines of code (excluding whitespace and comments).
    Code,
    /// Indicates the number of lines containing comments.
    Comments,
    /// Indicates the number of blank or empty lines.
    Blanks,
}

impl FromStr for Column {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let col = match s {
            "files" => Self::Files,
            "lines" => Self::Lines,
            "code" => Self::Code,
            "comments" => Self::Comments,
            "blanks" => Self::Blanks,
            _ => return Err(format!("Unknown Column: {}", s)),
        };
        Ok(col)
    }
}

impl<'de> Deserialize<'de> for Column {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

impl<'a> From<Column> for Cow<'a, Column> {
    fn from(from: Column) -> Self {
        Cow::Owned(from)
    }
}

impl<'a> From<&'a Column> for Cow<'a, Column> {
    fn from(from: &'a Column) -> Self {
        Cow::Borrowed(from)
    }
}
