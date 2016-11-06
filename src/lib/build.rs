#[cfg(feature = "io")]
extern crate serde_codegen;
extern crate serde;
extern crate serde_json;
extern crate handlebars;

use serde_json::Value;
use handlebars::{Context, Handlebars};
use std::fs::File;
use std::env;
use std::path::Path;

fn main() {
    expand();
}

#[cfg(feature = "io")]
fn expand() {
    use std::thread;
    render_handlebars();

    let builder = thread::Builder::new()
        .name(String::from("Build Thread"))
        .stack_size(8388608);

    let handle = builder.spawn(|| {
        let out_dir = env::var_os("OUT_DIR").unwrap();

        let src = Path::new("src/lib/lib.rs.in");
        let dst = Path::new(&out_dir).join("lib.rs.in");

        serde_codegen::expand(&src, &dst).unwrap();
    });

    let _ = handle.unwrap().join();
}

#[cfg(not(feature = "io"))]
fn expand() {
    render_handlebars();
}

fn render_handlebars() {
    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(handlebars::no_escape);
    let raw_data: Value = serde_json::from_reader(
        File::open(&"src/lib/languages.json").unwrap()).unwrap();
    let data = Context::wraps(&raw_data);
    let out = Path::new(&env::var_os("OUT_DIR").unwrap()).join("language_type.rs");
    let mut source_template = File::open(&"src/lib/language/language_type.rs.hbs")
        .expect("Can't find Template");
    let mut output_file = File::create(&out).expect("Can't create!");
    if let Err(err) = handlebars.template_renderw2(&mut source_template,
                                                   &data,
                                                   &mut output_file)
    {
        panic!("Failed to generate languages! ERROR: {:?}", err);
    }
}
