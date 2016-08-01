 #![deny(missing_debug_implementations, missing_copy_implementations,
            trivial_casts, trivial_numeric_casts,
            unsafe_code,
            unstable_features,
            unused_import_braces)]

//! # Tokei: Code Analysis Library
//!
//! A simple, effcient library for analysing code in directories.[_For the binary_](https://github.com/Aaronepower/tokei/)
//!
//! ## How to use
//!
//! Tokei provides both `Languages` a map of existing programming languages and `Language` for creating custom languages.
//!
//! ### Example(Get total lines of code from all rust files in current directory, and all subdirectories)
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
//!     let paths = vec!["**/*.rs"];
//!     // Exclude any path that contains any of these strings.
//!     let excluded = vec!["target", ".git"];
//!
//!     // Create new Languages
//!     let mut languages = Languages::new();
//!
//!     // Get statistics
//!     languages.get_statistics(&*paths, &*excluded);
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
extern crate maplit;
extern crate glob;
extern crate rayon;
#[cfg(feature = "io")]
extern crate serde;
// #[cfg(feature = "cbor")]
// extern crate serde_cbor;
#[cfg(feature = "json")]
extern crate serde_json;
#[cfg(feature = "yaml")]
extern crate serde_yaml;
#[cfg(feature = "toml-io")]
extern crate toml;
extern crate walkdir;

#[cfg(feature = "io")]
include!(concat!(env!("OUT_DIR"), "/lib.rs.in"));

#[cfg(not(feature = "io"))]
include!("lib.rs.in");
