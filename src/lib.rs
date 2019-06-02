//! # Tokei: Easily count code.
//!
//! A simple, efficient library for counting code in directories. This
//! functionality is also provided as a
//! [CLI utility](//github.com/XAMPPRocky/tokei). Tokei uses a small state
//! machine rather than regular expressions found in other code counters. Tokei
//! can accurately count a lot more edge cases such as nested comments, or
//! comment syntax inside string literals.
//!
//! # Examples
//!
//! Gets the total lines of code from all rust files in current directory,
//! and all subdirectories.
//!
//! ```no_run
//! extern crate tokei;
//!
//! use std::collections::BTreeMap;
//! use std::fs::File;
//! use std::io::Read;
//!
//! use tokei::{Config, Languages, LanguageType};
//!
//! fn main() {
//!     // The paths to search. Accepts absolute, relative, and glob paths.
//!     let paths = &["src", "tests"];
//!     // Exclude any path that contains any of these strings.
//!     let excluded = &["target"];
//!     // `Config` allows you to configure what is searched and counted.
//!     let config = Config::default();
//!
//!     let mut languages = Languages::new();
//!     languages.get_statistics(paths, excluded, &config);
//!     let rust = &languages[&LanguageType::Rust];
//!
//!     println!("Lines of code: {}", rust.code);
//! }
//! ```

 #![deny(trivial_casts,
         trivial_numeric_casts,
         unused_variables,
         unstable_features,
         unused_import_braces,
         missing_docs)]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

#[macro_use] mod utils;
mod config;
mod language;
mod sort;
mod stats;

pub use self::{
    config::Config,
    language::{LanguageType, Languages, Language},
    sort::Sort,
    stats::Stats,
};
