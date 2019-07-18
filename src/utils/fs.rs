use std::{
    collections::BTreeMap,
    error::Error,
    path::Path,
    sync::mpsc,
};

use ignore::{
    WalkBuilder,
    WalkState::Continue,
    overrides::OverrideBuilder,
};

use rayon::prelude::*;

// This is just a re-export from the auto generated file.
pub use crate::language::get_filetype_from_shebang;

use crate::{
    config::Config,
    language::{Language, LanguageType},
};

const IGNORE_FILE: &str = ".tokeignore";

pub fn get_all_files<A: AsRef<Path>>(paths: &[A],
                     ignored_directories: &[&str],
                     languages: &mut BTreeMap<LanguageType, Language>,
                     config: &Config)
{
    let (tx, rx) = mpsc::channel();

    let mut paths = paths.iter();
    let mut walker = WalkBuilder::new(paths.next().unwrap());

    for path in paths {
        walker.add(path);
    }

    if !ignored_directories.is_empty() {
        let mut overrides = OverrideBuilder::new(".");

        for ignored in ignored_directories {
            rs_error!(overrides.add(&format!("!{}", ignored)));
        }

        walker.overrides(overrides.build().expect("Excludes provided were invalid"));
    }

    // Flip the booleans as ignore's semantics are the opposite of our options.
    let no_ignore_vcs = config.no_ignore_vcs.map(|b| !b).unwrap_or(true);

    walker.add_custom_ignore_filename(IGNORE_FILE)
          .git_exclude(no_ignore_vcs)
          .git_global(no_ignore_vcs)
          .git_ignore(no_ignore_vcs)
          .hidden(config.hidden.map(|b| !b).unwrap_or(true))
          .ignore(config.no_ignore.map(|b| !b).unwrap_or(true))
          .parents(config.no_ignore_parent.map(|b| !b).unwrap_or(true));

    walker.build_parallel().run(move|| {
        let tx = tx.clone();
        Box::new(move |entry| {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    use ignore::Error;
                    if let Error::WithDepth { err: ref error, .. } = error {
                        if let Error::WithPath { ref path, err: ref error } = **error {
                            error!("{} reading {}", error.description(), path.display());
                            return Continue;
                        }
                    }
                    error!("{}", error.description());
                    return Continue;
                }
            };

            if let Some(file_type) = entry.file_type() {
                if file_type.is_file() {
                    tx.send(entry).unwrap();
                }
            }

            Continue
        })
    });

    let types: Option<&[LanguageType]> = config.types.as_ref().map(|v| &**v);

    let iter = rx.into_iter()
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter_map(|e| LanguageType::from_path(e.path(), &config).map(|l| (e, l)))
        .filter(|(_, l)| types.map(|t| t.contains(l)).unwrap_or(true))
        .map(|(entry, language)| {
            language.parse(entry.into_path(), &config)
                .map(|stats| (language, Some(stats)))
                .unwrap_or_else(|(e, path)| {
                    error!("{} reading {}", e.description(), path.display());
                    (language, None)
                })
        })
        .collect::<Vec<_>>();

    for (language_type, stats) in iter {
        let entry = languages.entry(language_type).or_insert_with(Language::new);

        if let Some(stats) = stats {
            entry.add_stat(stats);
        } else {
            entry.mark_inaccurate();
        }
    }
}

pub(crate) fn get_extension(path: &Path) -> Option<String> {
    path.extension().map(|e| e.to_string_lossy().to_lowercase())
}

pub(crate) fn get_filename(path: &Path) -> Option<String> {
    path.file_name().map(|e| e.to_string_lossy().to_lowercase())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use crate::{config::Config, language::{LanguageType, languages::Languages}};
    use super::IGNORE_FILE;

    const FILE_CONTENTS: &[u8] = &*b"fn main() {}";
    const FILE_NAME: &str = "main.rs";
    const IGNORE_PATTERN: &str = "*.rs";
    const LANGUAGE: &LanguageType = &LanguageType::Rust;

    #[test]
    fn ignore_directory_with_extension() {
        let mut languages = Languages::new();
        let tmp_dir = TempDir::new().expect("Couldn't create temp dir");
        let path_name = tmp_dir.path().join("directory.rs");

        fs::create_dir(&path_name)
            .expect("Couldn't create directory.rs within temp");

        super::get_all_files(&[tmp_dir.into_path().to_str().unwrap()],
                             &[],
                             &mut languages,
                             &Config::default());

        assert!(languages.get(LANGUAGE).is_none());
    }

    #[test]
    fn hidden() {
        let dir = TempDir::new().expect("Couldn't creat temp dir.");
        let mut config = Config::default();
        let mut languages = Languages::new();

        fs::write(dir.path().join(".hidden.rs"), FILE_CONTENTS).unwrap();

        super::get_all_files(&[dir.path().to_str().unwrap()],
                             &[],
                             &mut languages,
                             &config);

        assert!(languages.get(LANGUAGE).is_none());

        config.hidden = Some(true);

        super::get_all_files(&[dir.path().to_str().unwrap()],
                             &[],
                             &mut languages,
                             &config);

        assert!(languages.get(LANGUAGE).is_some());
    }

    #[test]
    fn no_ignore() {
        let dir = TempDir::new().expect("Couldn't creat temp dir.");
        let mut config = Config::default();
        let mut languages = Languages::new();

        //git2::Repository::init(dir.path()).expect("Couldn't create git repo.");

        fs::write(dir.path().join(".ignore"), IGNORE_PATTERN).unwrap();
        fs::write(dir.path().join(FILE_NAME), FILE_CONTENTS).unwrap();

        super::get_all_files(&[dir.path().to_str().unwrap()],
                             &[],
                             &mut languages,
                             &config);

        assert!(languages.get(LANGUAGE).is_none());

        config.no_ignore = Some(true);

        super::get_all_files(&[dir.path().to_str().unwrap()],
                             &[],
                             &mut languages,
                             &config);

        assert!(languages.get(LANGUAGE).is_some());
    }

    #[test]
    fn no_ignore_parent() {
        let parent_dir = TempDir::new().expect("Couldn't create temp dir");
        let child_dir = parent_dir.path().join("child/");
        let mut config = Config::default();
        let mut languages = Languages::new();

        fs::create_dir_all(&child_dir).unwrap_or_else(|_| panic!("Couldn't create {:?}", child_dir));
        fs::write(parent_dir.path().join(".ignore"), IGNORE_PATTERN)
            .expect("Couldn't create .gitinore.");
        fs::write(child_dir.join(FILE_NAME), FILE_CONTENTS)
            .expect("Couldn't create child.rs");

        super::get_all_files(&[child_dir.as_path().to_str().unwrap()],
                             &[],
                             &mut languages,
                             &config);

        assert_eq!(None, languages.get(&LanguageType::Rust));

        config.no_ignore_parent = Some(true);

        super::get_all_files(&[child_dir.as_path().to_str().unwrap()],
                             &[],
                             &mut languages,
                             &config);

        assert!(languages.get(&LanguageType::Rust).is_some());
    }

    #[test]
    fn no_ignore_vcs_gitignore() {
        let dir = TempDir::new().expect("Couldn't creat temp dir.");
        let mut config = Config::default();
        let mut languages = Languages::new();

        git2::Repository::init(dir.path()).expect("Couldn't create git repo.");

        fs::write(dir.path().join(".gitignore"), IGNORE_PATTERN).unwrap();
        fs::write(dir.path().join(FILE_NAME), FILE_CONTENTS).unwrap();

        super::get_all_files(&[dir.path().to_str().unwrap()],
                             &[],
                             &mut languages,
                             &config);

        assert!(languages.get(LANGUAGE).is_none());

        config.no_ignore_vcs = Some(true);

        super::get_all_files(&[dir.path().to_str().unwrap()],
                             &[],
                             &mut languages,
                             &config);

        assert!(languages.get(LANGUAGE).is_some());
    }

    #[test]
    fn no_ignore_vcs_gitexclude() {
        let dir = TempDir::new().expect("Couldn't creat temp dir.");
        let mut config = Config::default();
        let mut languages = Languages::new();

        git2::Repository::init(dir.path()).expect("Couldn't create git repo.");

        fs::write(dir.path().join(".git/info/exclude"), IGNORE_PATTERN).unwrap();
        fs::write(dir.path().join(FILE_NAME), FILE_CONTENTS).unwrap();

        super::get_all_files(&[dir.path().to_str().unwrap()],
                             &[],
                             &mut languages,
                             &config);

        assert!(languages.get(LANGUAGE).is_none());

        config.no_ignore_vcs = Some(true);

        super::get_all_files(&[dir.path().to_str().unwrap()],
                             &[],
                             &mut languages,
                             &config);

        assert!(languages.get(LANGUAGE).is_some());
    }

    #[test]
    fn custom_ignore() {
        let dir = TempDir::new().expect("Couldn't creat temp dir.");
        let config = Config::default();
        let mut languages = Languages::new();

        git2::Repository::init(dir.path()).expect("Couldn't create git repo.");

        fs::write(dir.path().join(IGNORE_FILE), IGNORE_PATTERN).unwrap();
        fs::write(dir.path().join(FILE_NAME), FILE_CONTENTS).unwrap();

        super::get_all_files(&[dir.path().to_str().unwrap()],
                             &[],
                             &mut languages,
                             &config);

        assert!(languages.get(LANGUAGE).is_none());

        fs::remove_file(dir.path().join(IGNORE_FILE)).unwrap();

        super::get_all_files(&[dir.path().to_str().unwrap()],
                             &[],
                             &mut languages,
                             &config);

        assert!(languages.get(LANGUAGE).is_some());
    }
}
