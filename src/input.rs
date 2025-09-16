use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, error::Error, str::FromStr};

use tokei::{Language, LanguageType, Languages};

type LanguageMap = BTreeMap<LanguageType, Language>;

#[derive(Deserialize, Serialize, Debug)]
struct Output {
    #[serde(flatten)]
    languages: LanguageMap,
    #[serde(rename = "Total")]
    totals: Language,
}

macro_rules! supported_formats {
    ($(
        ($name:ident, $feature:expr, $variant:ident [$($krate:ident),+]) =>
            $parse_kode:expr,
            $print_kode:expr,
    )+) => (
        $( // for each format
            $( // for each required krate
                #[cfg(feature = $feature)] extern crate $krate;
            )+
        )+

        /// Supported serialization formats.
        ///
        /// To enable all formats compile with the `all` feature.
        #[cfg_attr(test, derive(strum_macros::EnumIter))]
        #[derive(Debug, Clone)]
        pub enum Format {
            Json,
            $(
                #[cfg(feature = $feature)] $variant
            ),+
            // TODO: Allow adding format at runtime when used as a lib?
        }

        impl Format {
            pub fn supported() -> &'static [&'static str] {
                &[
                    "json",
                    $(
                        #[cfg(feature = $feature)] stringify!($name)
                    ),+
                ]
            }

            pub fn all() -> &'static [&'static str] {
                &[
                    $( stringify!($name) ),+
                ]
            }

            pub fn all_feature_names() -> &'static [&'static str] {
                &[
                    $( $feature ),+
                ]
            }

            pub fn not_supported() -> &'static [&'static str] {
                &[
                    $(
                        #[cfg(not(feature = $feature))] stringify!($name)
                    ),+
                ]
            }

            pub fn parse(input: &str) -> Option<LanguageMap> {
                if input.is_empty() {
                    return None
                }

                if let Ok(Output { languages, .. }) = serde_json::from_str::<Output>(input) {
                    return Some(languages);
                }

                $(
                    // attributes are not yet allowed on `if` expressions
                    #[cfg(feature = $feature)]
                    {
                        let parse = &{ $parse_kode };

                        if let Ok(Output { languages, .. }) = parse(input) {
                            return Some(languages)
                        }
                    }
                )+

                // Didn't match any of the compiled serialization formats
                None
            }

            pub fn print(&self, languages: &Languages) -> Result<String, Box<dyn Error>> {
                let output = Output {
                    languages: (*languages).to_owned(),
                    totals: languages.total()
                };

                match *self {
                    Format::Json => Ok(serde_json::to_string(&output)?),
                    $(
                        #[cfg(feature = $feature)] Format::$variant => {
                            let print= &{ $print_kode };
                            Ok(print(&output)?)
                        }
                    ),+
                }
            }
        }

        impl FromStr for Format {
            type Err = String;

            fn from_str(format: &str) -> Result<Self, Self::Err> {
                match format {
                    "json" => Ok(Format::Json),
                    $(
                        stringify!($name) => {
                            #[cfg(feature = $feature)]
                            return Ok(Format::$variant);

                            #[cfg(not(feature = $feature))]
                            return Err(format!(
"This version of tokei was compiled without \
any '{format}' serialization support, to enable serialization, \
reinstall tokei with the features flag.

    cargo install tokei --features {feature}

If you want to enable all supported serialization formats, you can use the 'all' feature.

    cargo install tokei --features all\n",
                                format = stringify!($name),
                                feature = $feature)
                            );
                        }
                    ),+
                    format => Err(format!("{:?} is not a supported serialization format", format)),
                }
            }
        }
    )
}

// The ordering of these determines the attempted order when parsing.
supported_formats!(
    (cbor, "cbor", Cbor [serde_cbor, hex]) =>
        |input| {
            hex::FromHex::from_hex(input)
                .map_err(|e: hex::FromHexError| <Box<dyn Error>>::from(e))
                .and_then(|hex: Vec<_>| Ok(serde_cbor::from_slice(&hex)?))
        },
        |languages| serde_cbor::to_vec(&languages).map(hex::encode),

    (json, "json", Json [serde_json]) =>
        serde_json::from_str,
        serde_json::to_string,

    (yaml, "yaml", Yaml [serde_yaml]) =>
        serde_yaml::from_str,
        serde_yaml::to_string,

    (csv, "csv", Csv [csv]) =>
        serialize_csv::from_str,
        serialize_csv::to_string,
);

pub fn add_input(input: &str, languages: &mut Languages) -> bool {
    use std::fs::File;
    use std::io::Read;

    let map = match File::open(input) {
        Ok(mut file) => {
            let contents = {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("Couldn't read file");
                contents
            };

            convert_input(&contents)
        }
        Err(_) => {
            if input == "stdin" {
                let mut stdin = ::std::io::stdin();
                let mut buffer = String::new();

                let _ = stdin.read_to_string(&mut buffer);
                convert_input(&buffer)
            } else {
                convert_input(input)
            }
        }
    };

    if let Some(map) = map {
        *languages += map;
        true
    } else {
        false
    }
}

fn convert_input(contents: &str) -> Option<LanguageMap> {
    self::Format::parse(contents)
}

#[cfg(feature = "csv")]
mod serialize_csv {
    //! CSV serialization
    //!
    //! Linearizes hierarchical blob structures into flat CSV format.
    //!
    //! Files contain Reports with CodeStats that have nested blobs:
    //!
    //! ```
    //! README.md (Markdown)
    //!   └─ Rust (code block)
    //!       └─ Markdown (comment nested within Rust)
    //! ```
    //!
    //! are flattened using `nested` column:
    //!
    //! | File      | Language | Nested          | Lines | Code | Comments | Blanks |
    //! |-----------|----------|-----------------|-------|------|----------|--------|
    //! | README.md | Markdown | ""              | 100   | 80   | 15       | 5      |
    //! | README.md | Markdown | "Rust"          | 50    | 45   | 3        | 2      |
    //! | README.md | Markdown | "Rust,Markdown" | 20    | 18   | 1        | 1      |
    //!
    //! using depth-first traversal of the CodeStats blob tree.

    use std::collections::hash_map::Entry;
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use std::error::Error;
    use std::path::PathBuf;

    use super::LanguageMap;
    use super::Output;
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use tokei::CodeStats;
    use tokei::Language;
    use tokei::LanguageType;
    use tokei::Report;

    /// CSV record for language statistics.
    ///
    /// Represents either:
    /// - Primary file stats (nested = empty)  
    /// - Nested blob stats (nested = path to blob)
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Record {
        file: PathBuf,
        /// File's primary language (constant for all blobs)
        language: LanguageType,
        /// Comma-separated nested path (e.g., "Rust,Markdown")
        #[serde(
            serialize_with = "Record::serialize_nested_langs",
            deserialize_with = "Record::deserialize_nested_langs"
        )]
        nested: Vec<LanguageType>,
        lines: usize,
        code: usize,
        comments: usize,
        blanks: usize,
        /// Accuracy flag
        inaccurate: bool,
    }

    impl Record {
        fn new(
            file: PathBuf,
            language: LanguageType,
            nested: Vec<LanguageType>,
            inaccurate: bool,
            stats: &CodeStats,
        ) -> Self {
            Self {
                file,
                language,
                nested,
                lines: stats.lines(),
                code: stats.code,
                comments: stats.comments,
                blanks: stats.blanks,
                inaccurate,
            }
        }

        fn serialize_nested_langs<S>(
            nested: &[LanguageType],
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            if nested.is_empty() {
                return serializer.serialize_str("");
            }
            let s = nested
                .iter()
                .map(LanguageType::to_string)
                .collect::<Vec<String>>()
                .join(",");
            serializer.serialize_str(&s)
        }

        fn deserialize_nested_langs<'de, D>(deserializer: D) -> Result<Vec<LanguageType>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let s = s.trim();

            if s.is_empty() {
                return Ok(Vec::new());
            }

            s.split(',')
                .map(|x| {
                    LanguageType::from_name(x.trim())
                        .ok_or_else(|| serde::de::Error::custom(format!("Unknown language {x}")))
                })
                .collect()
        }

        fn to_code_stats(&self) -> CodeStats {
            let mut cs = CodeStats::new();

            cs.blanks = self.blanks;
            cs.code = self.code;
            cs.comments = self.comments;

            cs
        }

        fn to_report(&self) -> Report {
            let mut report = Report::new(self.file.clone());
            report.stats += self.to_code_stats();
            report
        }
    }

    /// Recursively serializes blob tree to CSV records.
    ///
    /// Depth-first traversal maintaining current path in `nested` vector.
    fn serialize_blobs(
        csv: &mut csv::Writer<Vec<u8>>,
        file: &PathBuf,
        primary: LanguageType,
        nested: &mut Vec<LanguageType>,
        blobs: &BTreeMap<LanguageType, CodeStats>,
    ) -> Result<(), Box<dyn Error>> {
        for (lang_type, stats) in blobs {
            nested.push(*lang_type);

            csv.serialize(Record::new(
                file.clone(),
                primary,
                nested.clone(),
                false,
                stats,
            ))?;

            serialize_blobs(csv, file, primary, nested, &stats.blobs)?;
            nested.pop();
        }

        Ok(())
    }

    pub(super) fn to_string(output: &Output) -> Result<String, Box<dyn Error>> {
        let mut csv = csv::Writer::from_writer(vec![]);

        for (lang_type, lang) in &output.languages {
            for report in &lang.reports {
                csv.serialize(Record::new(
                    report.name.clone(),
                    *lang_type,
                    Vec::new(),
                    lang.inaccurate,
                    &report.stats,
                ))?;
                let mut nested = Vec::new();
                serialize_blobs(
                    &mut csv,
                    &report.name,
                    *lang_type,
                    &mut nested,
                    &report.stats.blobs,
                )?;
            }
        }

        Ok(String::from_utf8(csv.into_inner()?)?)
    }

    /// Parses CSV string into Output structure
    ///
    /// Reconstructs hierarchical blob structure from linearized CSV.
    ///
    /// Steps:
    /// 1. Parse CSV records, group by file
    /// 2. Use `nested` field as navigation path to rebuild blob tree
    /// 3. Sort by original order, aggregate statistics
    ///
    /// Example (simplified):
    ///
    /// | File      | Language | Nested          | Lines |
    /// |-----------|----------|-----------------|-------|
    /// | README.md | Markdown | ""              | 100   |
    /// | README.md | Markdown | "Rust"          | 50    |
    /// | README.md | Markdown | "Rust,Markdown" | 20    |
    ///
    /// becomes:
    ///
    /// ```
    /// README.md (Markdown) {
    ///   stats: 100 lines
    ///   blobs: {
    ///     Rust: {
    ///       stats: 50 lines  
    ///       blobs: { Markdown: { 20 lines } }
    ///     }
    ///   }
    /// }
    /// ```
    pub(super) fn from_str(s: &str) -> Result<Output, Box<dyn Error>> {
        let mut csv = csv::Reader::from_reader(s.as_bytes());
        let mut files: HashMap<PathBuf, (usize, LanguageType, Report)> = HashMap::new();

        // Parse CSV records and group by file
        for (idx, record) in csv.deserialize::<Record>().enumerate() {
            let record = record?;
            match files.entry(record.file.clone()) {
                Entry::Occupied(mut entry) => {
                    let (_, _, report) = entry.get_mut();

                    // Navigate blob tree path, create missing nodes
                    let stats = record.nested.iter().fold(&mut report.stats, |stats, lang| {
                        stats.blobs.entry(*lang).or_default()
                    });
                    *stats += record.to_code_stats();
                }
                Entry::Vacant(entry) => {
                    entry.insert((idx, record.language, record.to_report()));
                }
            }
        }

        let mut languages = LanguageMap::new();
        let mut totals = Language::new();

        // Sort by original order and aggregate
        let mut sorted_files: Vec<_> = files.into_values().collect();
        sorted_files.sort_unstable_by_key(|(idx, _, _)| *idx);

        for (_, lang_type, report) in sorted_files {
            totals.add_report(report.clone());
            languages.entry(lang_type).or_default().add_report(report);
        }

        languages.values_mut().for_each(Language::total);
        totals.total();

        Ok(Output { languages, totals })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use strum::IntoEnumIterator;
    use tokei::Config;

    use std::path::Path;

    #[test]
    fn formatting_print_matches_parse() {
        // Get language results from sample dir
        let data_dir = Path::new("tests").join("data");
        let mut langs = Languages::new();
        langs.get_statistics(&[data_dir], &[], &Config::default());

        // Check that the value matches after serializing and deserializing
        for variant in Format::iter() {
            let serialized = variant
                .print(&langs)
                .unwrap_or_else(|_| panic!("Failed serializing variant: {:?}", variant));
            let deserialized = Format::parse(&serialized)
                .unwrap_or_else(|| panic!("Failed deserializing variant: {:?}", variant));
            assert_eq!(*langs, deserialized);
        }
    }
}
