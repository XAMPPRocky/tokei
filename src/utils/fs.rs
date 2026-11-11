use std::{collections::BTreeMap, path::Path};

use ignore::{
    overrides::OverrideBuilder, DirEntry, ParallelVisitor, ParallelVisitorBuilder, WalkBuilder,
    WalkState::{self, Continue},
};
use parking_lot::Mutex;

use crate::{
    config::Config,
    language::{Language, LanguageType},
};

const IGNORE_FILE: &str = ".tokeignore";

pub fn get_all_files<A: AsRef<Path>>(
    paths: &[A],
    ignored_directories: &[&str],
    languages: &mut BTreeMap<LanguageType, Language>,
    config: &Config,
) {
    let mut paths_iter = paths.iter();
    let mut walker = WalkBuilder::new(paths_iter.next().unwrap());

    for path in paths_iter {
        walker.add(path);
    }

    let ignore = config.no_ignore.map(|b| !b).unwrap_or(true);
    let ignore_dot = ignore && config.no_ignore_dot.map(|b| !b).unwrap_or(true);
    let ignore_vcs = ignore && config.no_ignore_vcs.map(|b| !b).unwrap_or(true);
    let ignore_parents = ignore && config.no_ignore_parent.map(|b| !b).unwrap_or(true);

    let gitattributes_excludes = if ignore_vcs {
        collect_gitattributes_excludes(paths, ignore_parents)
    } else {
        Vec::new()
    };

    if !ignored_directories.is_empty() || !gitattributes_excludes.is_empty() {
        let mut overrides = OverrideBuilder::new(".");

        for ignored in ignored_directories {
            rs_error!(overrides.add(&format!("!{}", ignored)));
        }

        for pattern in &gitattributes_excludes {
            rs_error!(overrides.add(&format!("!{}", pattern)));
        }

        walker.overrides(overrides.build().expect("Excludes provided were invalid"));
    }

    // Custom ignore files always work even if the `ignore` option is false,
    // so we only add if that option is not present.
    if ignore_dot {
        walker.add_custom_ignore_filename(IGNORE_FILE);
    }

    // `ignore` only uses 2 threads by default; since files are parsed on the
    // walker's threads, use all available cores.
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get());

    walker
        .threads(threads)
        .git_exclude(ignore_vcs)
        .git_global(ignore_vcs)
        .git_ignore(ignore_vcs)
        .hidden(config.hidden.map(|b| !b).unwrap_or(true))
        .ignore(ignore_dot)
        .parents(ignore_parents);

    // Files are parsed directly on the walker's worker threads, so that
    // directory traversal and parsing overlap. Each thread accumulates its
    // results locally and merges them into `languages` once, when it's done.
    let languages = Mutex::new(languages);
    walker.build_parallel().visit(&mut FileVisitorBuilder {
        config,
        languages: &languages,
    });
}

fn collect_gitattributes_excludes<A: AsRef<Path>>(paths: &[A], check_parents: bool) -> Vec<String> {
    let mut patterns = Vec::new();
    let mut visited = std::collections::HashSet::new();

    for path in paths {
        let path = path.as_ref();
        let start_dir = if path.is_dir() {
            Some(path)
        } else {
            path.parent()
        };
        let Some(mut curr) = start_dir else {
            continue;
        };

        loop {
            let attr_path = curr.join(".gitattributes");
            if visited.insert(attr_path.clone()) {
                if let Ok(contents) = std::fs::read_to_string(&attr_path) {
                    parse_gitattributes(&contents, &mut patterns);
                }
            }
            if !check_parents || curr.join(".git").exists() {
                break;
            }
            match curr.parent() {
                Some(p) if p != curr => curr = p,
                _ => break,
            }
        }
    }

    patterns
}

fn parse_gitattributes(contents: &str, patterns: &mut Vec<String>) {
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let Some(pattern) = parts.next() else {
            continue;
        };
        let mut should_ignore = false;
        for attr in parts {
            let (name, val) = match attr.split_once('=') {
                Some((k, v)) => (k, Some(v)),
                None => (attr, None),
            };
            let (negated, key) = if let Some(rest) = name.strip_prefix('-') {
                (true, rest)
            } else {
                (false, name)
            };
            if matches!(
                key,
                "linguist-generated" | "linguist-vendored" | "linguist-documentation"
            ) {
                let enabled = !negated && !matches!(val, Some("false" | "0"));
                should_ignore = enabled;
            }
        }
        if should_ignore {
            patterns.push(pattern.to_string());
        }
    }
}

struct FileVisitorBuilder<'a, 'b> {
    config: &'a Config,
    languages: &'a Mutex<&'b mut BTreeMap<LanguageType, Language>>,
}

impl<'s, 'a: 's, 'b: 's> ParallelVisitorBuilder<'s> for FileVisitorBuilder<'a, 'b> {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        Box::new(FileVisitor {
            config: self.config,
            languages: self.languages,
            local: BTreeMap::new(),
        })
    }
}

struct FileVisitor<'a, 'b> {
    config: &'a Config,
    languages: &'a Mutex<&'b mut BTreeMap<LanguageType, Language>>,
    local: BTreeMap<LanguageType, Language>,
}

impl ParallelVisitor for FileVisitor<'_, '_> {
    fn visit(&mut self, entry: Result<DirEntry, ignore::Error>) -> WalkState {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                use ignore::Error;
                if let Error::WithDepth { err: ref error, .. } = error {
                    if let Error::WithPath {
                        ref path,
                        err: ref error,
                    } = **error
                    {
                        error!("{} reading {}", error, path.display());
                        return Continue;
                    }
                }
                error!("{}", error);
                return Continue;
            }
        };

        if !entry.file_type().map_or(false, |ft| ft.is_file()) {
            return Continue;
        }

        let config = self.config;
        let Some(language) = LanguageType::from_path(entry.path(), config) else {
            return Continue;
        };

        if config.types.as_deref().map_or(false, |t| !t.contains(&language)) {
            return Continue;
        }

        let result = language.parse(entry.into_path(), config);
        match result {
            Ok(stats) => {
                let lines = stats.stats.lines();
                if config.min_lines.map_or(false, |min| lines < min) {
                    return Continue;
                }
                if config.max_lines.map_or(false, |max| lines > max) {
                    return Continue;
                }
                if let Some(f) = config.for_each_fn {
                    f(language, stats.clone())
                };
                self.local.entry(language).or_default().add_report(stats);
            }
            Err((error, path)) => {
                self.local.entry(language).or_default().mark_inaccurate();
                error!("Error reading {}:\n{}", path.display(), error);
            }
        }

        Continue
    }
}

impl Drop for FileVisitor<'_, '_> {
    fn drop(&mut self) {
        if self.local.is_empty() {
            return;
        }

        let mut languages = self.languages.lock();
        for (language_type, language) in std::mem::take(&mut self.local) {
            *languages.entry(language_type).or_default() += language;
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
    fn gitattributes_linguist_generated() {
        let dir = TempDir::new().expect("Couldn't create temp dir.");
        let mut config = Config::default();
        let mut languages = Languages::new();

        fs::write(
            dir.path().join(".gitattributes"),
            "generated.rs linguist-generated\nkept.rs linguist-generated=false\n",
        )
        .unwrap();
        fs::write(dir.path().join("generated.rs"), FILE_CONTENTS).unwrap();
        fs::write(dir.path().join("kept.rs"), FILE_CONTENTS).unwrap();

        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        let rust = languages.get(LANGUAGE).expect("Expected Rust language");
        assert_eq!(rust.reports.len(), 1);
        assert!(rust.reports[0].name.ends_with("kept.rs"));

        config.no_ignore_vcs = Some(true);
        let mut languages = Languages::new();
        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        let rust = languages.get(LANGUAGE).expect("Expected Rust language");
        assert_eq!(rust.reports.len(), 2);
    }

    #[test]
    fn min_and_max_lines_filter() {
        let dir = TempDir::new().expect("Couldn't create temp dir.");
        fs::write(dir.path().join("short.rs"), "fn a() {}\n").unwrap();
        fs::write(
            dir.path().join("medium.rs"),
            "fn a() {}\nfn b() {}\nfn c() {}\n",
        )
        .unwrap();
        fs::write(
            dir.path().join("long.rs"),
            "fn a() {}\nfn b() {}\nfn c() {}\nfn d() {}\nfn e() {}\n",
        )
        .unwrap();

        let config = Config {
            min_lines: Some(2),
            max_lines: Some(4),
            ..Config::default()
        };
        let mut languages = Languages::new();
        super::get_all_files(
            &[dir.path().to_str().unwrap()],
            &[],
            &mut languages,
            &config,
        );

        let rust = languages.get(LANGUAGE).expect("Expected Rust language");
        assert_eq!(rust.reports.len(), 1);
        assert!(rust.reports[0].name.ends_with("medium.rs"));
    }
}
