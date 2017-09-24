// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the APACHE2.0/MIT licence that can be
// found in the LICENCE-{APACHE/MIT} file.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::mpsc;
use std::error::Error;

use ignore::WalkParallel;
use ignore::WalkState::*;

use language::{Language, Languages, LanguageType};
// This is just a re-export from the auto generated file.
pub use language::get_filetype_from_shebang;

pub fn get_all_files(walker: WalkParallel,
                     languages: &mut BTreeMap<LanguageType, Language>)
{
    let (tx, rx) = mpsc::channel();

    walker.run(move|| {
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

            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Some(language) = LanguageType::from_path(entry) {
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
        },
        None => None
    }
}

pub fn get_filename<P: AsRef<Path>>(path: P) -> Option<String> {
    let path = path.as_ref();
    match path.file_name() {
        Some(filename_os) => {
            Some(filename_os.to_string_lossy().to_lowercase())
        },
        None => None
    }
}
