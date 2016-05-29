use super::{Language, LanguageName};


#[derive(Debug)]
pub struct Languages {
    inner: BTreeMap<LanguageName, Language>,
}


impl Languages {
    pub fn convert_input(contents: String) -> Option<BTreeMap<String, Language>> {
        if contents.is_empty() {
            None
        } else if let Ok(result) = serde_json::from_str(&*contents) {
            Some(result)
        } else if let Ok(result) = serde_yaml::from_str(&*contents) {
            Some(result)
        } else if let Ok(result) = serde_cbor::from_slice(&*contents.from_hex().unwrap()) {
            Some(result)
        } else {
            None
        }
    }

    pub fn get_statistics<I, P>(paths: P, ignored: P)
        where I: Iterator<Item = &str>,
              P: Into<Cow<'a, I>>
    {

        get_all_files(paths.into(), ignored_directories.into(), &mut self.inner);

        self.inner.par_iter_mut().for_each(|&mut (name, ref mut language)| {
            if language.files.is_empty() {
                return;
            }

            language.total_files = language.files.len();
            let is_fortran = name == FortranModern || name == FortranLegacy;

            let files: Vec<_> = language.files.drain(..).collect();
            for file in files {
                let mut is_in_comments = false;
                let mut previous_comment_start = "";
                let mut comment_depth: usize = 0;
                let mut stats = Stats::new(opt_or_cont!(file.to_str()));

                let contents = {
                    let mut contents = String::new();
                    let _ = rs_or_cont!(rs_or_cont!(File::open(file)).read_to_string(&mut contents));
                    contents
                };

                let lines = contents.lines();
                stats.lines += lines.size_hint().0;

                if language.is_blank() {
                    stats.code += lines.count();
                    continue;
                }

                'line: for line in lines {
                    // FORTRAN has a rule where it only counts as a comment if it's the first character
                    // in the column, so removing starting whitespace could cause a miscount.
                    let line = if is_fortran {
                        line
                    } else {
                        line.trim()
                    };

                    if line.trim().is_empty() {
                        stats.blanks += 1;
                        continue;
                    }

                    for &(multi_line, multi_line_end) in &language.multi_line {
                        if line.starts_with(multi_line) ||
                           has_trailing_comments(line, multi_line, multi_line_end, language.nested) {
                            previous_comment_start = multi_line;
                            is_in_comments = true;
                            if language.nested {
                                comment_depth += 1;
                            }
                        }
                    }


                    if is_in_comments {
                        for &(multi_line, multi_line_end) in &language.multi_line {
                            if multi_line == previous_comment_start && line.contains(multi_line_end) {
                                if language.nested {
                                    comment_depth -= 1;
                                    if comment_depth == 0 {
                                        is_in_comments = false;
                                    }
                                } else {
                                    is_in_comments = false;
                                }
                            }
                        }
                        stats.comments += 1;
                        continue;
                    }

                    for single in &language.line_comment {
                        if line.starts_with(single) {
                            stats.comments += 1;
                            continue 'line;
                        }
                    }
                    stats.code += 1;
                }

                *language += stats;
            }
        });
    }

    pub fn new() -> Self {
        let map = btreemap! {
            ActionScript => Language::new_c(),
            Assembly => Language::new_single(vec![";"]),
            Bash => Language::new_hash(),
            Batch => Language::new_single(vec!["REM"]),
            C => Language::new_c(),
            CHeader => Language::new_c(),
            Clojure => Language::new_single(vec![";","#"]),
            CoffeeScript => Language::new(vec!["#"], vec![("###", "###")]),
            ColdFusion => Language::new_multi(vec![("<!---", "--->")]),
            ColdFusionScript => Language::new_c(),
            Coq => Language::new_func(),
            Cpp => Language::new_c(),
            CppHeader => Language::new_c(),
            CSharp => Language::new_c(),
            CShell => Language::new_hash(),
            Css => Language::new_c(),
            D => Language::new_c(),
            Dart => Language::new_c(),
            DeviceTree => Language::new_c(),
            Erlang => Language::new_single(vec!["%"]),
            FortranLegacy => Language::new_single(vec!["c","C","!","*"]),
            FortranModern => Language::new_single(vec!["!"]),
            Go => Language::new_c(),
            Haskell => Language::new_single(vec!["--"]),
            Html => Language::new_html(),
            Idris => Language::new(vec!["--"], vec![("{-", "-}")]),
            Isabelle => Language::new(vec!["--"], vec![("{*","*}"), ("(*","*)"), ("‹","›"), ("\\<open>", "\\<close>")]),
            Jai => Language::new_c(),
            Java => Language::new_c(),
            JavaScript => Language::new_c(),
            Json => Language::new_blank(),
            Jsx => Language::new_c(),
            Julia => Language::new(vec!["#"], vec![("#=", "=#")]),
            Kotlin => Language::new_c(),
            Less => Language::new_c(),
            LinkerScript => Language::new_c(),
            Lisp => Language::new(vec![";"], vec![("#|", "|#")]),
            Lua => Language::new(vec!["--"], vec![("--[[", "]]")]),
            Makefile => Language::new_hash(),
            Markdown => Language::new_blank(),
            Mustache => Language::new_multi(vec![("{{!", "}}")]),
            Nim => Language::new_hash(),
            ObjectiveC => Language::new_c(),
            ObjectiveCpp => Language::new_c(),
            OCaml => Language::new_func(),
            Oz => Language::new_pro(),
            Pascal => Language::new(vec!["//","(*"], vec![("{", "}")]),
            Perl => Language::new(vec!["#"], vec![("=", "=cut")]),
            Php => Language::new(vec!["#","//"], vec![("/*", "*/")]),
            Polly => Language::new_html(),
            Prolog => Language::new_pro(),
            Protobuf => Language::new_single(vec!["//"]),
            Python => Language::new(vec!["#"], vec![("'''", "'''")]),
            Qcl => Language::new_c(),
            R => Language::new_hash(),
            Ruby => Language::new(vec!["#"], vec![("=begin", "=end")]),
            RubyHtml => Language::new_html(),
            Rust => Language::new_c().nested(),
            Sass => Language::new_c(),
            Scala => Language::new_c(),
            Sml => Language::new_func(),
            Sql => Language::new(vec!["--"], vec![("/*", "*/")]),
            Swift => Language::new_c(),
            Tex => Language::new_single(vec!["%"]),
            Text => Language::new_blank(),
            Toml => Language::new_hash(),
            TypeScript => Language::new_c(),
            UnrealScript => Language::new_c(),
            VimScript => Language::new_single(vec!["\""]),
            Wolfram => Language::new_func(),
            Xml => Language::new_html(),
            Yaml => Language::new_hash(),
            Zsh => Language::new_hash(),
        };

        Languages { inner: map }
    }
}
