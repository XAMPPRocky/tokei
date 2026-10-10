use std::fs;

use tempfile::TempDir;
use tokei::{Config, LanguageType, Languages};

fn count(source: &str) -> tokei::Language {
    let root = TempDir::new().unwrap();
    fs::write(root.path().join("lib.rs"), source).unwrap();
    let mut languages = Languages::new();
    languages.get_statistics(&[root.path()], &[], &Config::default());
    languages[&LanguageType::Rust].clone()
}

fn assert_partition(source: &str, test_code: usize, other_code: usize) {
    let language = count(source);
    let tests = language.test_statistics();
    let other = language.non_test_statistics();
    assert_eq!(tests.code, test_code);
    assert_eq!(other.code, other_code);
    assert_eq!(tests.lines() + other.lines(), language.lines());
    assert_eq!(tests.reports.len(), 1);
    assert_eq!(other.reports.len(), 1);
    assert_eq!(tests.test_statistics(), tests);
    assert_eq!(other.non_test_statistics(), other);
}

#[test]
fn cfg_test_module_does_not_consume_following_code() {
    assert_partition(
        "pub fn before() {}\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn works() {}\n}\npub fn after() {}\n",
        5,
        2,
    );
}

#[test]
fn test_attribute_marks_a_single_function() {
    assert_partition(
        "fn before() {}\n#[test]\nfn works() {}\nfn after() {}\n",
        2,
        2,
    );
}

#[test]
fn cfg_test_item_inside_regular_module_is_found() {
    assert_partition(
        "mod inner {\n    fn ordinary() {}\n    #[cfg(test)]\n    fn helper() {}\n}\n",
        2,
        3,
    );
}

#[test]
fn attribute_text_in_strings_and_comments_is_ignored() {
    let language = count("const TEXT: &str = \"#[cfg(test)]\";\n// #[test]\nfn normal() {}\n");
    assert!(language.test_statistics().reports.is_empty());
    assert_eq!(language.non_test_statistics().code, language.code);
}

#[test]
fn inline_classification_survives_json_round_trip() {
    let language = count("fn ordinary() {}\n#[cfg(test)]\nmod tests { fn works() {} }\n");
    let json = serde_json::to_string(&language).unwrap();
    let restored: tokei::Language = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.test_statistics(), language.test_statistics());
    assert_eq!(
        restored.non_test_statistics(),
        language.non_test_statistics()
    );
}

#[test]
fn cfg_test_method_inside_impl_is_found() {
    assert_partition(
        "struct Example;\nimpl Example {\n    fn ordinary() {}\n    #[cfg(test)]\n    fn helper() {}\n}\n",
        2,
        4,
    );
}

#[test]
fn rust_documentation_inside_tests_preserves_embedded_totals() {
    let language = count(
        "fn ordinary() {}\n#[cfg(test)]\nmod tests {\n    /// ```rust\n    /// let value = 1;\n    /// ```\n    fn helper() {}\n}\n",
    );
    let tests = language.test_statistics();
    let other = language.non_test_statistics();
    assert_eq!(tests.lines() + other.lines(), language.lines());
    assert_eq!(
        tests.summarise().lines() + other.summarise().lines(),
        language.summarise().lines()
    );
}

#[test]
fn external_test_module_counts_its_helpers_as_tests() {
    let root = TempDir::new().unwrap();
    fs::write(
        root.path().join("lib.rs"),
        "fn ordinary() {}\n#[cfg(test)]\nmod tests;\n",
    )
    .unwrap();
    fs::write(
        root.path().join("tests.rs"),
        "#[test]\nfn works() {}\nfn helper() {}\n",
    )
    .unwrap();

    let mut languages = Languages::new();
    languages.get_statistics(&[root.path()], &[], &Config::default());
    let rust = &languages[&LanguageType::Rust];
    assert_eq!(rust.code, 6);
    assert_eq!(rust.test_statistics().code, 5);
    assert_eq!(rust.non_test_statistics().code, 1);
    let module = rust
        .reports
        .iter()
        .find(|report| report.name.ends_with("tests.rs"))
        .unwrap();
    assert!(module.is_test);
    assert!(module.test_stats.is_none());
}

#[test]
fn nested_external_modules_inherit_test_classification() {
    let root = TempDir::new().unwrap();
    fs::create_dir(root.path().join("tests")).unwrap();
    fs::write(root.path().join("lib.rs"), "#[cfg(test)]\nmod tests;\n").unwrap();
    fs::write(root.path().join("tests.rs"), "mod helpers;\n").unwrap();
    fs::write(root.path().join("tests/helpers.rs"), "fn helper() {}\n").unwrap();

    let mut languages = Languages::new();
    languages.get_statistics(&[root.path()], &[], &Config::default());
    let rust = &languages[&LanguageType::Rust];
    assert_eq!(rust.test_statistics().code, rust.code);
    assert!(
        rust.reports
            .iter()
            .find(|report| report.name.ends_with("helpers.rs"))
            .unwrap()
            .is_test
    );
}

#[test]
fn external_module_path_attribute_is_resolved() {
    let root = TempDir::new().unwrap();
    fs::write(
        root.path().join("lib.rs"),
        "#[cfg(test)]\n#[path = \"support.rs\"]\nmod tests;\n",
    )
    .unwrap();
    fs::write(root.path().join("support.rs"), "fn helper() {}\n").unwrap();

    let mut languages = Languages::new();
    languages.get_statistics(&[root.path()], &[], &Config::default());
    let rust = &languages[&LanguageType::Rust];
    assert!(
        rust.reports
            .iter()
            .find(|report| report.name.ends_with("support.rs"))
            .unwrap()
            .is_test
    );
}

#[test]
fn inline_test_module_resolves_external_children() {
    let root = TempDir::new().unwrap();
    fs::create_dir(root.path().join("tests")).unwrap();
    fs::write(
        root.path().join("lib.rs"),
        "#[cfg(test)]\nmod tests {\n    mod helpers;\n}\n",
    )
    .unwrap();
    fs::write(root.path().join("tests/helpers.rs"), "fn helper() {}\n").unwrap();

    let mut languages = Languages::new();
    languages.get_statistics(&[root.path()], &[], &Config::default());
    let rust = &languages[&LanguageType::Rust];
    assert!(
        rust.reports
            .iter()
            .find(|report| report.name.ends_with("helpers.rs"))
            .unwrap()
            .is_test
    );
}

#[test]
fn inline_test_module_resolves_path_attribute() {
    let root = TempDir::new().unwrap();
    fs::create_dir(root.path().join("tests")).unwrap();
    fs::write(
        root.path().join("lib.rs"),
        "#[cfg(test)]\nmod tests {\n    #[path = \"custom.rs\"]\n    mod helpers;\n}\n",
    )
    .unwrap();
    fs::write(root.path().join("tests/custom.rs"), "fn helper() {}\n").unwrap();

    let mut languages = Languages::new();
    languages.get_statistics(&[root.path()], &[], &Config::default());
    let rust = &languages[&LanguageType::Rust];
    assert!(
        rust.reports
            .iter()
            .find(|report| report.name.ends_with("custom.rs"))
            .unwrap()
            .is_test
    );
}

#[test]
fn ordinary_external_module_remains_outside_tests() {
    let root = TempDir::new().unwrap();
    fs::write(root.path().join("lib.rs"), "mod ordinary;\n").unwrap();
    fs::write(root.path().join("ordinary.rs"), "fn helper() {}\n").unwrap();

    let mut languages = Languages::new();
    languages.get_statistics(&[root.path()], &[], &Config::default());
    let rust = &languages[&LanguageType::Rust];
    assert!(rust.test_statistics().reports.is_empty());
    assert_eq!(rust.non_test_statistics().code, rust.code);
}

#[test]
fn cargo_integration_test_files_count_in_full() {
    let root = TempDir::new().unwrap();
    fs::create_dir_all(root.path().join("src/tests")).unwrap();
    fs::create_dir_all(root.path().join("tests/support")).unwrap();
    fs::write(
        root.path().join("Cargo.toml"),
        "[package]\nname = \"sample\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    fs::write(root.path().join("src/lib.rs"), "fn ordinary() {}\n").unwrap();
    fs::write(
        root.path().join("src/tests/helper.rs"),
        "fn ordinary_helper() {}\n",
    )
    .unwrap();
    fs::write(
        root.path().join("tests/integration.rs"),
        "#[test]\nfn works() {}\nfn helper() {}\n",
    )
    .unwrap();
    fs::write(
        root.path().join("tests/support/data.rs"),
        "fn fixture() {}\n",
    )
    .unwrap();

    let mut languages = Languages::new();
    languages.get_statistics(&[root.path()], &[], &Config::default());
    let rust = &languages[&LanguageType::Rust];
    assert_eq!(rust.test_statistics().code, 4);
    assert_eq!(rust.non_test_statistics().code, 2);
    for report in &rust.reports {
        let under_integration_tests = report.name.starts_with(root.path().join("tests"));
        assert_eq!(report.is_test, under_integration_tests);
        if under_integration_tests {
            assert!(report.test_stats.is_none());
        }
    }
}

#[test]
fn tests_directory_without_cargo_manifest_is_not_classified() {
    let root = TempDir::new().unwrap();
    fs::create_dir(root.path().join("tests")).unwrap();
    fs::write(root.path().join("tests/helper.rs"), "fn helper() {}\n").unwrap();

    let mut languages = Languages::new();
    languages.get_statistics(&[root.path()], &[], &Config::default());
    let rust = &languages[&LanguageType::Rust];
    assert!(rust.test_statistics().reports.is_empty());
    assert_eq!(rust.non_test_statistics().code, 1);
}
