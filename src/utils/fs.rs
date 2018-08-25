// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the APACHE2.0/MIT licence that can be
// found in the LICENCE-{APACHE/MIT} file.

use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;
use std::sync::mpsc;

use ignore::WalkBuilder;
use ignore::overrides::OverrideBuilder;
use ignore::WalkState::*;

use rayon::prelude::*;

// This is just a re-export from the auto generated file.
pub use language::get_filetype_from_shebang;
use language::{Language, LanguageType};

pub fn get_all_files(paths: &[&str],
                     ignored_directories: Vec<&str>,
                     languages: &mut BTreeMap<LanguageType, Language>,
                     types: Option<Vec<LanguageType>>)
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

    let types: Option<&[LanguageType]> = types.as_ref().map(|v| &**v);

    let iter: Vec<_> = rx.into_iter()
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter_map(|entry| {
            if let Some(language) = LanguageType::from_path(entry.path()) {
                if (types.is_some() &&
                    types.map(|t| t.contains(&language)).unwrap()) ||
                    types.is_none()
                {
                    return language.parse(entry)
                        .ok()
                        .and_then(|s| Some((language, s)))
                }
            }

            None
    }).collect();

    for (language_type, stats) in iter {
        languages.entry(language_type)
            .or_insert_with(|| Language::new())
            .add_stat(stats);
    }
}

pub(crate) fn get_extension(path: &Path) -> Option<String> {
    match path.extension() {
        Some(extension_os) => {
            Some(extension_os.to_string_lossy().to_lowercase())
        },
        None => None
    }
}

pub(crate) fn get_filename(path: &Path) -> Option<String> {
    match path.file_name() {
        Some(filename_os) => {
            Some(filename_os.to_string_lossy().to_lowercase())
        },
        None => None
    }
}

#[cfg(test)]
mod test {
    extern crate tempdir;
    use super::*;
    use std::fs::create_dir;
    use language::languages::Languages;
    use language::LanguageType;
    use self::tempdir::TempDir;

    #[test]
    fn walker_directory_as_file() {
        let tmp_dir = TempDir::new("test").expect("Couldn't create temp dir");
        let path_name = tmp_dir.path().join("directory.rs");
        create_dir(&path_name).expect("Couldn't create directory.rs within temp");

        let mut l = Languages::new();
        get_all_files(&[tmp_dir.into_path().to_str().unwrap()], vec![], &mut l, None);

        assert!(l.get(&LanguageType::Rust).is_none());
    }
}
