use crate::language::LanguageType;
use globset::GlobMatcher;
use std::str::FromStr;

/// A parsed classification pattern.
/// Format: `CategoryName:pattern` or `Language:CategoryName:pattern`
#[derive(Debug, Clone)]
pub struct ClassificationPattern {
    /// The classification category (e.g., "Tests", "Generated", "Benchmarks")
    pub category: String,
    /// Optional language this pattern applies to (None = applies to all languages)
    pub language: Option<LanguageType>,
    /// The precompiled glob matcher to match files against
    pub pattern: GlobMatcher,
}

impl ClassificationPattern {
    /// Parse a classification pattern string.
    ///
    /// Formats:
    /// - Folder shorthand: `folder` (e.g., `tests` → `tests:tests/**/*`)
    /// - Global: `CategoryName:pattern` (e.g., `Tests:**/*.test.*`)
    /// - Language-specific: `Language:CategoryName:pattern` (e.g., `JavaScript:Benchmarks:**/*_bench.js`)
    pub fn parse(input: &str) -> Result<Self, String> {
        if input.is_empty() {
            return Err("Pattern cannot be empty".to_string());
        }

        let parts: Vec<&str> = input.splitn(3, ':').collect();

        match parts.len() {
            1 => {
                // Folder shorthand: "tests" -> "tests:tests/**/*"
                let folder = parts[0].trim_end_matches('/');
                let pattern_str = format!("{}/**/*", folder);
                let pattern = globset::Glob::new(&pattern_str)
                    .map_err(|e| format!("Invalid glob pattern: {}", e))?
                    .compile_matcher();
                Ok(ClassificationPattern {
                    category: folder.to_string(),
                    language: None,
                    pattern,
                })
            }
            2 => {
                // Global pattern: CategoryName:pattern
                let pattern = globset::Glob::new(parts[1])
                    .map_err(|e| format!("Invalid glob pattern '{}': {}", parts[1], e))?
                    .compile_matcher();
                Ok(ClassificationPattern {
                    category: parts[0].to_string(),
                    language: None,
                    pattern,
                })
            }
            3 => {
                // Language-specific: Language:CategoryName:pattern
                let language = LanguageType::from_str(parts[0])
                    .map_err(|e| format!("Invalid language '{}': {}", parts[0], e))?;
                let pattern = globset::Glob::new(parts[2])
                    .map_err(|e| format!("Invalid glob pattern '{}': {}", parts[2], e))?
                    .compile_matcher();
                Ok(ClassificationPattern {
                    category: parts[1].to_string(),
                    language: Some(language),
                    pattern,
                })
            }
            _ => Err(format!("Invalid pattern format: '{}'", input)),
        }
    }
}

/// Classifies a file based on classification patterns.
///
/// # Arguments
/// * `path` - The relative path to the file
/// * `language` - The language type of the file
/// * `patterns` - The classification patterns to check against
pub fn classify_file(
    path: &std::path::Path,
    language: &LanguageType,
    patterns: &[ClassificationPattern],
) -> Option<String> {
    let path_str = path.to_str()?;

    for pattern in patterns {
        // Skip language-specific patterns that don't match this file's language
        if let Some(pattern_lang) = &pattern.language {
            if pattern_lang != language {
                continue;
            }
        }

        // Check if the path matches the precompiled glob pattern
        if pattern.pattern.is_match(path_str) {
            return Some(pattern.category.clone());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_global_pattern() {
        let pattern = "Tests:**/*.test.*";
        let parsed = ClassificationPattern::parse(pattern).unwrap();

        assert_eq!(parsed.category, "Tests");
        assert_eq!(parsed.language, None);
        assert!(parsed.pattern.is_match("src/app.test.js"));
    }

    #[test]
    fn test_parse_language_specific_pattern() {
        let pattern = "JavaScript:Benchmarks:**/*_bench.js";
        let parsed = ClassificationPattern::parse(pattern).unwrap();

        assert_eq!(parsed.category, "Benchmarks");
        assert_eq!(parsed.language, Some(LanguageType::JavaScript));
        assert!(parsed.pattern.is_match("perf/render_bench.js"));
    }

    #[test]
    fn test_parse_pattern_with_colon_in_glob() {
        let pattern = "Generated:**/*.generated.*";
        let parsed = ClassificationPattern::parse(pattern).unwrap();

        assert_eq!(parsed.category, "Generated");
        assert_eq!(parsed.language, None);
        assert!(parsed.pattern.is_match("src/schema.generated.ts"));
    }

    #[test]
    fn test_parse_invalid_language() {
        let pattern = "InvalidLanguage:Tests:**/*.test.*";
        let parsed = ClassificationPattern::parse(pattern);
        assert!(parsed.is_err());
    }

    #[test]
    fn test_parse_folder_shorthand() {
        let pattern = "tests";
        let parsed = ClassificationPattern::parse(pattern).unwrap();

        assert_eq!(parsed.category, "tests");
        assert_eq!(parsed.language, None);
        assert!(parsed.pattern.is_match("tests/unit/app.test.js"));
    }

    #[test]
    fn test_parse_folder_shorthand_ending_in_slash() {
        let pattern = "tests/";
        let parsed = ClassificationPattern::parse(pattern).unwrap();

        assert_eq!(parsed.category, "tests");
        assert_eq!(parsed.language, None);
        assert!(parsed.pattern.is_match("tests/unit/app.test.js"));
    }

    #[test]
    fn test_parse_invalid_format() {
        // Empty string should still be an error
        let pattern = "";
        let parsed = ClassificationPattern::parse(pattern);
        assert!(parsed.is_err());
    }

    #[test]
    fn test_parse_invalid_glob_pattern() {
        let pattern = "Tests:**/[invalid";
        let parsed = ClassificationPattern::parse(pattern);
        assert!(parsed.is_err());
    }

    #[test]
    fn test_classify_file_with_global_pattern() {
        use std::path::Path;

        let patterns = vec![
            ClassificationPattern::parse("Tests:**/*.test.*").unwrap(),
            ClassificationPattern::parse("Generated:**/*.generated.*").unwrap(),
        ];

        // Should match test pattern
        let classification = classify_file(
            Path::new("src/main.test.js"),
            &LanguageType::JavaScript,
            &patterns,
        );
        assert_eq!(classification, Some("Tests".to_string()));

        // Should match generated pattern
        let classification = classify_file(
            Path::new("src/schema.generated.ts"),
            &LanguageType::TypeScript,
            &patterns,
        );
        assert_eq!(classification, Some("Generated".to_string()));

        // Should not match any pattern
        let classification = classify_file(
            Path::new("src/main.js"),
            &LanguageType::JavaScript,
            &patterns,
        );
        assert_eq!(classification, None);
    }

    #[test]
    fn test_classify_file_with_language_specific_pattern() {
        use std::path::Path;

        let patterns = vec![
            ClassificationPattern::parse("JavaScript:Benchmarks:**/*_bench.js").unwrap(),
            ClassificationPattern::parse("Tests:**/*.test.*").unwrap(),
        ];

        // Should match language-specific benchmark pattern (JS only)
        let classification = classify_file(
            Path::new("perf/render_bench.js"),
            &LanguageType::JavaScript,
            &patterns,
        );
        assert_eq!(classification, Some("Benchmarks".to_string()));

        // Should NOT match for TypeScript (language doesn't match)
        let classification = classify_file(
            Path::new("perf/render_bench.js"),
            &LanguageType::TypeScript,
            &patterns,
        );
        assert_eq!(classification, None);

        // Should match global test pattern for any language
        let classification = classify_file(
            Path::new("src/app.test.ts"),
            &LanguageType::TypeScript,
            &patterns,
        );
        assert_eq!(classification, Some("Tests".to_string()));
    }
}
