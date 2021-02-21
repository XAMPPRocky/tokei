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
    language::syntax::{FileContext, LanguageContext, SyntaxCounter},
    stats::{CodeStats, Report},
    utils::{ext::SliceExt, fs as fsutils},
};

use encoding_rs_io::DecodeReaderBytesBuilder;
use grep_searcher::{LineIter, LineStep};
use rayon::prelude::*;

use self::LanguageType::*;

include!(concat!(env!("OUT_DIR"), "/language_type.rs"));

impl LanguageType {
    /// Parses a given `Path` using the `LanguageType`. Returning `Report`
    /// on success and giving back ownership of PathBuf on error.
    pub fn parse(self, path: PathBuf, config: &Config) -> Result<Report, (io::Error, PathBuf)> {
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

        let mut stats = Report::new(path);

        stats += self.parse_from_slice(&text, config);

        Ok(stats)
    }

    /// Parses the text provided as the given `LanguageType`.
    pub fn parse_from_str<A: AsRef<str>>(self, text: A, config: &Config) -> CodeStats {
        self.parse_from_slice(text.as_ref().as_bytes(), config)
    }

    /// Parses the bytes provided as the given `LanguageType`.
    pub fn parse_from_slice<A: AsRef<[u8]>>(self, text: A, config: &Config) -> CodeStats {
        let text = text.as_ref();

        if self == LanguageType::Jupyter {
            return self
                .parse_jupyter(text.as_ref(), config)
                .unwrap_or_else(CodeStats::new);
        }

        let syntax = SyntaxCounter::new(self);

        if let Some(end) = syntax
            .shared
            .important_syntax
            .earliest_find(text)
            .and_then(|m| {
                // Get the position of the last line before the important
                // syntax.
                text[..=m.start()]
                    .iter()
                    .rev()
                    .position(|&c| c == b'\n')
                    .filter(|&p| p != 0)
                    .map(|p| m.start() - p)
            })
        {
            let (skippable_text, rest) = text.split_at(end + 1);
            let is_fortran = syntax.shared.is_fortran;
            let is_literate = syntax.shared.is_literate;
            let comments = syntax.shared.line_comments;
            trace!(
                "Using Simple Parse on {:?}",
                String::from_utf8_lossy(skippable_text)
            );
            let parse_lines = move || self.parse_lines(config, rest, CodeStats::new(), syntax);
            let simple_parse = move || {
                LineIter::new(b'\n', skippable_text)
                    .par_bridge()
                    .map(|line| {
                        // FORTRAN has a rule where it only counts as a comment if it's the
                        // first character in the column, so removing starting whitespace
                        // could cause a miscount.
                        let line = if is_fortran { line } else { line.trim() };
                        if line.trim().is_empty() {
                            (1, 0, 0)
                        } else if is_literate
                            || comments.iter().any(|c| line.starts_with(c.as_bytes()))
                        {
                            (0, 0, 1)
                        } else {
                            (0, 1, 0)
                        }
                    })
                    .reduce(|| (0, 0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2))
            };

            let (mut stats, (blanks, code, comments)) = rayon::join(parse_lines, simple_parse);

            stats.blanks += blanks;
            stats.code += code;
            stats.comments += comments;
            stats
        } else {
            self.parse_lines(config, text, CodeStats::new(), syntax)
        }
    }

    #[inline]
    fn parse_lines(
        self,
        config: &Config,
        lines: &[u8],
        mut stats: CodeStats,
        mut syntax: SyntaxCounter,
    ) -> CodeStats {
        let mut stepper = LineStep::new(b'\n', 0, lines.len());

        while let Some((start, end)) = stepper.next(lines) {
            let line = &lines[start..end];
            // FORTRAN has a rule where it only counts as a comment if it's the
            // first character in the column, so removing starting whitespace
            // could cause a miscount.
            let line = if syntax.shared.is_fortran {
                line
            } else {
                line.trim()
            };
            trace!("{}", String::from_utf8_lossy(line));

            if syntax.can_perform_single_line_analysis(line, &mut stats) {
                continue;
            }

            let started_in_comments = !syntax.stack.is_empty()
                || (config.treat_doc_strings_as_comments == Some(true)
                    && syntax.quote.is_some()
                    && syntax.quote_is_doc_quote);
            let ended_with_comments =
                match syntax.perform_multi_line_analysis(lines, start, end, config) {
                    crate::language::syntax::AnalysisReport::Normal(end) => end,
                    crate::language::syntax::AnalysisReport::ChildLanguage(FileContext {
                        language,
                        end,
                        stats: blob,
                    }) => {
                        match language {
                            LanguageContext::Markdown { balanced, language } => {
                                // Add the lines for the code fences.
                                stats.comments += if balanced { 2 } else { 1 };
                                // Add the code inside the fence to the stats.
                                *stats.blobs.entry(language).or_default() += blob;
                            }
                            LanguageContext::Rust => {
                                // Add all the markdown blobs.
                                *stats.blobs.entry(LanguageType::Markdown).or_default() += blob;
                            }
                            LanguageContext::Html { language } => {
                                stats.code += 1;
                                // Add all the markdown blobs.
                                *stats.blobs.entry(language).or_default() += blob;
                            }
                        }

                        // Advance to after the language code and the delimiter..
                        stepper = LineStep::new(b'\n', end, lines.len());
                        continue;
                    }
                };
            trace!("{}", String::from_utf8_lossy(line));

            if syntax.shared.is_literate
                || syntax.line_is_comment(line, config, ended_with_comments, started_in_comments)
            {
                stats.comments += 1;
                trace!("Comment No.{}", stats.comments);
                trace!("Was the Comment stack empty?: {}", !started_in_comments);
            } else {
                stats.code += 1;
                trace!("Code No.{}", stats.code);
            }
        }

        stats
    }

    fn parse_jupyter(&self, json: &[u8], config: &Config) -> Option<CodeStats> {
        #[derive(Deserialize)]
        struct Jupyter {
            cells: Vec<JupyterCell>,
            metadata: JupyterMetadata,
        }

        #[derive(Clone, Copy, Deserialize, PartialEq, Eq)]
        #[serde(rename_all = "lowercase")]
        enum CellType {
            Markdown,
            Code,
        }

        #[derive(Deserialize)]
        struct JupyterCell {
            cell_type: CellType,
            source: Vec<String>,
        }

        #[derive(Deserialize)]
        struct JupyterMetadata {
            kernelspec: serde_json::Value,
            language_info: serde_json::Value,
        }

        let jupyter: Jupyter = serde_json::from_slice(json).ok()?;

        let mut jupyter_stats = CodeStats::new();

        let language = jupyter
            .metadata
            .kernelspec
            .get("language")
            .and_then(serde_json::Value::as_str)
            .and_then(|v| LanguageType::from_str(v).ok())
            .or_else(|| {
                jupyter
                    .metadata
                    .language_info
                    .get("file_extension")
                    .and_then(serde_json::Value::as_str)
                    .and_then(LanguageType::from_file_extension)
            })
            .unwrap_or(LanguageType::Python);

        let iter = jupyter
            .cells
            .par_iter()
            .map(|cell| match cell.cell_type {
                CellType::Markdown => (
                    LanguageType::Markdown,
                    LanguageType::Markdown.parse_from_str(cell.source.join(""), config),
                ),
                CellType::Code => (
                    language,
                    language.parse_from_str(cell.source.join(""), config),
                ),
            })
            .collect::<Vec<_>>();

        for (language, stats) in iter {
            *jupyter_stats.blobs.entry(language).or_default() += stats;
        }

        Some(jupyter_stats)
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
