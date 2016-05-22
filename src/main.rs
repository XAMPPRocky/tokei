#[macro_use]
extern crate clap;
extern crate glob;
#[macro_use]
extern crate maplit;
extern crate rayon;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_cbor;
extern crate serde_json;
extern crate serde_yaml;
extern crate walkdir;

include!(concat!(env!("OUT_DIR"), "/main.rs"));
