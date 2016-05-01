use std::fmt;

#[derive(Debug)]
pub struct Stats {
    pub name: String,
    pub code: usize,
    pub blanks: usize,
    pub lines: usize,
    pub comments: usize,
}


impl Stats {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Stats {
            name: name.into(),
            code: 0,
            blanks: 0,
            lines: 0,
            comments: 0,
        }
    }
}


impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = if self.name.len() > 24 {
            let mut name = String::from("|");
            name.push_str(&self.name[self.name.len() - 24..]);
            name
        } else {
            self.name.clone()
        };
        write!(f,
               " {: <25} {:>12} {:>12} {:>12} {:>12}",
               name,
               self.lines,
               self.blanks,
               self.comments,
               self.code)
    }
}
