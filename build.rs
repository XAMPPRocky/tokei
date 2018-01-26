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

    let mut json: Value = {
        let json = File::open(&"languages.json").expect("Cant open json");
        serde_json::from_reader(json).expect("Can't parse json")
    };

    for (key, ref mut item) in json.get_mut("languages")
                                   .unwrap()
                                   .as_object_mut()
                                   .unwrap()
                                   .iter_mut()
    {
        macro_rules! sort_prop {
            ($prop:expr) => {{
                if let Some(ref mut prop) = item.get_mut($prop) {
                    prop.as_array_mut()
                        .unwrap()
                        .sort_unstable_by(compare_json_str_len)
                }
            }}
        }

        sort_prop!("quotes");
        sort_prop!("multi_line");
    }

    let output = Path::new(&out_dir).join("language_type.rs");
    let mut source_template = File::open(&"src/language/language_type.hbs.rs")
        .expect("Can't find Template");
    let mut output_file = File::create(&output).expect("Can't create output");

    if let Err(err) = handlebars.render_template_source_to_write(&mut source_template,
                                                                 &json,
                                                                 &mut output_file)
    {
        panic!("Failed to generate languages! ERROR: {:?}", err);
    }
}

fn compare_json_str_len(a: &Value, b: &Value) -> ::std::cmp::Ordering {
    let a = a.as_array().expect("a as array");
    let b = b.as_array().expect("b as array");

    let max_a_size = a.iter().map(|e| e.as_str().unwrap().len()).max().unwrap();
    let max_b_size = b.iter().map(|e| e.as_str().unwrap().len()).max().unwrap();

    max_b_size.cmp(&max_a_size)
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
