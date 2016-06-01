// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT/APACHE2.0 license that can be
// found in the LICENCE-{APACHE, MIT} file.

#[macro_use]
extern crate clap;
extern crate tokei;

use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;

use clap::App;

use tokei::language::{Language, Languages};

pub const ROW: &'static str = "-------------------------------------------------------------------------------";
pub const FILES: &'static str = "files";

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
    let paths = matches.values_of("input").unwrap();
    let ignored_directories = {
        let mut ignored_directories = vec![".git"];
        if let Some(user_ignored) = matches.values_of("exclude") {
            for ignored in user_ignored {
                ignored_directories.push(ignored);
            }
        }
        ignored_directories
    };

    let languages = Languages::new();

    if language_option {
        for key in languages.keys() {
            println!("{:<25}", key);
        }
        return;
    }

    if let Some(input) = input_option {
        match File::open(input) {
            Ok(mut file) => {
                let contents = {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    contents
                };

                language.add_previous(contents)
            }
            Err(_) => {
                if input == "stdin" {
                    let mut stdin = std::io::stdin();
                    let mut buffer = String::new();

                    let _ = stdin.read_to_string(&mut buffer);
                    language.add_previous(buffer)
                } else {
                    language.add_previous(String::from(input))
                }
            }
        };
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

    let mut total = LanguageStatistics::new_blank();

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
        print!("{}", CLEAR);
    }

    for language in &languages {
        if !language.is_empty() {
            if sort_option == None && output_option == None {
                if files_option {
                    language.print(name);
                    println!("{}", ROW);

                    for stat in &language.stats {
                        println!("{}", stat);
                    }
                    println!("{}", ROW);
                } else if output_option == None {
                    language.print(name);
                }
            }
        }
    }

    let _ = tx.send(());
    let _ = child.join();

    for &(_, ref language) in &languages {
        if !language.is_empty() {
            total += language;
        }
    }

    if let Some(format) = output_option {
        match &*format {
            "cbor" => {
                let cbor: Vec<_> = languages.to_cbor().unwrap();

                for byte in cbor {
                    print!("{:02x}", byte);
                }
            }
            "json" => print!("{}", languages.to_json().unwrap()),
            // "toml" => print!("{}", {
            //     let encoder = toml::Encoder::new();
            //     lang_map.encode(&mut encoder).unwrap();
            //     encoder.toml
            // }),
            "yaml" => print!("{}", languages.to_yaml().unwrap()),
            _ => unreachable!(),
        }
    } else if let Some(sort_category) = sort_option {

        for &mut (_, ref mut language) in &mut languages {
            match &*sort_category {
                BLANKS => language.sort_by(BLANKS),
                COMMENTS => language.sort_by(COMMENTS),
                CODE => language.sort_by(CODE),
                FILES => {}
                TOTAL => language.sort_by(TOTAL),
                _ => unreachable!(),
            }
        }

        let languages: Vec<_> = languages.into_iter().collect();

        match &*sort_category {
            BLANKS => languages.sort_by(|a, b| b.1.blanks.cmp(&a.1.blanks)),
            COMMENTS => languages.sort_by(|a, b| b.1.comments.cmp(&a.1.comments)),
            CODE => languages.sort_by(|a, b| b.1.code.cmp(&a.1.code)),
            FILES => languages.sort_by(|a, b| b.1.files.len().cmp(&a.1.files.len())),
            TOTAL => languages.sort_by(|a, b| b.1.lines.cmp(&a.1.lines)),
            _ => unreachable!(),
        }

        for (name, language) in languages {
            if !language.is_empty() {
                if !files_option {
                    language.print(name);
                } else {
                    language.print(name);
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
        total.print(__Total);
        println!("{}", ROW);
    }
}
