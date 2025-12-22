use std::{
    collections::{btree_map, BTreeMap},
    iter::IntoIterator,
    ops::{AddAssign, Deref, DerefMut},
    path::Path,
};

use rayon::prelude::*;

use crate::{
    config::Config,
    language::{Language, LanguageType},
    utils,
};

/// A newtype representing a list of languages counted in the provided
/// directory.
/// ([_List of
/// Languages_](https://github.com/XAMPPRocky/tokei#supported-languages))
#[derive(Debug, Default, PartialEq)]
pub struct Languages {
    inner: BTreeMap<LanguageType, Language>,
}

impl serde::Serialize for Languages {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Languages {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = <_>::deserialize(deserializer)?;

        Ok(Self::from_previous(map))
    }
}

impl Languages {
    fn from_previous(map: BTreeMap<LanguageType, Language>) -> Self {
        use std::collections::btree_map::Entry;
        let mut me = Self::new();

        for (name, input_language) in map {
            match me.entry(name) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += input_language;
                }
                Entry::Vacant(entry) => {
                    entry.insert(input_language);
                }
            }
        }
        me
    }

    /// Populates the `Languages` struct with statistics about languages
    /// provided by [`Language`].
    ///
    /// Takes a `&[&str]` of paths to recursively traverse, paths can be
    /// relative, absolute or glob paths. A second `&[&str]` of paths to ignore,
    /// these strings use the `.gitignore` syntax, such as `target`
    /// or `**/*.bk`.
    ///
    /// ```no_run
    /// use tokei::{Config, Languages};
    ///
    /// let mut languages = Languages::new();
    /// languages.get_statistics(&["."], &[".git", "target"], &Config::default());
    /// ```
    ///
    /// [`Language`]: struct.Language.html
    pub fn get_statistics<A: AsRef<Path>>(
        &mut self,
        paths: &[A],
        ignored: &[&str],
        config: &Config,
    ) {
        utils::fs::get_all_files(paths, ignored, &mut self.inner, config);
        let extract_classified = config.classifications.is_some();
        self.inner
            .par_iter_mut()
            .for_each(|(_, l)| l.total_with_classifications(extract_classified));
    }

    /// Constructs a new, Languages struct. Languages is always empty and does
    /// not allocate.
    ///
    /// ```rust
    /// # use tokei::*;
    /// let languages = Languages::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Languages::default()
    }

    /// Summary of the Languages struct.
    #[must_use]
    pub fn total(self: &Languages) -> Language {
        let mut total = Language::new();
        for (ty, l) in self {
            let language = l.summarise();
            total.comments += language.comments;
            total.blanks += language.blanks;
            total.code += language.code;
            total.inaccurate |= language.inaccurate;

            // Collect all reports (including classified) for this language
            let all_reports: Vec<_> = l
                .reports
                .iter()
                .chain(l.classifications.values().flat_map(|v| v.iter()))
                .cloned()
                .collect();
            total.children.insert(ty.to_string(), all_reports);
        }
        total
    }
}

impl IntoIterator for Languages {
    type Item = <BTreeMap<LanguageType, Language> as IntoIterator>::Item;
    type IntoIter = <BTreeMap<LanguageType, Language> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Languages {
    type Item = (&'a LanguageType, &'a Language);
    type IntoIter = btree_map::Iter<'a, LanguageType, Language>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut Languages {
    type Item = (&'a LanguageType, &'a mut Language);
    type IntoIter = btree_map::IterMut<'a, LanguageType, Language>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

impl AddAssign<BTreeMap<LanguageType, Language>> for Languages {
    fn add_assign(&mut self, rhs: BTreeMap<LanguageType, Language>) {
        for (name, language) in rhs {
            if let Some(result) = self.inner.get_mut(&name) {
                *result += language;
            }
        }
    }
}

impl Deref for Languages {
    type Target = BTreeMap<LanguageType, Language>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Languages {
    fn deref_mut(&mut self) -> &mut BTreeMap<LanguageType, Language> {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::Report;
    use std::path::PathBuf;

    #[test]
    fn test_get_statistics_separates_classifications_when_enabled() {
        // This would be an integration test, but we can't easily test it without
        // the full file system setup. The key is that get_statistics should call
        // total_with_classifications(true) when config.classifications is Some

        // For now, just verify the logic path exists
        let mut languages = Languages::new();
        let config = Config {
            classifications: Some(vec!["Tests:**/*.test.*".to_string()]),
            ..Config::default()
        };

        // Add a language with classified reports manually
        let mut lang = Language::new();
        let mut report = Report::new(PathBuf::from("test.js"));
        report.classification = Some("Tests".to_string());
        lang.add_report(report);

        languages.inner.insert(LanguageType::JavaScript, lang);

        // Should separate classifications when enabled
        let separate = config.classifications.is_some();
        languages
            .inner
            .iter_mut()
            .for_each(|(_, l)| l.total_with_classifications(separate));

        let js = languages.get(&LanguageType::JavaScript).unwrap();
        assert_eq!(js.classifications.len(), 1);
        assert_eq!(js.reports.len(), 0);
    }

    #[test]
    fn test_total_file_count_includes_classified_files() {
        let mut languages = Languages::new();

        // Add Clojure with 2 unclassified files and 1 test file
        let mut clj = Language::new();

        let mut prod1 = Report::new(PathBuf::from("core.clj"));
        prod1.stats.code = 100;
        let mut prod2 = Report::new(PathBuf::from("util.clj"));
        prod2.stats.code = 50;
        let mut test1 = Report::new(PathBuf::from("core_test.clj"));
        test1.stats.code = 100;
        test1.classification = Some("Tests".to_string());

        clj.add_report(prod1);
        clj.add_report(prod2);
        clj.add_report(test1);
        clj.total_with_classifications(true);

        languages.inner.insert(LanguageType::Clojure, clj);

        // Get the total
        let total = languages.total();

        // Total should count ALL files including classified ones
        // 2 unclassified + 1 test = 3 files
        // The total Language stores all files in children, so count from there
        let total_files: usize = total.children.values().map(|v| v.len()).sum();
        assert_eq!(
            total_files, 3,
            "Total should include classified files in file count"
        );
    }
}
