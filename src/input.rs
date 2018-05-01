use std::str::FromStr;
use std::error::Error;
use std::collections::BTreeMap;

use tokei::Languages;
use tokei::{LanguageType, Language};

type LanguageMap = BTreeMap<LanguageType, Language>;

macro_rules! supported_formats {
    ($(
        ($name:ident, $feature:expr, $variant:ident [$($krate:ident),+]) =>
            $parse_kode:expr,
            $print_kode:expr,
    )+) => (
        $( // for each format
            $( // for each required krate
                #[cfg(feature = $feature)] extern crate $krate;
            )+
        )+

        /// Supported serialization formats.
        ///
        /// To enable all formats compile with the `all` feature.
        #[derive(Debug)]
        pub enum Format {
            $(
                #[cfg(feature = $feature)] $variant
            ),+

            // TODO: Allow adding format at runtime when used as a lib?
        }

        impl Format {
            pub fn supported() -> &'static [&'static str] {
                &[
                    $(
                        #[cfg(feature = $feature)] stringify!($name)
                    ),+
                ]
            }

            pub fn all() -> &'static [&'static str] {
                &[
                    $( stringify!($name) ),+
                ]
            }

            pub fn all_feature_names() -> &'static [&'static str] {
                &[
                    $( $feature ),+
                ]
            }

            pub fn not_supported() -> &'static [&'static str] {
                &[
                    $(
                        #[cfg(not(feature = $feature))] stringify!($name)
                    ),+
                ]
            }

            pub fn parse(input: &str) -> Option<LanguageMap> {
                if input.is_empty() {
                    return None
                }

                $(
                    // attributes are not yet allowed on `if` expressions
                    #[cfg(feature = $feature)]
                    {
                        let parse = &{ $parse_kode };

                        if let Ok(result) = parse(input) {
                            return Some(result)
                        }
                    }
                )+

                // Didn't match any of the compiled serialization formats
                None
            }

            pub fn print(&self, _languages: Languages) -> Result<String, Box<Error>> {
                match *self {
                    $(
                        #[cfg(feature = $feature)] Format::$variant => {
                            let print= &{ $print_kode };
                            Ok(print(&_languages)?)
                        }
                    ),+
                }
            }
        }

        impl FromStr for Format {
            type Err = String;

            fn from_str(format: &str) -> Result<Self, Self::Err> {
                match format {
                    $(
                        stringify!($name) => {
                            #[cfg(feature = $feature)]
                            return Ok(Format::$variant);

                            #[cfg(not(feature = $feature))]
                            return Err(format!(
"This version of tokei was compiled without \
any '{format}' serialization support, to enable serialization, \
reinstall tokei with the features flag.

    cargo install tokei --features {feature}

If you want to enable all supported serialization formats, you can use the 'all' feature.

    cargo install tokei --features all\n",
                                format = stringify!($name),
                                feature = $feature)
                            );
                        }
                    ),+
                    format => Err(format!("{:?} is not a supported serialization format", format)),
                }
            }
        }
    )
}

// The ordering of these determines the attempted order when parsing.
supported_formats!(
    (cbor, "cbor", Cbor [serde_cbor, hex]) =>
        |input| {
            hex::FromHex::from_hex(input)
                .map_err(|e: hex::FromHexError| <Box<Error>>::from(e))
                .and_then(|hex: Vec<_>| Ok(serde_cbor::from_slice(&hex)?))
        },
        |languages| serde_cbor::to_vec(&languages).map(hex::encode),

    (json, "json", Json [serde_json]) =>
        serde_json::from_str,
        serde_json::to_string,

    (yaml, "yaml", Yaml [serde_yaml]) =>
        serde_yaml::from_str,
        serde_yaml::to_string,

    (toml, "toml-io", Toml [toml]) =>
        toml::from_str,
        toml::to_string,
    (csv, "csv-io", Csv [csv]) =>
        serde_yaml::from_str,
        csv_to_string,
);



// #[cfg(feature = "csv-io")]
// pub fn csv_parse(data: & str) -> Result<Languages, Box<Error>>{
//     let mut rdr = csv::ReaderBuilder::new()
//         .delimiter(b',')
//         .from_reader(data.as_bytes());

//     return rdr.records().next().unwrap();
    
// }


#[cfg(feature = "csv-io")]
pub fn csv_to_string(languages: & Languages) -> Result<String, String>{
    use std::io::Cursor;
    use std::io::Read;
    
    let cursor = Cursor::new(vec![]);
    let mut wtr = csv::Writer::from_writer(cursor);

    // headers
    let _ = wtr.write_record(&["language", "files", "blank", "comment", "code"]);
    for (name, language) in languages {
        let _ = wtr.serialize((name,
                       language.stats.len(),
                       language.blanks,
                       language.comments,
                       language.code));
    }
    
    let _ = wtr.flush();
    
    match wtr.into_inner() {
        Ok(mut lines) => {
            let mut result = String::new();
            lines.set_position(0);
            let _ = lines.read_to_string(&mut result);
            return Ok(result);
        },
        Err(e) => {
            println!("{:?}", e);
            return Err("Nope".to_string()); // TODO
        },
    }
}

    

pub fn add_input(input: &str, languages: &mut Languages) -> bool {
    use std::fs::File;
    use std::io::Read;

    let map = match File::open(input) {
        Ok(mut file) => {
            let contents = {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("Couldn't read file");
                contents
            };

            convert_input(contents)
        }
        Err(_) => {
            if input == "stdin" {
                let mut stdin = ::std::io::stdin();
                let mut buffer = String::new();

                let _ = stdin.read_to_string(&mut buffer);
                convert_input(buffer)
            } else {
                convert_input(String::from(input))
            }
        }
    };

    if let Some(map) = map {
        *languages += map;
        true
    } else {
        false
    }
}

fn convert_input(contents: String) -> Option<LanguageMap> {
    self::Format::parse(&contents)
}
