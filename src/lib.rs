#[macro_use]
extern crate maplit;
extern crate glob;
extern crate rayon;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_cbor;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;
extern crate walkdir;

include!(concat!(env!("OUT_DIR"), "/lib.rs"));
