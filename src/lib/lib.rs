#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces)]

//! # Tokei: Code Analysis Library(For the [binary](https://github.com/Aaronepower/tokei/))
//!
//! Tokei is the library powering the application of the same name.

#[macro_use]
extern crate maplit;
#[macro_use]
extern crate serializable_enum;
extern crate glob;
extern crate rayon;
extern crate serde;
extern crate serde_cbor;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;
extern crate walkdir;

include!(concat!(env!("OUT_DIR"), "/lib.rs"));
