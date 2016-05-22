use std::fmt;

#[derive(Default, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Stats {
    pub name: String,
    pub code: usize,
    pub blanks: usize,
    pub lines: usize,
    pub comments: usize,
}


impl Stats {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Stats { name: name.into(), ..Self::default() }
    }
}


impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name_length = self.name.len();

        let name = if name_length == 25 {
            self.name.clone()
        } else if self.name.len() > 24 {
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
               self.code,
               self.comments,
               self.blanks)
    }
}
