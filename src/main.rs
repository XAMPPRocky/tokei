extern crate getopts;

pub mod language;
pub mod fsutil;

use std::env;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use getopts::Options;
use language::Language;
use fsutil::{get_all_files, contains_comments};

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut opts = Options::new();

	opts.optflag("h", "help", "Print this help menu");
	opts.optopt("", "exclude-dir",
		"Example: --exclude-dir=docs",
		"\tDirectories wanted to be ignored");

	let matches = opts.parse(&args[1..]).unwrap();
	let mut ignored_directories: Vec<String> = Vec::new();
	ignored_directories.push(".git".to_string());

	if matches.opt_present("h") {
		let brief = format!("Usage: {} [options] [paths]", args[0].clone());
		println!("{}", opts.usage(&brief));
		return;
	}

	if matches.opt_present("exclude-dir") {
		let exclude_args = matches.opt_str("exclude-dir").unwrap();
		let exclude_vec = exclude_args.split(",");

		for excluded in exclude_vec {
			ignored_directories.push(excluded.to_string());
		}
	}

	if matches.free.is_empty() {
		println!("ERROR: ");
		println!("You must provide a file, or folder path as an argument.");
		return;
	}


	let row = "----------------------------------------------------------------------------------------------------";

	println!("{}", row);
	println!(" {:<15} {:>15} {:>15}  {:>15}  {:>15}  {:>15} ",
		"language", "files", "total", "blanks", "comments", "code");
	println!("{}", row);
	let mut languages: HashMap<&str, Language> = HashMap::new();
	languages.insert("cpp"  , Language::new("C++", "//","/*","*/"));
	languages.insert("hpp"  , Language::new("C++ Header", "//","/*","*/"));
	languages.insert("c"    , Language::new("C", "//","/*","*/"));
	languages.insert("h"    , Language::new("C Header", "//","/*","*/"));
	languages.insert("css"  , Language::new("CSS", "//","/*","*/"));
	languages.insert("java" , Language::new("Java", "//","/*","*/"));
	languages.insert("js"   , Language::new("JavaScript", "//","/*","*/"));
	languages.insert("rs"   , Language::new("Rust", "//","/*","*/"));
	languages.insert("xml"  , Language::new("XML", "<!--","<!--","-->"));
	languages.insert("html" , Language::new("HTML", "<!--","<!--","-->"));
	languages.insert("py"   , Language::new("Python", "#","'''","'''"));
	languages.insert("rb"   , Language::new("Ruby", "#","=begin","=end"));
	languages.insert("php"   , Language::new("PHP", "#,//","/*","*/"));

	for path in matches.free {
		let files = get_all_files(path, &ignored_directories);

		for file in files {
			let extension = match Path::new(&file).extension() {
				Some(result) => result.to_str().unwrap(),
				None => continue,
			};

			let mut language = match languages.get_mut(extension) {
				Some(result) => result,
				None => continue,
			};
			language.files.push(file.to_string());
		}
	}

	let mut total = Language::new("Total", "", "", "");

	for (_, language) in languages.iter_mut() {

		for file in language.files.iter() {
			let mut buffer: Vec<u8> = Vec::new();

			let mut file_ref = match File::open(&file) {
				Ok(result) => result,
				_ => continue,
			};

			let _ = file_ref.read_to_end(&mut buffer);

			let contents = match String::from_utf8(buffer) {
				Ok(result) => result,
				Err(_) => continue,
			};

			let mut is_in_comments = false;

			for line in contents.lines() {
				let line = line.trim();
				language.lines += 1;

				if line.starts_with(language.multi_line) {
					language.comments += 1;
					is_in_comments = true;
				} else if contains_comments(line, language.multi_line) {
					language.code += 1;
					is_in_comments = true;
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
					} else if line.trim().is_empty() {
						language.blanks += 1;
					} else {
						language.code += 1;
					}
				}
			}
		}

		if !language.is_empty() {
			println!("{}", language);
		}

		total.total    += language.files.len();
		total.lines    += language.lines;
		total.comments += language.comments;
		total.blanks   += language.blanks;
		total.code     += language.code;
	}

	println!("{}", row);
	println!("{}", total);
	println!("{}", row);
}