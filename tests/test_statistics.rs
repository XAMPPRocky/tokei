use std::{fs, path::Path};

use tempfile::TempDir;
use tokei::{Config, LanguageType, Languages};

fn write_file(root: &Path, name: &str) -> std::path::PathBuf {
    let path = root.join(name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, "package example\nfunc helper() {}\n").unwrap();
    path
}

#[test]
fn saved_statistics_merge_with_explicit_test_file_input() {
    let root = TempDir::new().unwrap();
    let first = write_file(root.path(), "one_test.go");
    let second = write_file(root.path(), "nested/two_test.go");
    let mut languages = Languages::new();
    languages.get_statistics(&[first], &[], &Config::default());
    let json = serde_json::to_string(&languages).unwrap();
    let mut restored: Languages = serde_json::from_str(&json).unwrap();
    assert_eq!(restored, languages);
    restored.get_statistics(&[second], &[], &Config::default());
    let tests = restored[&LanguageType::Go].test_statistics();
    assert_eq!(tests.code, 4);
    assert_eq!(tests.reports.len(), 2);
}

#[test]
fn excluded_test_files_do_not_contribute_to_subtotals() {
    let root = TempDir::new().unwrap();
    write_file(root.path(), "one_test.go");
    write_file(root.path(), "nested/two_test.go");
    let mut languages = Languages::new();
    languages.get_statistics(&[root.path()], &["nested"], &Config::default());
    let tests = languages[&LanguageType::Go].test_statistics();
    assert_eq!(tests.code, 2);
    assert_eq!(tests.reports.len(), 1);
}
