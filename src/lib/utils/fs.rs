// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the APACHE2.0/MIT licence that can be
// found in the LICENCE-{APACHE/MIT} file.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;

use glob::glob;
use walkdir::{WalkDir, WalkDirIterator};

use language::{Language, LanguageType};
use language::LanguageType::*;
pub use language::get_filetype_from_shebang;

macro_rules! get_language {
    ($languages:expr, $entry:expr) => {
        if let Some(language_type) = LanguageType::from_extension($entry) {
            opt_error!($languages.get_mut(&language_type), "Unknown Language? Shouldn't happen.")
        } else {
            continue;
        }
    }
}

pub fn get_all_files<'a>(paths: Cow<'a, [&'a str]>,
                         ignored_directories: Cow<'a, [&'a str]>,
                         languages: &mut BTreeMap<LanguageType, Language>) {
    for path in &*paths {
        // A small metadata check to check if the file actually exists,
        // this is used over calling  File::open because we're going to be
        // passing the path to either glob() or WalkDir::new()
        if let Err(_) = Path::new(path).metadata() {
            if let Ok(paths) = glob(path) {
                'path: for path in paths {
                    let path = rs_error!(path);
                    let path_str = opt_error!(path.to_str(),
                                             "DURING FILE LOOKUP: Couldn't convert path to string.");

                    for ig in &*ignored_directories {
                        if path_str.contains(ig) {
                            continue 'path;
                        }
                    }
                    let mut language = if path_str.contains("Makefile") {
                        languages.get_mut(&Makefile).unwrap()
                    } else {
                        get_language!(languages, &path)
                    };

                    language.files.push(path.to_owned());
                }
            }
        } else {
            let walker = WalkDir::new(path).into_iter().filter_entry(|entry| {
                for ig in &*ignored_directories {
                    if entry.path().to_str().unwrap().contains(&*ig) {
                        return false;
                    }
                }
                true
            });

            for entry in walker {
                let entry = rs_error!(entry);

                let mut language = if opt_error!(entry.path().to_str(),
                                                "Walkdir: Couldn't convert path to string")
                    .contains("Makefile") {
                    languages.get_mut(&Makefile).unwrap()
                } else {
                    get_language!(languages, entry.path())
                };

                language.files.push(entry.path().to_owned());
            }
        }
    }
}

pub fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
    let path = path.as_ref();
    match path.extension() {
        Some(extension_os) => {
            match extension_os.to_str() {
                Some(extension) => Some(extension.to_lowercase()),
                None => None,
            }
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
