use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;
use std::sync::mpsc;

use ignore::WalkBuilder;
use ignore::overrides::OverrideBuilder;
use ignore::WalkState::*;

use rayon::prelude::*;

// This is just a re-export from the auto generated file.
pub use crate::language::get_filetype_from_shebang;
use crate::language::{Language, LanguageType};
use crate::config::Config;

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
mod test {
    use std::fs::create_dir;

    use tempdir::TempDir;

    use crate::{config::Config, language::{LanguageType, languages::Languages}};
    use super::*;

    #[test]
    fn walker_directory_as_file() {
        let tmp_dir = TempDir::new("test").expect("Couldn't create temp dir");
        let path_name = tmp_dir.path().join("directory.rs");
        create_dir(&path_name).expect("Couldn't create directory.rs within temp");

        let mut languages = Languages::new();
        get_all_files(&[tmp_dir.into_path().to_str().unwrap()],
                      &[],
                      &mut languages,
                      &Config::default());

        assert!(languages.get(&LanguageType::Rust).is_none());
    }
}
