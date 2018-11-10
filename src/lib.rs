 #![deny(trivial_casts,
         trivial_numeric_casts,
         unused_variables,
         unstable_features,
         unused_import_braces)]

//! # Tokei: Code Analysis Library
//!
//! A simple, effcient library for counting code in directories.
//! [_For the binary_](https://github.com/Aaronepower/tokei/)
//!
//! ## How to use
//!
//! Tokei provides both `Languages` struct which a map of many existing programming languages,
//! and `Language` for creating custom languages.
//!
//! ### Example
//!
//! Gets the total lines of code from all rust files in current directory, and all subdirectories.
//!
//! ```no_run
//! extern crate tokei;
//!
//! use std::collections::BTreeMap;
//! use std::fs::File;
//! use std::io::Read;
//!
//! use tokei::{Languages, LanguageType};
//!
//! fn main() {
//!     // The paths to search. Accepts absolute, relative, and glob paths.
//!     let paths = &["**/*.rs"];
//!     // Exclude any path that contains any of these strings.
//!     let excluded = vec!["target", ".git"];
//!
//!     // Create new Languages
//!     let mut languages = Languages::new();
//!
//!     // Get statistics
//!     languages.get_statistics(paths, excluded, None);
//!
//!     // Remove empty languages
//!     let language_map = languages.remove_empty();
//!
//!     // Get Rust from statistics
//!     let rust = language_map.get(&LanguageType::Rust).unwrap();
//!
//!     // Print the number of lines that were code.
//!     println!("Lines of code: {}", rust.code);
//! }
//! ```

#[macro_use]
extern crate log;
extern crate encoding_rs;
extern crate ignore;
extern crate rayon;

#[cfg(feature = "io")]
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "io")]
extern crate serde;

#[macro_use]
mod utils;
mod language;
mod stats;
mod sort;

pub use language::{LanguageType, Languages, Language};
pub use stats::Stats;
pub use sort::Sort;
