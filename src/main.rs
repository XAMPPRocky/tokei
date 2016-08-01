// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE, MIT} file.

#[macro_use]
extern crate clap;
// #[cfg(feature = "cbor")]
// extern crate serde_cbor;
#[cfg(feature = "json")]
extern crate serde_json;
#[cfg(feature = "yaml")]
extern crate serde_yaml;
#[cfg(feature = "toml-io")]
extern crate toml;
// #[cfg(feature = "cbor")]
// extern crate rustc_serialize;
extern crate tokei;

use std::borrow::Cow;
#[cfg(feature = "io")]
use std::collections::BTreeMap;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;

use clap::App;
// #[cfg(feature = "cbor")]
// use rustc_serialize::hex::FromHex;

use tokei::{Languages, Language, LanguageType};
use tokei::Sort::*;
const ROW: &'static str = "-------------------------------------------------------------------\
                                ------------";
const BLANKS: &'static str = "blanks";
const COMMENTS: &'static str = "comments";
const CODE: &'static str = "code";
const FILES: &'static str = "files";
const LINES: &'static str = "lines";
#[cfg(not(feature = "io"))]
const OUTPUT_ERROR: &'static str = "This version of tokei was compiled without any serialization
    formats, to enable serialization, reinstall tokei with the features flag.

        cargo install tokei --features all
";

fn main() {
    // Get options at the beginning, so the program doesn't have to make any extra calls to get the
    // information, and there isn't any magic strings.
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let files_option = matches.is_present(FILES);
    let input_option = matches.value_of("file_input");
    let output_option = matches.value_of("output");
    let language_option = matches.is_present("languages");
    let sort_option = matches.value_of("sort");
    let ignored_directories = {
        let mut ignored_directories: Vec<&str> = vec![".git"];
        if let Some(user_ignored) = matches.values_of("exclude") {
            for ignored in user_ignored {
                ignored_directories.push(ignored);
            }
        }
        ignored_directories
    };

    let mut languages = Languages::new();

    if language_option {
        for key in languages.keys() {
            println!("{:<25}", key);
        }
        return;
    }

    let paths: Vec<&str> = matches.values_of("input").unwrap().collect();

    if let Some(input) = input_option {
        add_input(input, &mut languages);
    }

    if output_option == None {
        println!("{}", ROW);
        println!(" {:<12} {:>12} {:>12} {:>12} {:>12} {:>12}",
                 "Language",
                 "Files",
                 "Lines",
                 "Code",
                 "Comments",
                 "Blanks");
        println!("{}", ROW);
    }

    let mut total = Language::new_blank();

    let print_animation = output_option == None;
    let (tx, rx) = channel();
    let child = thread::spawn(move || {
        loop {
            if let Ok(_) = rx.try_recv() {
                break;
            }

            if print_animation {
                print!(" Counting files.  \r");
                thread::sleep(Duration::from_millis(4));
                print!(" Counting files..\r");
                thread::sleep(Duration::from_millis(4));
                print!(" Counting files...\r");
                thread::sleep(Duration::from_millis(4));
            }
        }
    });

    languages.get_statistics(paths, ignored_directories);

    if output_option == None {
    }

    for (name, language) in &languages {
        if !language.is_empty() {
            if sort_option == None && output_option == None {
                if files_option {
                    print_language(&language, name);
                    println!("{}", ROW);

                    for stat in &language.stats {
                        println!("{}", stat);
                    }
                    println!("{}", ROW);
                } else if output_option == None {
                    print_language(&language, name);
                }
            }
        }
    }

    let _ = tx.send(());
    let _ = child.join();

    for (_, language) in &languages {
        if !language.is_empty() {
            total += language;
        }
    }

    if let Some(format) = output_option {
        match_output(&format, &languages);
    } else if let Some(sort_category) = sort_option {

        for (_, ref mut language) in &mut languages {
            match &*sort_category {
                BLANKS => language.sort_by(Blanks),
                COMMENTS => language.sort_by(Comments),
                CODE => language.sort_by(Code),
                FILES => language.sort_by(Files),
                LINES => language.sort_by(Lines),
                _ => unreachable!(),
            }
        }

        let mut languages: Vec<_> = languages.into_iter().collect();

        match &*sort_category {
            BLANKS => languages.sort_by(|a, b| b.1.blanks.cmp(&a.1.blanks)),
            COMMENTS => languages.sort_by(|a, b| b.1.comments.cmp(&a.1.comments)),
            CODE => languages.sort_by(|a, b| b.1.code.cmp(&a.1.code)),
            FILES => {}
            LINES => languages.sort_by(|a, b| b.1.files.len().cmp(&a.1.files.len())),
            _ => unreachable!(),
        }

        for (name, language) in languages {
            if !language.is_empty() {
                if !files_option {
                    print_language(&language, name);
                } else {
                    print_language(&language, name);
                    println!("{}", ROW);
                    for file in &language.stats {
                        println!("{}", file);
                    }
                    println!("{}", ROW);
                }
            }
        }
    }

    if output_option == None {
        if !files_option {
            println!("{}", ROW);
        }
        println!(" {: <18} {: >6} {:>12} {:>12} {:>12} {:>12}",
                 "Total",
                 total.stats.len(),
                 total.lines,
                 total.code,
                 total.comments,
                 total.blanks);
        println!("{}", ROW);
    }
}


#[cfg(feature = "all")]
fn add_input(input: &str, languages: &mut Languages) {
    use std::fs::File;
    use std::io::Read;

    let map = match File::open(input) {
        Ok(mut file) => {
            let contents = {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                contents
            };

            convert_input(contents)
        }
        Err(_) => {
            if input == "stdin" {
                let mut stdin = std::io::stdin();
                let mut buffer = String::new();

                let _ = stdin.read_to_string(&mut buffer);
                convert_input(buffer)
            } else {
                convert_input(String::from(input))
            }
        }
    };

    if let Some(map) = map {
        *languages += map;
    }

}

#[cfg(not(feature = "io"))]
#[allow(unused_variables)]
fn add_input(input: &str, map: &mut Languages) -> ! {
    panic!(OUTPUT_ERROR)
}


/// This originally  too a &[u8], but the u8 didn't directly correspond with the hexadecimal u8, so
/// it had to be changed to a String, and add the rustc_serialize dependency.
#[cfg(feature = "all")]
pub fn convert_input(contents: String) -> Option<BTreeMap<LanguageType, Language>> {
    if contents.is_empty() {
        None
    } else if let Ok(result) = serde_json::from_str(&*contents) {
        Some(result)
    } else if let Ok(result) = serde_yaml::from_str(&*contents) {
        Some(result)
    } else if let Some(result) = toml::decode_str(&*contents) {
        Some(result)
    } else {
        None
    }
}

#[cfg(feature = "all")]
fn match_output(format: &str, languages: &Languages) {
    match format {
        "cbor" => {
            // let cbor: Vec<u8> = languages.to_cbor().unwrap();

            // for byte in cbor {
            //    print!("{:02x}", byte);
            // }
        }
        "json" => print!("{}", languages.to_json().unwrap()),
        "toml" => print!("{}", languages.to_toml()),
        "yaml" => print!("{}", languages.to_yaml().unwrap()),
        _ => unreachable!(),
    }
}

#[cfg(not(feature = "io"))]
#[allow(unused_variables)]
fn match_output(format: &str, languages: &Languages) -> ! {
    panic!(OUTPUT_ERROR)
}


#[cfg(not(feature = "io"))]
#[allow(unused_variables)]
pub fn convert_input(contents: String) -> ! {
    panic!(OUTPUT_ERROR);
}

fn print_language<'a, C>(language: &'a Language, name: C)
    where C: Into<Cow<'a, LanguageType>>
{
    println!(" {: <18} {: >6} {:>12} {:>12} {:>12} {:>12}",
             name.into().name(),
             language.stats.len(),
             language.lines,
             language.code,
             language.comments,
             language.blanks)
}
