#[macro_use]
extern crate clap;

#[macro_use]
pub mod macros;
pub mod language;
pub mod fsutil;

use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::collections::BTreeMap;

use clap::App;

use language::Language;
use fsutil::{get_all_files, contains_comments};

fn main() {
    let yaml = load_yaml!("../cli.yml");
	let matches = App::from_yaml(yaml).get_matches();

	let mut languages: BTreeMap<&str, Language> = BTreeMap::new();
	languages.insert("as"     , Language::new_c("ActionScript"));
	languages.insert("c"      , Language::new_c("C"));
	languages.insert("cs"     , Language::new_c("C#"));
	languages.insert("clj"    , Language::new_single("Clojure", ";,#,#_"));
	languages.insert("coffee" , Language::new("CoffeeScript", "#", "###", "###"));
	languages.insert("cfm"    , Language::new("ColdFusion", "<!---", "<!---", "--->"));
	languages.insert("cfc"    , Language::new_c("ColdFusion CFScript"));
	languages.insert("cpp"    , Language::new_c("C++"));
	languages.insert("css"    , Language::new_c("CSS"));
	languages.insert("d"      , Language::new_c("D"));
	languages.insert("dart"   , Language::new_c("Dart"));
	languages.insert("go"     , Language::new_c("Go"));
	languages.insert("h"      , Language::new_c("C Header"));
	languages.insert("hs"     , Language::new_single("Haskell", "--"));
	languages.insert("hpp"    , Language::new_c("C++ Header"));
	languages.insert("html"   , Language::new_html("HTML"));
	languages.insert("java"   , Language::new_c("Java"));
	languages.insert("js"     , Language::new_c("JavaScript"));
    languages.insert("jl"     , Language::new("Julia", "#", "#=", "=#"));
	languages.insert("json"   , Language::new_blank("JSON"));
	languages.insert("jsx"    , Language::new_c("JSX"));
	languages.insert("el"     , Language::new("LISP", ";", "#|", "|#"));
	languages.insert("m"      , Language::new_c("Objective-C"));
	languages.insert("mm"     , Language::new_c("Objective-C++"));
	languages.insert("php"    , Language::new("PHP", "#,//","/*","*/"));
	languages.insert("pas"    , Language::new("Pascal", "//,(*","{","}"));
	languages.insert("pl"     , Language::new("Perl", "#","=","=cut"));
	languages.insert("py"     , Language::new("Python", "#","'''","'''"));
	languages.insert("rs"     , Language::new("Rust", "//,///,//!", "/*", "*/"));
	languages.insert("r"      , Language::new("R", "#","",""));
	languages.insert("rb"     , Language::new("Ruby", "#","=begin","=end"));
	languages.insert("rhtml"  , Language::new_html("Ruby HTML"));
	languages.insert("sass"   , Language::new_c("Sass"));
	languages.insert("sh"     , Language::new_single("BASH", "#"));
	languages.insert("sql"    , Language::new("SQL", "--", "/*", "*/"));
	languages.insert("swift"  , Language::new_c("Swift"));
	languages.insert("ts"     , Language::new_c("TypeScript"));
	languages.insert("xml"    , Language::new_html("XML"));
	languages.insert("yml"    , Language::new_single("YAML", "#"));

    if matches.is_present("languages") {
        for (ext, language) in languages {
        println!("{:<25} ({})", language.name, ext);
        }
        return;
    }


    let paths = matches.values_of("input").unwrap();

    let mut ignored_directories: Vec<String> = Vec::new();

	if let Some(user_ignored) = matches.values_of("exclude") {
		for ignored in user_ignored {
            ignored_directories.push(ignored.to_owned());
        }
	}

    let mut sort = String::new();
    if let Some(sort_by) = matches.value_of("sort") {
        match &*sort_by.to_lowercase() {
            "files" | "total" | "blanks" | "comments" | "code" => sort.push_str(&*sort_by.to_lowercase()),
            _ => println!("--sort must be any of the following files, total, blanks, comments, code"),
        }
    }
    let sort_empty = sort.is_empty();

	let row = "--------------------------------------------------------------------------------------------------";

	println!("{}", row);
	println!(" {:<15} {:>15} {:>15} {:>15} {:>15} {:>15}",
		"Language", "Files", "Total", "Blanks", "Comments", "Code");
	println!("{}", row);
		for path in paths {
		let files = get_all_files(path.to_owned(), &ignored_directories);

		for file in files {
			let extension = unwrap_opt_cont!(unwrap_opt_cont!(Path::new(&file).extension()).to_str());

			let lowercase: &str = &extension.to_lowercase();

			let mut language = unwrap_opt_cont!(languages.get_mut(lowercase));
			language.files.push(file.to_owned());
		}
	}

	let mut total = Language::new_blank("Total");

	for (_, language) in &mut languages {

		for file in language.files.iter() {

			let mut file_ref = unwrap_rs_cont!(File::open(&file));
			let mut contents = String::new();

			let _ = unwrap_rs_cont!(file_ref.read_to_string(&mut contents));

			let mut is_in_comments = false;

			'line: for line in contents.lines() {
				let line = line.trim();
				language.lines += 1;

                if line.trim().is_empty() {
                    language.blanks += 1;
                    continue;
                }
                if !language.multi_line.is_empty() {
                    if line.starts_with(language.multi_line) {
                        is_in_comments = true;
                    } else if contains_comments(line, language.multi_line) {
                        language.code += 1;
                        is_in_comments = true;
                    }
                }

				if is_in_comments {
					if line.contains(language.multi_line_end) {
						is_in_comments = false;
					}
					language.comments += 1;
					continue;
				}
				let single_comments = language.line_comment.split(",");
				for single in single_comments {
					if line.starts_with(single) {
						language.comments += 1;
                        continue 'line;
					}
                }
                language.code += 1;
			}
		}


		if !language.is_empty() && sort_empty {
			println!("{}", language);
		}

		total.total    += language.files.len();
		total.lines    += language.lines;
		total.comments += language.comments;
		total.blanks   += language.blanks;
		total.code     += language.code;
	}

    if !sort_empty {
        let mut unsorted_vec:Vec<(&&str, &Language)> = languages.iter().collect();
        match &*sort {
            "files" => {
                unsorted_vec.sort_by(|a, b| b.1.files.len().cmp(&a.1.files.len()))
            },
            "total" => {
                unsorted_vec.sort_by(|a, b| b.1.lines.cmp(&a.1.lines))
            },
            "blanks" => {
                unsorted_vec.sort_by(|a, b| b.1.blanks.cmp(&a.1.blanks))
            },
            "comments" => {
                unsorted_vec.sort_by(|a, b| b.1.comments.cmp(&a.1.comments))
            },
            "code" => {
                unsorted_vec.sort_by(|a, b| b.1.code.cmp(&a.1.code))
            },
            _ => unreachable!(),
        };

        for (_, language) in unsorted_vec {
            if !language.is_empty() {
                println!("{}", language);
            }
        }
    }

	println!("{}", row);
	println!("{}", total);
	println!("{}", row);
}
