pub use self::io::*;

#[cfg(feature = "io")]
mod io {
    #[cfg(feature = "cbor")] extern crate serde_cbor;
    #[cfg(feature = "json")] extern crate serde_json;
    #[cfg(feature = "yaml")] extern crate serde_yaml;
    #[cfg(feature = "toml-io")] extern crate toml;
    #[cfg(feature = "cbor")] extern crate hex;

    use std::collections::BTreeMap;
    use tokei::Languages;
    use tokei::{LanguageType, Language};

    type LanguageMap = BTreeMap<LanguageType, Language>;

    pub fn add_input(input: &str, languages: &mut Languages) {
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
        }

    }

    fn convert_input(contents: String) -> Option<LanguageMap> {
        if contents.is_empty() {
            None
        } else if let Ok(result) = from_cbor(&contents) {
            Some(result)
        } else if let Ok(result) = from_json(&contents) {
            Some(result)
        } else if let Ok(result) = from_yaml(&contents) {
            Some(result)
        } else if let Ok(result) = from_toml(&contents) {
            Some(result)
        } else {
            None
        }
    }

    #[cfg(feature = "cbor")]
    pub fn from_cbor(contents: &String) -> serde_cbor::Result<LanguageMap> {
        use self::hex::FromHex;
        use std::error::Error;
        use std::process;

        let hex = match Vec::from_hex(contents) {
            Ok(hex) => hex,
            Err(err) => {
                errln!("{}", err.description());
                process::exit(1)
            }
        };
        serde_cbor::from_slice(&hex)
    }

    #[cfg(not(feature = "cbor"))]
    pub fn from_cbor(_: &String) -> Result<LanguageMap, ()> {
        Err(())
    }

    #[cfg(feature = "json")]
    pub fn from_json(contents: &String) -> serde_json::Result<LanguageMap> {
        serde_json::from_str(&contents)
    }

    #[cfg(not(feature = "json"))]
    pub fn from_json(_: &String) -> Result<LanguageMap, ()> {
        Err(())
    }

    #[cfg(feature = "toml-io")]
    pub fn from_toml(contents: &String) -> Result<LanguageMap, ()> {
        toml::decode_str(&contents).ok_or(())
    }

    #[cfg(not(feature = "toml-io"))]
    pub fn from_toml(_: &String) -> Result<LanguageMap, ()> {
        Err(())
    }

    #[cfg(feature = "yaml")]
    pub fn from_yaml(contents: &String) -> serde_yaml::Result<LanguageMap> {
        serde_yaml::from_str(&contents)
    }

    #[cfg(not(feature = "yaml"))]
    pub fn from_yaml(_: &String) -> Result<LanguageMap, ()> {
        Err(())
    }

    pub fn match_output(format: &str, languages: Languages) {
        match format {
            "cbor" => {
                let cbor: Vec<u8> = languages.to_cbor()
                    .expect("Couldn't convert to CBOR");

                for byte in cbor {
                    print!("{:02x}", byte);
                }
            }
            "json" => print!("{}", languages.to_json()
                             .expect("Couldn't convert to JSON")),
            "toml" => print!("{}", languages.to_toml()),
            "yaml" => print!("{}", languages.to_yaml()
                             .expect("Couldn't convert to YAML")),
            _ => unreachable!(),
        }
    }

}


#[cfg(not(feature = "io"))]
#[allow(unused_variables)]
mod io {
    use std::process;
    use tokei::Languages;

    const OUTPUT_ERROR: &'static str = "
    This version of tokei was compiled without any serialization formats, to
    enable serialization, reinstall tokei with the features flag.

        ALL:
        cargo install tokei --features all

        JSON:
        cargo install tokei --features json

        CBOR:
        cargo install tokei --features cbor

        YAML:
        cargo install tokei --features yaml

        CBOR:
        cargo install tokei --features cbor
";

    pub fn add_input(input: &str, map: &mut Languages) -> ! {
        errln!("{}", OUTPUT_ERROR);
        process::exit(1);
    }

    pub fn match_output(format: &str, languages: Languages) -> ! {
        errln!("{}", OUTPUT_ERROR);
        process::exit(1);
    }
}

