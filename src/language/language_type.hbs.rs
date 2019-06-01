/// Represents a individual programming language. Can be used to provide
/// information about the language, such as multi line comments, single line
/// comments, string literal syntax, whether a given language allows nesting
/// comments.
#[derive(Deserialize, Serialize)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LanguageType {
    {{~#each languages}}
        #[allow(missing_docs)]
        {{~@key}},
    {{/each}}
}

impl LanguageType {

    pub(crate) fn blank_allows_nested() -> bool {
        false
    }

    pub(crate) fn blank_line_comments() -> &'static [&'static str] {
        &[]
    }

    pub(crate) fn blank_multi_line_comments()
        -> &'static [(&'static str, &'static str)]
    {
        &[]
    }

    pub(crate) fn blank_quotes() -> &'static [(&'static str, &'static str)] {
        &[]
    }

    pub(crate) fn c_allows_nested() -> bool {
        Self::blank_allows_nested()
    }

    pub(crate) fn c_line_comments() -> &'static [&'static str] {
        &["//"]
    }

    pub(crate) fn c_multi_line_comments()
        -> &'static [(&'static str, &'static str)]
    {
        &[("/*", "*/")]
    }

    pub(crate) fn c_quotes() -> &'static [(&'static str, &'static str)] {
        &[("\"", "\"")]
    }

    pub(crate) fn func_allows_nested() -> bool {
        Self::blank_allows_nested()
    }

    pub(crate) fn func_line_comments() -> &'static [&'static str] {
        Self::blank_line_comments()
    }

    pub(crate) fn func_multi_line_comments()
        -> &'static [(&'static str, &'static str)]
    {
        &[("(*", "*)")]
    }

    pub(crate) fn func_quotes() -> &'static [(&'static str, &'static str)] {
        Self::c_quotes()
    }

    pub(crate) fn hash_allows_nested() -> bool {
        Self::blank_allows_nested()
    }

    pub(crate) fn hash_line_comments() -> &'static [&'static str] {
        &["#"]
    }

    pub(crate) fn hash_multi_line_comments()
        -> &'static [(&'static str, &'static str)]
    {
        Self::blank_multi_line_comments()
    }

    pub(crate) fn hash_quotes() -> &'static [(&'static str, &'static str)] {
        Self::blank_quotes()
    }

    pub(crate) fn haskell_allows_nested() -> bool {
        true
    }

    pub(crate) fn haskell_line_comments() -> &'static [&'static str] {
        &["--"]
    }

    pub(crate) fn haskell_multi_line_comments()
        -> &'static [(&'static str, &'static str)]
    {
        &[("{-", "-}")]
    }

    pub(crate) fn haskell_quotes() -> &'static [(&'static str, &'static str)] {
        Self::blank_quotes()
    }

    pub(crate) fn html_allows_nested() -> bool {
        Self::blank_allows_nested()
    }

    pub(crate) fn html_line_comments() -> &'static [&'static str] {
        Self::blank_line_comments()
    }

    pub(crate) fn html_multi_line_comments()
        -> &'static [(&'static str, &'static str)]
    {
        &[("<!--", "-->")]
    }

    pub(crate) fn html_quotes() -> &'static [(&'static str, &'static str)] {
        Self::c_quotes()
    }

    pub(crate) fn pro_allows_nested() -> bool {
        Self::blank_allows_nested()
    }

    pub(crate) fn pro_line_comments() -> &'static [&'static str] {
        &["%"]
    }

    pub(crate) fn pro_multi_line_comments()
        -> &'static [(&'static str, &'static str)]
    {
        &[("/*", "*/")]
    }

    pub(crate) fn pro_quotes() -> &'static [(&'static str, &'static str)] {
        Self::c_quotes()
    }

    /// Returns the display name of a language.
    ///
    /// ```
    /// # use tokei::*;
    /// let bash = LanguageType::Bash;
    ///
    /// assert_eq!(bash.name(), "BASH");
    /// ```
    pub fn name(self) -> &'static str {
        match self {
            {{~#each languages}}
                {{@key}} =>
                {{#if this.name}}
                    "{{~name}}"
                {{else}}
                    "{{~@key}}"
                {{~/if}},
            {{~/each}}
        }
    }

    pub(crate) fn is_blank(self) -> bool {
        match self {
            {{#each languages}}
                {{#if this.blank}}
                    {{@key}} => true,
                {{/if}}
            {{/each}}
            _ => false,
        }
    }

    pub(crate) fn is_fortran(self) -> bool {
        self == LanguageType::FortranModern ||
        self == LanguageType::FortranLegacy
    }

    /// Provides every variant in a Vec
    pub fn list() -> &'static [Self] {
        &[
            {{#each languages}}
                {{@key}},
            {{~/each}}
        ]
    }

    /// Returns the single line comments of a language.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::Rust;
    /// assert_eq!(lang.line_comments(), &["//"]);
    /// ```
    pub fn line_comments(self) -> &'static [&'static str] {
        match self {
            {{#each languages}}
                {{~@key}} =>
                    {{#if this.line_comment}}
                        &[
                            {{~#each this.line_comment}}
                                "{{~this}}",
                            {{~/each}}
                        ],
                    {{else}}
                        {{#if this.base}}
                            Self::{{this.base}}_line_comments(),
                        {{else}}
                            Self::blank_line_comments(),
                        {{~/if}}
                    {{~/if}}
            {{~/each}}
        }
    }

    /// Returns the single line comments of a language.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::Rust;
    /// assert_eq!(lang.multi_line_comments(), &[("/*", "*/")]);
    /// ```
    pub fn multi_line_comments(self) -> &'static [(&'static str, &'static str)]
    {
        match self {
            {{#each languages}}
                {{~@key}} =>
                    {{#if this.multi_line}}
                        &[
                            {{~#each this.multi_line}}
                                (
                                {{~#each this}}
                                    "{{~this}}",
                                {{~/each}}
                                ),
                            {{~/each}}
                        ],
                    {{else}}
                        {{#if this.base}}
                            Self::{{this.base}}_multi_line_comments(),
                        {{else}}
                            Self::blank_multi_line_comments(),
                        {{~/if}}
                    {{~/if}}
            {{~/each}}
        }
    }


    /// Returns whether the language allows nested multi line comments.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::Rust;
    /// assert!(lang.allows_nested());
    /// ```
    pub fn allows_nested(self) -> bool {
        match self {
            {{#each languages}}
                {{~@key}} =>
                    {{~#if this.base}}
                        {{~#if this.nested}}
                            true
                        {{else}}
                                Self::{{this.base}}_allows_nested()
                        {{~/if}}
                    {{else}}
                        {{~#if this.nested}}
                            true
                        {{else}}
                            false
                        {{~/if}}
                    {{~/if}},
            {{~/each}}
        }
    }

    /// Returns what nested comments the language has. (Currently only D has
    /// any of this type.)
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::D;
    /// assert_eq!(lang.nested_comments(), &[("/+", "+/")]);
    /// ```
    pub fn nested_comments(self) -> &'static [(&'static str, &'static str)]
    {
        match self {
            {{#each languages}}
                {{~@key}} => &[
                    {{~#each this.nested_comments}}
                    (
                        {{~#each this}} "{{this}}", {{~/each}}
                    ),
                    {{~/each}}
                ],
            {{~/each}}
        }
    }

    /// Returns the quotes of a language.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::Rust;
    /// assert_eq!(lang.quotes(), &[("r#\"", "\"#"), ("#\"", "\"#"), ("\"", "\"")]);
    /// ```
    pub fn quotes(self) -> &'static [(&'static str, &'static str)] {
        match self {
            {{#each languages}}
                {{~@key}} =>
                    {{#if this.quotes}}
                        &[
                            {{~#each this.quotes}}
                                (
                                {{~#each this}}
                                    "{{this}}",
                                {{~/each}}
                                ),
                            {{~/each}}
                        ],
                    {{else}}
                        {{#if this.base}}
                            Self::{{this.base}}_quotes(),
                        {{else}}
                            Self::blank_quotes(),
                        {{~/if}}
                    {{~/if}}
            {{~/each}}
        }
    }

    /// Returns the doc quotes of a language.
    /// ```
    /// use tokei::LanguageType;
    /// let lang = LanguageType::Python;
    /// assert_eq!(lang.doc_quotes(), &[("\"\"\"", "\"\"\""), ("'''", "'''")]);
    /// ```
    pub fn doc_quotes(self) -> &'static [(&'static str, &'static str)] {
        match self {
            {{#each languages}}
            {{#if this.doc_quotes}}
                {{~@key}} =>
                        &[
                            {{~#each this.doc_quotes}}
                                (
                                {{~#each this}}
                                    "{{this}}",
                                {{~/each}}
                                ),
                            {{~/each}}
                        ],
            {{~/if}}
            {{~/each}}
            _ => &[],
        }
    }

    /// Get language from a file path. May open and read the file.
    ///
    /// ```no_run
    /// use tokei::{Config, LanguageType};
    ///
    /// let rust = LanguageType::from_path("./main.rs", &Config::default());
    ///
    /// assert_eq!(rust, Some(LanguageType::Rust));
    /// ```
    pub fn from_path<P: AsRef<Path>>(entry: P, _config: &Config)
        -> Option<Self>
    {
        let entry = entry.as_ref();

        if let Some(filename) = fsutils::get_filename(&entry) {
            match &*filename {
                {{~#each languages}}
                    {{~#if this.filenames}}
                        {{~#each this.filenames}}
                            "{{~this}}" {{~#unless @last}} | {{~/unless}}
                        {{~/each}}
                            => return Some({{~@key}}),
                    {{~/if}}
                {{~/each}}
                _ => ()
            }
        }

        let extension = fsutils::get_extension(&entry);
        let filetype = extension.as_ref()
            .map(|s| &**s)
            .or_else(|| get_filetype_from_shebang(&entry));

        if let Some(extension) = filetype {
            /*
            if let Some(ref languages) = config.languages {
                for (language, lang_config) in languages {
                    if lang_config.extensions.iter().any(|e| e == extension) {
                        return Some(*language)
                    }
                }
            }
            */

            match extension {
                {{~#each languages}}
                    {{~#if this.extensions}}
                        {{~#each this.extensions}}
                            "{{~this}}" {{~#unless @last}} | {{~/unless}}
                        {{~/each}}
                            => Some({{~@key}}),
                    {{~/if}}
                {{~/each}}
                extension => {
                    warn!("Unknown extension: {}", extension);
                    None
                },
            }
        } else {
            None
        }
    }
}

impl FromStr for LanguageType {
    type Err = &'static str;

    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match &*from {
            {{~#each languages}}
                {{~#if this.name}}
                    "{{~this.name}}"
                {{else}}
                    "{{~@key}}"
                {{~/if}}
                    => Ok({{~@key}}),
            {{~/each}}
            _ => Err("Language not found, please use `-l` to see all available\
                     languages."),
        }
    }
}

impl fmt::Display for LanguageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}


impl<'a> From<LanguageType> for Cow<'a, LanguageType> {
    fn from(from: LanguageType) -> Self {
        Cow::Owned(from)
    }
}

impl<'a> From<&'a LanguageType> for Cow<'a, LanguageType> {
    fn from(from: &'a LanguageType) -> Self {
        Cow::Borrowed(from)
    }
}


/// This is for getting the file type from the first line of a file
pub fn get_filetype_from_shebang(file: &Path) -> Option<&'static str>
{
    let file = match File::open(file) {
        Ok(file) => file,
        _ => return None,
    };

    let mut buf = BufReader::new(file);
    let mut line = String::new();
    let _ = buf.read_line(&mut line);

    let mut words = line.split_whitespace();
    match words.next() {
        Some("#!/bin/sh") => Some("sh"),
        Some("#!/bin/csh") => Some("csh"),
        Some("#!/usr/bin/perl") => Some("pl"),
        Some("#!/usr/bin/env") => {
            if let Some(word) = words.next() {
                match word {
                    {{~#each languages}}
                        {{~#if this.env}}
                            {{~#each this.env}}
                                "{{~this}}"
                                {{~#unless @last}}
                                    |
                                {{~/unless}}
                            {{~/each}}
                                => Some("{{this.extensions.[0]}}"),
                        {{~/if}}
                    {{~/each}}
                    env => {
                        warn!("Unknown environment: {:?}", env);
                        None
                    }
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

