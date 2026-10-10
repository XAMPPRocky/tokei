//! Find test-only Rust items without changing the line counter's source text.

use std::{
    collections::{HashMap, VecDeque},
    fs,
    path::{Path, PathBuf},
};

use proc_macro2::Span;
use syn::{spanned::Spanned, Attribute, Item, Lit, Meta};

use crate::{config::Config, language::Language, stats::CodeStats, LanguageType};

/// Rust files beneath a package's top-level `tests` directory are integration tests.
pub(super) fn is_cargo_test_file(path: &Path) -> bool {
    path.ancestors().skip(1).any(|directory| {
        directory.file_name().is_some_and(|name| name == "tests")
            && directory
                .parent()
                .is_some_and(|root| root.join("Cargo.toml").is_file())
    })
}

/// Carry a test-only module's classification into files reached through `mod`.
pub(super) fn classify_external_modules(language: &mut Language) {
    let paths: HashMap<PathBuf, usize> = language
        .reports
        .iter()
        .enumerate()
        .map(|(index, report)| (report.name.clone(), index))
        .collect();
    let mut pending = VecDeque::new();

    for report in &language.reports {
        if report.test_stats.is_some() {
            pending.extend(external_modules(&report.name, false));
        }
    }

    while let Some(path) = pending.pop_front() {
        let Some(&index) = paths.get(&path) else {
            continue;
        };
        let report = &mut language.reports[index];
        if report.is_test {
            continue;
        }
        report.is_test = true;
        report.test_stats = None;
        pending.extend(external_modules(&report.name, true));
    }
}

fn external_modules(path: &Path, inherited_test: bool) -> Vec<PathBuf> {
    let Ok(source) = fs::read_to_string(path) else {
        return Vec::new();
    };
    let Ok(file) = syn::parse_file(&source) else {
        return Vec::new();
    };
    let mut result = Vec::new();
    collect_external_modules(
        &file.items,
        path,
        &module_directory(path),
        inherited_test,
        &mut result,
    );
    result
}

fn collect_external_modules(
    items: &[Item],
    source: &Path,
    directory: &Path,
    inherited_test: bool,
    result: &mut Vec<PathBuf>,
) {
    for item in items {
        let Item::Mod(module) = item else {
            continue;
        };
        let is_test = inherited_test || module.attrs.iter().any(is_test_attribute);
        if let Some((_, contents)) = &module.content {
            let nested_directory = directory.join(&module.ident.to_string());
            collect_external_modules(contents, source, &nested_directory, is_test, result);
        } else if is_test {
            if let Some(path) = module_path(source, directory, module) {
                result.push(path);
            }
        }
    }
}

fn module_directory(path: &Path) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    match path.file_stem().and_then(|stem| stem.to_str()) {
        Some("lib" | "main" | "mod") | None => parent.to_path_buf(),
        Some(stem) => parent.join(stem),
    }
}

fn module_path(source: &Path, directory: &Path, module: &syn::ItemMod) -> Option<PathBuf> {
    let attribute_directory = if directory == module_directory(source) {
        source.parent()?
    } else {
        directory
    };
    for attribute in &module.attrs {
        if !attribute.path().is_ident("path") {
            continue;
        }
        if let Meta::NameValue(value) = &attribute.meta {
            if let syn::Expr::Lit(expr) = &value.value {
                if let Lit::Str(path) = &expr.lit {
                    return Some(attribute_directory.join(path.value()));
                }
            }
        }
    }

    let stem = directory.join(module.ident.to_string());
    let file = stem.with_extension("rs");
    if file.exists() {
        Some(file)
    } else {
        let file = stem.join("mod.rs");
        file.exists().then_some(file)
    }
}

/// Count complete lines belonging to `#[cfg(test)]` items or `#[test]` functions.
/// A source file that syn cannot parse keeps its ordinary totals and has no
/// inferred inline test subset.
pub(super) fn statistics(text: &[u8], config: &Config) -> Option<CodeStats> {
    if !text.windows(2).any(|window| window == b"#[")
        || !text.windows(4).any(|window| window == b"test")
    {
        return None;
    }
    let source = std::str::from_utf8(text).ok()?;
    let file = syn::parse_file(source).ok()?;
    let lines: Vec<&str> = source.split_inclusive('\n').collect();
    let mut selected = vec![false; lines.len()];
    mark_items(&file.items, &mut selected);
    if !selected.iter().any(|selected| *selected) {
        return None;
    }

    let mut test_source = String::with_capacity(source.len());
    for (line, selected) in lines.iter().zip(&selected) {
        if *selected {
            test_source.push_str(line);
        }
    }
    Some(LanguageType::Rust.parse_from_str(test_source, config))
}

fn mark_items(items: &[Item], selected: &mut [bool]) {
    for item in items {
        let attributes = item_attributes(item);
        if let Some(attributes) = attributes {
            if mark_item(attributes, item.span(), selected) {
                continue;
            }
        }
        match item {
            Item::Mod(module) => {
                if let Some((_, contents)) = &module.content {
                    mark_items(contents, selected);
                }
            }
            Item::Impl(implementation) => {
                for member in &implementation.items {
                    if let syn::ImplItem::Fn(function) = member {
                        mark_item(&function.attrs, function.span(), selected);
                    }
                }
            }
            _ => {}
        }
    }
}

fn mark_item(attributes: &[Attribute], item: Span, selected: &mut [bool]) -> bool {
    if !attributes.iter().any(is_test_attribute) {
        return false;
    }
    let start = attributes.first().map(Spanned::span).unwrap_or(item);
    mark_range(start, item, selected);
    true
}

fn item_attributes(item: &Item) -> Option<&[Attribute]> {
    match item {
        Item::Const(item) => Some(&item.attrs),
        Item::Enum(item) => Some(&item.attrs),
        Item::Fn(item) => Some(&item.attrs),
        Item::Impl(item) => Some(&item.attrs),
        Item::Macro(item) => Some(&item.attrs),
        Item::Mod(item) => Some(&item.attrs),
        Item::Static(item) => Some(&item.attrs),
        Item::Struct(item) => Some(&item.attrs),
        Item::Trait(item) => Some(&item.attrs),
        Item::Type(item) => Some(&item.attrs),
        Item::Union(item) => Some(&item.attrs),
        Item::Use(item) => Some(&item.attrs),
        _ => None,
    }
}

fn is_test_attribute(attribute: &Attribute) -> bool {
    if attribute.path().is_ident("test") {
        return true;
    }
    matches!(&attribute.meta, Meta::List(list) if list.path.is_ident("cfg") &&
        list.tokens.to_string() == "test")
}

fn mark_range(start: Span, end: Span, selected: &mut [bool]) {
    let first = start.start().line.saturating_sub(1);
    let last = end.end().line.saturating_sub(1);
    for line in selected.iter_mut().take(last + 1).skip(first) {
        *line = true;
    }
}
