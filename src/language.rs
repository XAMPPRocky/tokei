use std::fmt;

pub struct Language<'a> {
	pub name: &'a str,
	pub line_comment: &'a str,
	pub multi_line: &'a str,
	pub multi_line_end: &'a str,
	pub files: Vec<String>,
	pub code: u32,
	pub comments: u32,
	pub blanks: u32,
	pub lines: u32,
	pub total: usize,
}

impl<'a> Language<'a> {
	pub fn new<'b>(name: &'a str,
		line_comment: &'a str,
		multi_line: &'a str,
		multi_line_end: &'a str) -> Language<'a> {

		Language {
			name: name,
			line_comment: line_comment,
			multi_line: multi_line,
			multi_line_end: multi_line_end,
			files: Vec::new(),
			code: 0,
			comments: 0,
			blanks: 0,
			lines: 0,
			total: 0,
		}
	}

	pub fn is_empty(&self) -> bool {
		self.code == 0 && self.comments == 0 && self.blanks == 0 && self.lines == 0
	}
}

impl<'a> fmt::Display for Language<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut total;

		if self.total == 0 {
			total = self.files.len()
		} else {
			total = self.total;
		}
		write!(f," {: <15} {: >15}  {:>15}  {:>15}  {:>15}  {:>15} ", self.name, total, self.lines, self.blanks, self.comments, self.code)
	}
}