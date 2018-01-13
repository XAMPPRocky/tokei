extern crate handlebars;
extern crate ignore;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;

use handlebars::Handlebars;
use serde_json::Value;

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("can't get OUT_DIR");
    generate_languages(&out_dir);
    generate_tests(&out_dir);
}

fn generate_languages(out_dir: &OsStr) {
    let handlebars = {
        let mut h = Handlebars::new();
        h.register_escape_fn(handlebars::no_escape);
        h
    };

    let json: Value =  {
        let json = File::open(&"languages.json").expect("Cant open json");
        serde_json::from_reader(json).expect("Can't parse json")
    };

    let output = Path::new(&out_dir).join("language_type.rs");
    let mut source_template = File::open(&"src/language/language_type.hbs.rs")
        .expect("Can't find Template");
    let mut output_file = File::create(&output).expect("Can't create output");

    if let Err(err) = handlebars.template_renderw2(&mut source_template,
                                                   &json,
                                                   &mut output_file)
    {
        panic!("Failed to generate languages! ERROR: {:?}", err);
    }
}

fn generate_tests(out_dir: &OsStr) {
    use std::io::Write;

    use ignore::Walk;

    let mut string = String::new();

    let walker = Walk::new("./tests/data/").filter(|p| {
        match p {
            &Ok(ref p) => {
                if let Ok(ref p) = p.metadata() {
                    p.is_file()
                } else {
                    false
                }
            },
            _ => false,
        }
    });

    for path in walker {
        let path = path.unwrap();
        let path = path.path();

        let name = path.file_stem().unwrap().to_str().unwrap().to_lowercase();

        string.push_str(&format!(r#"
        #[test]
        fn {0}() {{
            let mut languages = Languages::new();
            languages.get_statistics(&["{1}"], Vec::new());

            if languages.len() != 1 {{
                panic!("wrong languages detected: expected just {0}, found {{:?}}",
                       languages.into_iter().collect::<Vec<_>>());
            }}

            let (name, language) = languages.into_iter().next().unwrap();

            let mut contents = String::new();
            File::open("{1}").unwrap().read_to_string(&mut contents).unwrap();

            assert_eq!(get_digit!(LINES, contents), language.lines);
            println!("{{}} LINES MATCH", name);
            assert_eq!(get_digit!(CODE, contents), language.code);
            println!("{{}} CODE MATCH", name);
            assert_eq!(get_digit!(COMMENTS, contents), language.comments);
            println!("{{}} COMMENTS MATCH", name);
            assert_eq!(get_digit!(BLANKS, contents), language.blanks);
            println!("{{}} BLANKS MATCH", name);
        }}
        "#, name, path.display()));
    }

    File::create(Path::new(&out_dir).join("tests.rs")).unwrap()
        .write_all(string.as_bytes()).unwrap();
}
