#![cfg(feature = "cli")]

use std::{fs, path::Path, process::Command};

use tempfile::TempDir;

const OTHER: &str = "package example\nfunc main() {}\n";
const TEST: &str =
    "package example\n\n// Test helper\n// Another comment\nfunc helper() {}\nfunc second() {}\n";

fn project() -> TempDir {
    let root = TempDir::new().unwrap();
    fs::write(root.path().join("main.go"), OTHER).unwrap();
    fs::write(root.path().join("main_test.go"), TEST).unwrap();
    root
}

fn run(root: &Path, args: &[&str]) -> String {
    // Cargo applies the target runner to this test, but not to its subprocesses.
    let runner_key = format!(
        "CARGO_TARGET_{}_RUNNER",
        env!("TOKEI_TEST_TARGET").replace('-', "_").to_uppercase()
    );
    let runner = std::env::var(&runner_key).unwrap_or_default();
    let mut runner = runner.split_whitespace();
    let mut command = if let Some(program) = runner.next() {
        let mut command = Command::new(program);
        command.args(runner).arg(env!("CARGO_BIN_EXE_tokei"));
        command
    } else {
        Command::new(env!("CARGO_BIN_EXE_tokei"))
    };
    let output = command
        .current_dir(root)
        .env("NO_COLOR", "1")
        .args(args)
        .arg(".")
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    String::from_utf8(output.stdout).unwrap()
}

fn row<'a>(output: &'a str, name: &str) -> Vec<&'a str> {
    output
        .lines()
        .map(|line| line.trim_start().strip_prefix("|- ").unwrap_or(line))
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .find(|columns| columns.first() == Some(&name))
        .unwrap_or_else(|| panic!("Missing {name} row in:\n{output}"))
}

#[test]
fn default_output_partitions_tests_before_total() {
    let root = project();
    let output = run(root.path(), &[]);
    assert_eq!(row(&output, "Go"), ["Go", "1", "2", "2", "0", "0"]);
    assert_eq!(row(&output, "Tests"), ["Tests", "1", "6", "3", "2", "1"]);
    assert_eq!(row(&output, "(Total)"), ["(Total)", "8", "5", "2", "1"]);
    assert_eq!(row(&output, "Total"), ["Total", "2", "8", "5", "2", "1"]);
    assert!(output.find("Tests").unwrap() < output.find("(Total)").unwrap());
    assert!(output.lines().any(|line| line.starts_with(" |- Tests")));
    assert!(!output.lines().any(|line| line.starts_with(" |- Other")));
    assert!(output.lines().any(|line| line.starts_with(" (Total)")));
}

#[test]
fn compact_output_keeps_inclusive_totals() {
    let root = project();
    let output = run(root.path(), &["--compact"]);
    assert_eq!(row(&output, "Go"), ["Go", "2", "8", "5", "2", "1"]);
    assert!(!output.contains("Tests"));
    assert!(!output.contains("Other"));
}

#[test]
fn file_listing_keeps_default_breakdown() {
    let root = project();
    let output = run(root.path(), &["--files"]);
    assert_eq!(row(&output, "Tests"), ["Tests", "1", "6", "3", "2", "1"]);
    assert!(output.contains("main_test.go"));
    assert!(output.contains("main.go"));
}

#[test]
fn compact_file_listing_hides_breakdown() {
    let root = project();
    let output = run(root.path(), &["--compact", "--files"]);
    assert!(!output.contains("Tests"));
    assert!(!output.contains("Other"));
    assert!(output.contains("main_test.go"));
}

#[test]
fn sorting_keeps_test_breakdown() {
    let root = project();
    for option in ["--sort", "--rsort"] {
        let output = run(root.path(), &[option, "code"]);
        assert_eq!(row(&output, "Tests"), ["Tests", "1", "6", "3", "2", "1"]);
        assert_eq!(row(&output, "Go"), ["Go", "1", "2", "2", "0", "0"]);
    }
}

#[test]
fn all_test_files_leave_language_row_without_counts() {
    let root = TempDir::new().unwrap();
    fs::write(root.path().join("main_test.go"), TEST).unwrap();
    let output = run(root.path(), &[]);
    assert_eq!(row(&output, "Go"), ["Go"]);
    assert_eq!(row(&output, "Tests"), ["Tests", "1", "6", "3", "2", "1"]);
}

#[test]
fn unclassified_files_keep_single_language_row() {
    let root = TempDir::new().unwrap();
    fs::write(root.path().join("main.go"), OTHER).unwrap();
    let output = run(root.path(), &[]);
    assert_eq!(row(&output, "Go"), ["Go", "1", "2", "2", "0", "0"]);
    assert!(!output.contains("Tests"));
    assert!(!output.contains("(Total)"));
}

#[test]
fn streaming_json_identifies_test_files() {
    let root = project();
    let output = run(root.path(), &["--streaming", "json"]);
    let reports: Vec<serde_json::Value> = output
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(reports.len(), 2);
    for report in reports {
        let name = report["stats"]["name"].as_str().unwrap();
        assert_eq!(report["stats"]["is_test"], name.ends_with("main_test.go"));
    }
}

#[test]
fn streaming_json_identifies_external_rust_test_modules() {
    let root = TempDir::new().unwrap();
    fs::write(root.path().join("lib.rs"), "#[cfg(test)]\nmod tests;\n").unwrap();
    fs::write(root.path().join("tests.rs"), "fn helper() {}\n").unwrap();

    let output = run(root.path(), &["--streaming", "json"]);
    let reports: Vec<serde_json::Value> = output
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(reports.len(), 2);
    let module = reports
        .iter()
        .find(|report| {
            report["stats"]["name"]
                .as_str()
                .unwrap()
                .ends_with("tests.rs")
        })
        .unwrap();
    assert_eq!(module["stats"]["is_test"], true);
}

#[test]
fn cargo_integration_tests_appear_in_output_and_streaming_json() {
    let root = TempDir::new().unwrap();
    fs::create_dir(root.path().join("tests")).unwrap();
    fs::write(
        root.path().join("Cargo.toml"),
        "[package]\nname = \"sample\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    fs::write(root.path().join("lib.rs"), "fn ordinary() {}\n").unwrap();
    fs::write(root.path().join("tests/integration.rs"), "fn helper() {}\n").unwrap();

    let output = run(root.path(), &[]);
    assert_eq!(row(&output, "Rust"), ["Rust", "1", "1", "1", "0", "0"]);
    assert_eq!(row(&output, "Tests"), ["Tests", "1", "1", "1", "0", "0"]);
    assert_eq!(row(&output, "(Total)"), ["(Total)", "2", "2", "0", "0"]);

    let output = run(root.path(), &["--streaming", "json"]);
    let reports: Vec<serde_json::Value> = output
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let integration = reports
        .iter()
        .find(|report| {
            report["stats"]["name"]
                .as_str()
                .unwrap()
                .ends_with("integration.rs")
        })
        .unwrap();
    assert_eq!(integration["stats"]["is_test"], true);
    assert!(integration["stats"].get("test_stats").is_none());
}

#[test]
fn rust_inline_tests_show_overlapping_file_counts_and_partitioned_lines() {
    let root = TempDir::new().unwrap();
    fs::write(
        root.path().join("lib.rs"),
        "fn ordinary() {}\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn works() {}\n}\nfn after() {}\n",
    )
    .unwrap();
    let output = run(root.path(), &[]);
    assert_eq!(row(&output, "Rust"), ["Rust", "1", "2", "2", "0", "0"]);
    assert_eq!(row(&output, "Tests"), ["Tests", "1", "5", "5", "0", "0"]);
    assert_eq!(row(&output, "(Total)"), ["(Total)", "7", "7", "0", "0"]);
    assert_eq!(row(&output, "Total"), ["Total", "1", "7", "7", "0", "0"]);
}

#[test]
fn rust_tests_preserve_embedded_markdown_row() {
    let root = TempDir::new().unwrap();
    fs::write(
        root.path().join("lib.rs"),
        "/// Intro\nfn ordinary() {}\n#[cfg(test)]\nmod tests {\n    /// ```rust\n    /// let value = 1;\n    /// ```\n    fn helper() {}\n}\n",
    )
    .unwrap();
    let output = run(root.path(), &[]);
    assert_eq!(row(&output, "Rust"), ["Rust", "1", "1", "1", "0", "0"]);
    assert_eq!(row(&output, "Tests"), ["Tests", "1", "4", "4", "0", "0"]);
    assert_eq!(
        row(&output, "Markdown"),
        ["Markdown", "1", "4", "1", "3", "0"]
    );
    assert_eq!(row(&output, "(Total)"), ["(Total)", "9", "6", "3", "0"]);
    assert_eq!(row(&output, "Total"), ["Total", "1", "9", "6", "3", "0"]);
    assert!(output.find("Tests").unwrap() < output.find("Markdown").unwrap());
    assert!(output.find("Markdown").unwrap() < output.find("(Total)").unwrap());
}
