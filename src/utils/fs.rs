// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the APACHE2.0/MIT licence that can be
// found in the LICENCE-{APACHE/MIT} file.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::mpsc;
use std::error::Error;

use ignore::WalkBuilder;
use ignore::overrides::OverrideBuilder;
use ignore::WalkState::*;

use language::{Language, Languages, LanguageType};
use language::LanguageType::*;
pub use language::get_filetype_from_shebang;

pub fn get_all_files(paths: Vec<&str>,
                     ignored_directories: Vec<&str>,
                     languages: &mut BTreeMap<LanguageType, Language>)
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

        walker.overrides(overrides.build().expect("Excludes were in invalid"));
    }

    walker.build_parallel().run(move|| {
        let tx = tx.clone();
        Box::new(move|entry| {

            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    error!("{}", error.description());
                    return Continue;
                }
            };

            let entry = entry.path();

            if entry.to_string_lossy().contains("Makefile") {
                tx.send((Makefile, entry.to_owned())).unwrap();
                return Continue;
            }

            if let Some(language) = LanguageType::from_extension(entry) {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        tx.send((language, entry.to_owned())).unwrap();
                    }
                }
            }

            Continue
        })
    });

    for (language_type, pathbuf) in rx {
        languages.entry(language_type)
                 .or_insert(Languages::generate_language(language_type))
                 .files.push(pathbuf);
    }
}

pub fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
    let path = path.as_ref();
    match path.extension() {
        Some(extension_os) => {
            Some(extension_os.to_string_lossy().to_lowercase())
        }
        None => {
            match get_filetype_from_shebang(path) {
                // Using String::from here because all file extensions from
                // get_filetype_from_shebang are guaranteed to be lowercase.
                Some(extension) => Some(String::from(extension)),
                None => None,
            }
        }
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
        get_all_files(vec![tmp_dir.into_path().to_str().unwrap()], vec![], &mut l);

        assert!(l.get(&LanguageType::Rust).is_none());
    }
}
