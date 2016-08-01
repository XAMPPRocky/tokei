#[cfg(feature = "io")]
extern crate serde_codegen;

fn main() {
    expand();
}

#[cfg(feature = "io")]
fn expand() {
    use std::env;
    use std::path::Path;

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let src = Path::new("src/lib/lib.rs.in");
    let dst = Path::new(&out_dir).join("lib.rs.in");

    serde_codegen::expand(&src, &dst).unwrap();
}

#[cfg(not(feature = "io"))]
fn expand() {}
