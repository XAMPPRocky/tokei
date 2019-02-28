use std::str::FromStr;
use std::{fmt, process, io::{self, Write}};

use clap::crate_version;

use tokei::{Language, LanguageType};
use crate::input::Format;

pub const FALLBACK_ROW_LEN: usize = 79;
const NO_LANG_HEADER_ROW_LEN: usize = 67;
const NO_LANG_ROW_LEN: usize = 61;
const NO_LANG_ROW_LEN_NO_SPACES: usize = 54;
const IDENT_INACCURATE: &str = "(!)";

pub fn crate_version() -> String {
    if Format::supported().is_empty() {
        format!("{} compiled without serialization formats.", crate_version!())
    } else {
        format!(
            "{} compiled with serialization support: {}",
            crate_version!(),
            Format::supported().join(", ")
        )
    }
}

pub fn setup_logger(verbose_option: u64) {
    use log::LevelFilter;

    let mut builder = ::env_logger::Builder::new();

    let filter_level = match verbose_option {
        1 => LevelFilter::Warn,
        2 => LevelFilter::Debug,
        3 => LevelFilter::Trace,
        _ => LevelFilter::Error,
    };

    builder.filter(None, filter_level);
    builder.init();
}

pub fn parse_or_exit<T>(s: &str) -> T
where T: FromStr,
      T::Err: fmt::Display
{
    T::from_str(s).unwrap_or_else(|e| {
        eprintln!("Error:\n{}", e);
        process::exit(1);
    })
}

pub fn print_header<W: Write>(sink: &mut W, row: &str, columns: usize)
    -> io::Result<()>
{
    writeln!(sink, "{}", row)?;
    writeln!(sink, " {:<6$} {:>12} {:>12} {:>12} {:>12} {:>12}",
                "Language",
                "Files",
                "Lines",
                "Code",
                "Comments",
                "Blanks",
                columns - NO_LANG_HEADER_ROW_LEN)?;

    writeln!(sink, "{}", row)
}

pub fn print_results<'a, I, W>(sink: &mut W, row: &str, columns: usize, languages: I, list_files: bool)
    -> io::Result<()>
    where I: Iterator<Item = (&'a LanguageType, &'a Language)>,
          W: Write,
{
    let path_len = columns - NO_LANG_ROW_LEN_NO_SPACES;
    let lang_section_len = columns;
    for (name, language) in languages.filter(isnt_empty) {
        print_language(sink, lang_section_len, language, name.name())?;

        if list_files {
            writeln!(sink, "{}", row)?;
            for stat in &language.stats {
                writeln!(sink, "{:1$}", stat, path_len)?;
            }
            writeln!(sink, "{}", row)?;
        }
    }

    Ok(())
}

pub fn isnt_empty(&(_, language): &(&LanguageType, &Language)) -> bool {
    !language.is_empty()
}

pub fn print_language<W>(sink: &mut W,
                     lang_section_len: usize,
                     language: &Language,
                     name: &str)
    -> io::Result<()>
    where W: Write,
{
    if language.inaccurate {
        write!(sink, " {} {:<len$} ",
               name, IDENT_INACCURATE,
               len = lang_section_len - (NO_LANG_ROW_LEN + name.len() + 1))?;
    } else {
        write!(sink, " {:<len$} ", name, len = lang_section_len - NO_LANG_ROW_LEN)?;
    }
    writeln!(sink,
             "{:>6} {:>12} {:>12} {:>12} {:>12}",
             language.stats.len(),
             language.lines,
             language.code,
             language.comments,
             language.blanks)
}

pub fn print_inaccuracy_warning<W>(sink: &mut W) -> io::Result<()> where W: Write {
    writeln!(sink, "Note: results can be inaccurate for languages marked with '{}'", IDENT_INACCURATE)
}

// This is for a best efforts approach where we use box drawing characters
// for the separator lines, but only if we can easily determine that
// the terminal encoding is set to UTF8. We use the POSIX environment variables for that.
//
// The environment variables are not set by default under windows except in posix-y
// environments like WSL, cygwin, MSYS2.
// Unicode is badly supported in cmd.exe and powershell and not activated by default
// so we just fall back on ASCII there without checking the so called code page.
//
// The variables are assumed to follow XGP syntax:
// `language[_territory[.codeset]][@modifier]`
// https://www.gnu.org/software/libc/manual/html_node/Locale-Names.html
pub fn locale_requests_utf8() -> bool {
    // LC_ALL overrides LC_CTYPE which overrides LANG
    // LC_CTYPE is the locale category used for the same purpose in the `tree` utility
    let env_variables = ["LC_ALL", "LC_CTYPE", "LANG"];
    let env = env_variables.iter()
        .map(|&env| std::env::var(env))
        .find(|env|
            // filter out unset and empty variables
            env != &Err(std::env::VarError::NotPresent)
                && env != &Ok("".to_owned())
        );

    let lang = match env {
        // some variable is set and contains a valid unicode string
        Some(Ok(lang)) => lang,
        _ => return false,
    };

    // either a path or invalid
    // paths are rare, so we don't support them for simplicity and fall back to ascii
    if lang.contains('/') {
        return false;
    }
    // get the text between the first '.' and the first '@' (if it exists)
    // that should be the codeset
    let rest = match lang.split('.').nth(1) {
        Some(rest) => rest,
        None => return false,
    };
    let codeset = rest.split('@').next().unwrap();
    codeset.eq_ignore_ascii_case("UTF-8") || codeset.eq_ignore_ascii_case("utf8")
}
