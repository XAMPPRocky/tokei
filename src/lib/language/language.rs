use std::path::PathBuf;
use std::ops::AddAssign;

use utils::*;
use stats::Stats;

/// Struct representing a single Language.
#[derive(Clone, Debug,  Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Language {
    /// Number of blank lines.
    pub blanks: usize,
    /// Number of lines of code.
    pub code: usize,
    /// Number of comments(both single, and multi-line)
    pub comments: usize,
    /// A collection of files to be analysed.
    #[serde(skip_deserializing, skip_serializing)]
    pub files: Vec<PathBuf>,
    /// A collection of statistics based on the files provide from `files`
    pub stats: Vec<Stats>,
    /// Number of total lines.
    pub lines: usize,
    /// A collection of single line comments in the language. ie. `//` in Rust.
    #[serde(skip_deserializing, skip_serializing)]
    pub line_comment: Vec<&'static str>,
    /// A collection of tuples representing the start and end of multi line comments. ie. `/* comment */` in Rust.
    #[serde(skip_deserializing, skip_serializing)]
    pub multi_line: Vec<(&'static str, &'static str)>,
    /// Whether the language supports nested multi line comments or not.
    #[serde(skip_deserializing, skip_serializing)]
    pub nested: bool,
    /// The total number of files from `stats`.
    pub total_files: usize,
}

impl Language {
    /// Constructs a new  empty Language with the comments provided.
    ///
    /// ```
    /// let mut rust = Language::new(vec!["//"], vec![("/*", "*/")]);
    /// ```
    pub fn new(line_comment: Vec<&'static str>,
               multi_line: Vec<(&'static str, &'static str)>)
               -> Self {

        Language {
            line_comment: line_comment,
            multi_line: multi_line,
            ..Self::default()
        }
    }

    /// Convience constructor for creating a language that has no commenting syntax.
    ///
    /// ```
    /// let json = Language::new_blank();
    ///
    /// assert_eq!(json.line_comment, vec![]);
    /// ```
    pub fn new_blank() -> Self {
        Self::default()
    }

    /// Convience constructor for creating a language that has the same commenting syntax as C like languages.
    ///
    /// ```
    /// let rust = Language::new(vec!["//"], vec![("/*", "*/")]);
    /// let c = Language::new_c();
    ///
    /// assert_eq!(rust.line_comment, c.line_comment);
    /// assert_eq!(rust.multi_line, c.multi_line);
    /// ```
    pub fn new_c() -> Self {
        Language {
            line_comment: vec!["//"],
            multi_line: vec![("/*", "*/")],
            ..Self::default()
        }
    }

    /// Convience constructor for creating a language that has the same commenting syntax as ML like languages.
    ///
    /// ```
    /// let ocaml = Language::new_multi(vec![("(*", "*)")]);
    /// let coq = Language::new_func();
    ///
    /// assert_eq!(ocaml.line_comment, coq.line_comment);
    /// assert_eq!(ocaml.multi_line, coq.multi_line);
    /// ```
    pub fn new_func() -> Self {
        Language { multi_line: vec![("(*", "*)")], ..Self::default() }
    }

    /// Convience constructor for creating a language that has the same commenting syntax as HTML like languages.
    ///
    /// ```
    /// let xml = Language::new_multi(vec![("<!--", "-->")]);
    /// let html = Language::new_html();
    ///
    /// assert_eq!(xml.line_comment, html.line_comment);
    /// assert_eq!(xml.multi_line, html.multi_line);
    /// ```
    pub fn new_html() -> Self {
        Language { multi_line: vec![("<!--", "-->")], ..Self::default() }
    }

    /// Convience constructor for creating a language that has the same commenting syntax as Bash.
    ///
    /// ```
    /// let bash = Language::new_single(vec!["#"]);
    /// let yaml = Language::new_hash();
    ///
    /// assert_eq!(bash.line_comment, yaml.line_comment);
    /// assert_eq!(bash.multi_line, yaml.multi_line);
    /// ```
    pub fn new_hash() -> Self {
        Self::new_single(vec!["#"])
    }

    /// Convience constructor for creating a language that only has multi line comments.
    ///
    /// ```
    /// let mustache = Language::new_multi(vec![("{{!", "}}")]);
    /// ```
    pub fn new_multi(multi_line: Vec<(&'static str, &'static str)>) -> Self {
        Language { multi_line: multi_line, ..Self::default() }
    }

    /// Convience constructor for creating a language that has the same commenting syntax as Prolog.
    ///
    /// ```
    /// let prolog = Language::new(vec!["%"], vec![("/*", "*/")]);
    /// let oz = Language::new_pro();
    ///
    /// assert_eq!(prolog.line_comment, oz.line_comment);
    /// assert_eq!(prolog.multi_line, oz.multi_line);
    /// ```
    pub fn new_pro() -> Self {
        Language {
            line_comment: vec!["%"],
            multi_line: vec![("/*", "*/")],
            ..Self::default()
        }
    }

    /// Convience constructor for creating a language that only has single line comments.
    ///
    /// ```
    /// let haskell = Language::new_single(vec!["--"]);
    /// ```
    pub fn new_single(line_comment: Vec<&'static str>) -> Self {
        Language { line_comment: line_comment, ..Self::default() }
    }

    /// Checks if the language is empty. Empty meaning it doesn't have any statistics.
    ///
    /// ```
    /// let rust = Language::new_c();
    ///
    /// assert!(rust.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.code == 0 && self.comments == 0 && self.blanks == 0 && self.lines == 0
    }

    /// Checks if the language doesn't contain any comments.
    ///
    /// ```
    /// let json = Language::new_blank();
    ///
    /// assert!(json.is_blank());
    /// ```
    pub fn is_blank(&self) -> bool {
        self.line_comment.is_empty() && self.multi_line.is_empty()
    }

    /// Specify if the the language supports nested multi line comments.
    ///
    /// ```
    /// let mut rust = Language::new(vec!["//"], vec![("/*", "*/")]).nested();
    /// assert!(rust.nested);
    /// ```
    pub fn nested(mut self) -> Self {
        self.nested = true;
        self
    }

    /// Sorts each of the `Stats` structs contained in the language based on what category is provided
    /// panic!'s if given the wrong category.
    ///
    /// ```
    /// let mut rust = Language::new_c();
    /// let foo_stats = Stats::new();
    /// let bar_stats = Stats::new();
    ///
    /// foo_stats.code += 20;
    /// bar_stats.code += 10;
    ///
    /// rust.stats.push(bar_stats.clone());
    /// rust.stats.push(foo_stats.clone());
    ///
    /// assert_eq!(rust.stats, vec![bar_stats.clone(), foo_stats.clone()]);
    ///
    /// rust.sort_by(CODE);
    ///
    /// assert_eq!(rust.stats, vec![foo_stats, bar_stats]);
    ///
    /// ```
    pub fn sort_by(&mut self, category: &str) {
        match category {
            BLANKS => self.stats.sort_by(|a, b| b.blanks.cmp(&a.blanks)),
            COMMENTS => self.stats.sort_by(|a, b| b.comments.cmp(&a.comments)),
            CODE => self.stats.sort_by(|a, b| b.code.cmp(&a.code)),
            FILES => self.stats.sort_by(|a, b| a.name.cmp(&b.name)),
            LINES => self.stats.sort_by(|a, b| b.lines.cmp(&a.lines)),
            _ => unreachable!(),
        }
    }
}

impl AddAssign for Language {
    fn add_assign(&mut self, rhs: Self) {
        self.total_files += rhs.total_files;
        self.lines += rhs.lines;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
        self.code += rhs.code;
        self.stats.extend_from_slice(&*rhs.stats);
    }
}

impl<'a> AddAssign<&'a Language> for Language {
    fn add_assign(&mut self, rhs: &'a Self) {
        self.total_files += rhs.total_files;
        self.lines += rhs.lines;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
        self.code += rhs.code;
        self.stats.extend_from_slice(&*rhs.stats);
    }
}

impl<'a> AddAssign<&'a mut Language> for Language {
    fn add_assign(&mut self, rhs: &mut Self) {
        self.total_files += rhs.total_files;
        self.lines += rhs.lines;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
        self.code += rhs.code;
        self.stats.extend_from_slice(&*rhs.stats);
    }
}

impl AddAssign<Stats> for Language {
    fn add_assign(&mut self, rhs: Stats) {
        self.lines += rhs.lines;
        self.code += rhs.code;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
        self.stats.push(rhs);
    }
}
