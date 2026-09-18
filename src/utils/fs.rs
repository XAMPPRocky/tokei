use std::{
    collections::{btree_map::Entry, BTreeMap},
    path::Path,
};

use ignore::{overrides::OverrideBuilder, DirEntry, WalkBuilder, WalkState};

use crate::{
    config::Config,
    language::{Language, LanguageType},
};

const IGNORE_FILE: &str = ".tokeignore";
const FILES_PER_WORKER: usize = 64;

pub fn get_all_files<A: AsRef<Path>>(
    paths: &[A],
    ignored_directories: &[&str],
    languages: &mut BTreeMap<LanguageType, Language>,
    config: &Config,
) {
    let threads = config.thread_count();
    let _ = rayon::ThreadPoolBuilder::new()
        .num_threads(threads.max(1))
        .thread_name(|i| format!("tokei-{i}"))
        .build_global();

    let stream_lock = parking_lot::Mutex::new(());
    let walker = build_walker(paths, ignored_directories, config, threads);

    if threads <= 1 {
        count_sequential(walker, languages, config, &stream_lock);
    } else {
        count_parallel(walker, languages, config, &stream_lock, threads);
    }
}

fn build_walker<A: AsRef<Path>>(
    paths: &[A],
    ignored_directories: &[&str],
    config: &Config,
    threads: usize,
) -> WalkBuilder {
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

    let ignore = config.no_ignore.map(|b| !b).unwrap_or(true);
    let ignore_dot = ignore && config.no_ignore_dot.map(|b| !b).unwrap_or(true);
    let ignore_vcs = ignore && config.no_ignore_vcs.map(|b| !b).unwrap_or(true);

    // Custom ignore files always work even if the `ignore` option is false,
    // so we only add if that option is not present.
    if ignore_dot {
        walker.add_custom_ignore_filename(IGNORE_FILE);
    }

    walker
        .git_exclude(ignore_vcs)
        .git_global(ignore_vcs)
        .git_ignore(ignore_vcs)
        .hidden(config.hidden.map(|b| !b).unwrap_or(true))
        .ignore(ignore_dot)
        .parents(ignore && config.no_ignore_parent.map(|b| !b).unwrap_or(true))
        .threads(threads.max(1));

    walker
}

fn count_sequential(
    walker: WalkBuilder,
    languages: &mut BTreeMap<LanguageType, Language>,
    config: &Config,
    stream_lock: &parking_lot::Mutex<()>,
) {
    for entry in walker.build() {
        let Some(entry) = file_entry(entry) else {
            continue;
        };
        process_file(entry, languages, config, stream_lock);
    }
}

fn count_parallel(
    walker: WalkBuilder,
    languages: &mut BTreeMap<LanguageType, Language>,
    config: &Config,
    stream_lock: &parking_lot::Mutex<()>,
    threads: usize,
) {
    let queue_size = threads
        .saturating_mul(FILES_PER_WORKER)
        .max(FILES_PER_WORKER);
    let (tx, rx) = crossbeam_channel::bounded::<DirEntry>(queue_size);

    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..threads)
            .map(|_| {
                let rx = rx.clone();
                scope.spawn(move || {
                    let mut local = BTreeMap::new();
                    while let Ok(entry) = rx.recv() {
                        process_file(entry, &mut local, config, stream_lock);
                    }
                    local
                })
            })
            .collect();

        drop(rx);

        walker.build_parallel().run(move || {
            let tx = tx.clone();
            Box::new(move |entry| {
                let Some(entry) = file_entry(entry) else {
                    return WalkState::Continue;
                };

                if tx.send(entry).is_err() {
                    WalkState::Quit
                } else {
                    WalkState::Continue
                }
            })
        });

        for worker in workers {
            merge_languages(languages, worker.join().expect("tokei worker panicked"));
        }
    });
}

fn file_entry(entry: Result<DirEntry, ignore::Error>) -> Option<DirEntry> {
    let entry = match entry {
        Ok(entry) => entry,
        Err(error) => {
            log_walk_error(error);
            return None;
        }
    };

    if entry.file_type().is_some_and(|ft| ft.is_file()) {
        Some(entry)
    } else {
        None
    }
}

fn log_walk_error(error: ignore::Error) {
    use ignore::Error;

    if let Error::WithDepth { err: ref error, .. } = error {
        if let Error::WithPath {
            ref path,
            err: ref error,
        } = **error
        {
            error!("{} reading {}", error, path.display());
            return;
        }
    }
    error!("{}", error);
}

fn process_file(
    entry: DirEntry,
    languages: &mut BTreeMap<LanguageType, Language>,
    config: &Config,
    stream_lock: &parking_lot::Mutex<()>,
) {
    let Some(language) = LanguageType::from_path(entry.path(), config) else {
        return;
    };

    if let Some(types) = config.types.as_deref() {
        if !types.contains(&language) {
            return;
        }
    }

    match language.parse(entry.into_path(), config) {
        Ok(stats) => {
            if let Some(callback) = config.for_each_fn {
                let _guard = stream_lock.lock();
                callback(language, stats.clone());
            }
            languages.entry(language).or_default().add_report(stats);
        }
        Err((error, path)) => {
            languages.entry(language).or_default().mark_inaccurate();
            error!("Error reading {}:\n{}", path.display(), error);
        }
    }
}

fn merge_languages(
    dest: &mut BTreeMap<LanguageType, Language>,
    src: BTreeMap<LanguageType, Language>,
) {
    for (language, stats) in src {
        match dest.entry(language) {
            Entry::Occupied(mut entry) => *entry.get_mut() += stats,
            Entry::Vacant(entry) => {
                entry.insert(stats);
            }
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

    use super::IGNORE_FILE;
    use crate::{
        config::Config,
        language::{languages::Languages, LanguageType},
    };

    const FILE_CONTENTS: &[u8] = b"fn main() {}";
    const FILE_NAME: &str = "main.rs";
    const IGNORE_PATTERN: &str = "*.rs";
    const LANGUAGE: &LanguageType = &LanguageType::Rust;

    #[test]
    fn ignore_directory_with_extension() {
        let mut languages = Languages::new();
        let tmp_dir = TempDir::new().expect("Couldn't create temp dir");
        let path_name = tmp_dir.path().join("directory.rs");

        fs::create_dir(path_name).expect("Couldn't create directory.rs within temp");

        super::get_all_files(
            &[tmp_dir.into_path().to_str().unwrap()],
            &[],
            &mut languages,
            &Config::default(),
        );

        assert!(languages.get(LANGUAGE).is_none());
    }

    #[test]
    fn hidden() {
        let dir = TempDir::new().expect("Couldn't create temp dir.");
        let mut config = Config::default();
        let mut languages = Languages::new();

        fs::write(dir.path().join(".hidden.rs"), FILE_CONTENTS).unwrap();

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_none());

        config.hidden = Some(true);

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_some());
    }

    #[test]
    fn no_ignore_implies_dot() {
        let dir = TempDir::new().expect("Couldn't create temp dir.");
        let mut config = Config::default();
        let mut languages = Languages::new();

        fs::write(dir.path().join(".ignore"), IGNORE_PATTERN).unwrap();
        fs::write(dir.path().join(FILE_NAME), FILE_CONTENTS).unwrap();

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_none());

        config.no_ignore = Some(true);

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_some());
    }

    #[test]
    fn no_ignore_implies_vcs_gitignore() {
        let dir = TempDir::new().expect("Couldn't create temp dir.");
        let mut config = Config::default();
        let mut languages = Languages::new();

        git2::Repository::init(dir.path()).expect("Couldn't create git repo.");

        fs::write(dir.path().join(".gitignore"), IGNORE_PATTERN).unwrap();
        fs::write(dir.path().join(FILE_NAME), FILE_CONTENTS).unwrap();

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_none());

        config.no_ignore = Some(true);

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_some());
    }

    #[test]
    fn no_ignore_parent() {
        let parent_dir = TempDir::new().expect("Couldn't create temp dir.");
        let child_dir = parent_dir.path().join("child/");
        let mut config = Config::default();
        let mut languages = Languages::new();

        fs::create_dir_all(&child_dir)
            .unwrap_or_else(|_| panic!("Couldn't create {:?}", child_dir));
        fs::write(parent_dir.path().join(".ignore"), IGNORE_PATTERN)
            .expect("Couldn't create .gitignore.");
        fs::write(child_dir.join(FILE_NAME), FILE_CONTENTS).expect("Couldn't create child.rs");

        super::get_all_files(
            &[child_dir.as_path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_none());

        config.no_ignore_parent = Some(true);

        super::get_all_files(
            &[child_dir.as_path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_some());
    }

    #[test]
    fn no_ignore_dot() {
        let dir = TempDir::new().expect("Couldn't create temp dir.");
        let mut config = Config::default();
        let mut languages = Languages::new();

        fs::write(dir.path().join(".ignore"), IGNORE_PATTERN).unwrap();
        fs::write(dir.path().join(FILE_NAME), FILE_CONTENTS).unwrap();

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_none());

        config.no_ignore_dot = Some(true);

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_some());
    }

    #[test]
    fn no_ignore_dot_still_vcs_gitignore() {
        let dir = TempDir::new().expect("Couldn't create temp dir.");
        let mut config = Config::default();
        let mut languages = Languages::new();

        git2::Repository::init(dir.path()).expect("Couldn't create git repo.");

        fs::write(dir.path().join(".gitignore"), IGNORE_PATTERN).unwrap();
        fs::write(dir.path().join(FILE_NAME), FILE_CONTENTS).unwrap();

        config.no_ignore_dot = Some(true);

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_none());
    }

    #[test]
    fn no_ignore_dot_includes_custom_ignore() {
        let dir = TempDir::new().expect("Couldn't create temp dir.");
        let mut config = Config::default();
        let mut languages = Languages::new();

        fs::write(dir.path().join(IGNORE_FILE), IGNORE_PATTERN).unwrap();
        fs::write(dir.path().join(FILE_NAME), FILE_CONTENTS).unwrap();

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_none());

        config.no_ignore_dot = Some(true);

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_some());
    }

    #[test]
    fn no_ignore_vcs_gitignore() {
        let dir = TempDir::new().expect("Couldn't create temp dir.");
        let mut config = Config::default();
        let mut languages = Languages::new();

        git2::Repository::init(dir.path()).expect("Couldn't create git repo.");

        fs::write(dir.path().join(".gitignore"), IGNORE_PATTERN).unwrap();
        fs::write(dir.path().join(FILE_NAME), FILE_CONTENTS).unwrap();

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_none());

        config.no_ignore_vcs = Some(true);

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_some());
    }

    #[test]
    fn no_ignore_vcs_gitignore_still_dot() {
        let dir = TempDir::new().expect("Couldn't create temp dir.");
        let mut config = Config::default();
        let mut languages = Languages::new();

        fs::write(dir.path().join(".ignore"), IGNORE_PATTERN).unwrap();
        fs::write(dir.path().join(FILE_NAME), FILE_CONTENTS).unwrap();

        config.no_ignore_vcs = Some(true);

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_none());
    }

    #[test]
    fn no_ignore_vcs_gitexclude() {
        let dir = TempDir::new().expect("Couldn't create temp dir.");
        let mut config = Config::default();
        let mut languages = Languages::new();

        git2::Repository::init(dir.path()).expect("Couldn't create git repo.");

        fs::write(dir.path().join(".git/info/exclude"), IGNORE_PATTERN).unwrap();
        fs::write(dir.path().join(FILE_NAME), FILE_CONTENTS).unwrap();

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_none());

        config.no_ignore_vcs = Some(true);

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_some());
    }

    #[test]
    fn custom_ignore() {
        let dir = TempDir::new().expect("Couldn't create temp dir.");
        let config = Config::default();
        let mut languages = Languages::new();

        git2::Repository::init(dir.path()).expect("Couldn't create git repo.");

        fs::write(dir.path().join(IGNORE_FILE), IGNORE_PATTERN).unwrap();
        fs::write(dir.path().join(FILE_NAME), FILE_CONTENTS).unwrap();

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_none());

        fs::remove_file(dir.path().join(IGNORE_FILE)).unwrap();

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        assert!(languages.get(LANGUAGE).is_some());
    }

    #[test]
    fn parallel_matches_sequential_counts() {
        let paths = &["tests/data"];
        let mut sequential = Languages::new();
        sequential.get_statistics(
            paths,
            &[],
            &Config {
                threads: Some(1),
                ..Config::default()
            },
        );

        let mut parallel = Languages::new();
        parallel.get_statistics(
            paths,
            &[],
            &Config {
                threads: Some(8),
                ..Config::default()
            },
        );

        assert_eq!(sequential.len(), parallel.len());
        for (language, seq) in &sequential {
            let par = parallel
                .get(language)
                .expect("missing language in parallel count");
            assert_eq!(seq.blanks, par.blanks, "{language:?} blanks");
            assert_eq!(seq.code, par.code, "{language:?} code");
            assert_eq!(seq.comments, par.comments, "{language:?} comments");
            assert_eq!(seq.reports.len(), par.reports.len(), "{language:?} files");
            assert_eq!(seq.inaccurate, par.inaccurate, "{language:?} inaccurate");
        }
    }
}
