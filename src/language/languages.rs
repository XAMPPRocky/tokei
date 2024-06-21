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
    /// relative, absolute or glob paths. a second `&[&str]` of paths to ignore,
    /// these strings use the `.gitignore` syntax, such as `target`
    /// or `**/*.bk`.
    ///
    /// ```no_run
    /// use tokei::{Config, Languages};
    ///
    /// let mut languages = Languages::new();
    /// languages.get_statistics(&["."], &[".git", "target"], &Config::default(), false);
    /// ```
    ///
    /// [`Language`]: struct.Language.html
    pub fn get_statistics<A: AsRef<Path>>(
        &mut self,
        paths: &[A],
        ignored: &[&str],
        config: &Config,
        include_dirs: bool,
    ) {
        utils::fs::get_all_files(paths, ignored, &mut self.inner, config);
        self.inner.par_iter_mut().for_each(|(_, l)| {
            l.total();

            if include_dirs {
                l.dirs();
            }
        });
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
            total.children.insert(*ty, language.reports.clone());
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
