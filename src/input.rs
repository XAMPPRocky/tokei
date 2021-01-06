use std::{collections::BTreeMap, error::Error, str::FromStr};
use serde_json::{json, Map};

use tokei::{Language, LanguageType, Languages};

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
            Json,
            $(
                #[cfg(feature = $feature)] $variant
            ),+
            // TODO: Allow adding format at runtime when used as a lib?
        }

        impl Format {
            pub fn supported() -> &'static [&'static str] {
                &[
                    "json",
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
                if let Ok(result) = serde_json::from_str(input) {
                    return Some(result)
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

            pub fn print(&self, languages: &Languages) -> Result<String, Box<dyn Error>> {
                // To serde_json Map and add summary
                let mut map = Map::new();
                for (language_type, language) in languages.into_iter() {
                    map.insert(language_type.to_string(), json!(language));
                }

                map.insert(String::from("Total"), json!(languages.total()));

                match *self {
                    Format::Json => Ok(serde_json::to_string(&map)?),
                    $(
                        #[cfg(feature = $feature)] Format::$variant => {
                            let print= &{ $print_kode };
                            Ok(print(&map)?)
                        }
                    ),+
                }
            }
        }

        impl FromStr for Format {
            type Err = String;

            fn from_str(format: &str) -> Result<Self, Self::Err> {
                match format {
                    "json" => Ok(Format::Json),
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
                .map_err(|e: hex::FromHexError| <Box<dyn Error>>::from(e))
                .and_then(|hex: Vec<_>| Ok(serde_cbor::from_slice(&hex)?))
        },
        |languages| serde_cbor::to_vec(&languages).map(hex::encode),

    (json, "json", Json [serde_json]) =>
        serde_json::from_str,
        serde_json::to_string,

    (yaml, "yaml", Yaml [serde_yaml]) =>
        serde_yaml::from_str,
        serde_yaml::to_string,
);

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

            convert_input(&contents)
        }
        Err(_) => {
            if input == "stdin" {
                let mut stdin = ::std::io::stdin();
                let mut buffer = String::new();

                let _ = stdin.read_to_string(&mut buffer);
                convert_input(&buffer)
            } else {
                convert_input(input)
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

fn convert_input(contents: &str) -> Option<LanguageMap> {
    self::Format::parse(&contents)
}
