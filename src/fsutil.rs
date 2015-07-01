extern crate glob;

use std::fs;
use std::fs::metadata;
use self::glob::glob;

pub fn contains_comments(file: &str, comment: &str) -> bool {
	let vector = file.splitn(3, "\"").filter_map( |element| {
		if !(element == "") {
			Some(element)
		} else {
			None
		}
		
	}).collect::<Vec<&str>>();

	let length = vector.len();

	if length == 0 || length == 1 {
		return false
	}

	if length == 2 {
		for element in &vector {
			if element.contains(comment) {
				return true
			}
		}
		return false
	}

	if vector[0].contains(comment) {

		return true
	}

	if vector[2].contains("\"") {

		return contains_comments(vector[2], comment)
	} else if vector[2].contains(comment) {

		return true
	}
	false        
}

pub fn get_all_files(path: String, ignored_directories: &Vec<String>) -> Vec<String> {
	let mut files: Vec<String> = Vec::new();

	if let Ok(result) = metadata(&path) {
		if result.is_dir() {
			let dir = fs::read_dir(&path).unwrap();
			'file: for entry in dir {
				let entry = entry.unwrap();
				let file_path = entry.path();
				let file_str = file_path.to_str().unwrap();
				let file_string = file_str.to_owned();
				let path_metadata = metadata(&file_string).unwrap();

				if path_metadata.is_dir() {
					for ignored_directory in ignored_directories {
						if file_str.contains(ignored_directory) {
							continue 'file;
						}
					}
					for file in get_all_files(file_string, &ignored_directories) {
						files.push(file);
					}
				} else if path_metadata.is_file() {
					files.push(file_string);
				}
			}
		} else {
			files.push(path);
		}
	} else {
		for path_buf in glob(&path).unwrap() {
			let file_path = path_buf.unwrap().as_path().to_str().unwrap().to_owned();
			files.push(file_path);
		}
	}

	files
}