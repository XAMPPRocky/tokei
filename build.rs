#[cfg(feature = "io")] extern crate serde_codegen;
extern crate serde;
extern crate serde_json;
extern crate handlebars;
#[macro_use] extern crate errln;

use serde_json::Value;
use handlebars::{Context, Handlebars};
use std::fs::File;
use std::env;
use std::path::{Path, PathBuf};
use std::ffi::OsString;

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("can't get OUT_DIR");
    expand(out_dir);
}

#[cfg(feature = "io")]
fn expand(out_dir: OsString) {
    use std::thread;
    let hbs = render_handlebars(&out_dir);

    let builder = thread::Builder::new()
        .name(String::from("Build Thread"))
        .stack_size(16388608);

    let handle = builder.spawn(move || {

        let paths = [
            (
                Path::new("src/language/serde_types.in.rs"),
                Path::new(&out_dir).join("language_serde_types.rs"),
            ),
            (
                &*hbs,
                hbs.to_owned(),
            ),
            (
                Path::new("src/serde_types.in.rs"),
                Path::new(&out_dir).join("stats_serde_types.in.rs"),
            ),
        ];


        for &(ref src, ref dst) in &paths {
            serde_codegen::expand(src, dst)
                .expect(&format!("Can't serde {:?}", src));
        }
    });

    let _ = handle.unwrap().join();
}

#[cfg(not(feature = "io"))]
fn expand(out_dir: OsString) {
    render_handlebars(&out_dir);
}

fn render_handlebars(out_dir: &OsString) -> PathBuf {

    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(handlebars::no_escape);

    let raw_data: Value = serde_json::from_reader(
        File::open(&"languages.json").expect("Can't open JSON")
    ).expect("Can't parse JSON");

    let data = Context::wraps(&raw_data);
    let out = Path::new(&out_dir).join("language_type.rs");
    let mut source_template = File::open(&"src/language/language_type.hbs.rs")
        .expect("Can't find Template");
    let mut output_file = File::create(&out).expect("Can't create output");

    if let Err(err) = handlebars.template_renderw2(&mut source_template,
                                                   &data,
                                                   &mut output_file)
    {
        panic!("Failed to generate languages! ERROR: {:?}", err);
    }
    out
}
