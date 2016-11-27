
pub use self::io::*;

#[cfg(feature = "io")]
mod io {
    extern crate serde_cbor;
    extern crate serde_json;
    extern crate serde_yaml;
    extern crate toml;

    use std::collections::BTreeMap;
    use tokei::Languages;

    pub fn add_input(input: &str, languages: &mut Languages) {
        use std::fs::File;
        use std::io::Read;

        let map = match File::open(input) {
            Ok(mut file) => {
                let contents = {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    contents
                };

                convert_input(contents)
            }
            Err(_) => {
                if input == "stdin" {
                    let mut stdin = std::io::stdin();
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

    fn convert_input(contents: String) -> Option<BTreeMap<LanguageType, Language>> {
        if contents.is_empty() {
            None
        } else if let Ok(result) = from_cbor(&contents) {
            Some(result)
        } else if let Ok(result) = from_json(&contents) {
            Some(result)
        } else if let Ok(result) = from_yaml(&contents) {
            Some(result)
        } else if let Some(result) = from_toml(&contents) {
            Some(result)
        } else {
            None
        }
    }

    #[cfg(feature = "cbor")]
    pub fn from_cbor<T>(contents: &String) -> serde_json::Result<T> {
        extern crate hex;
        use hex::FromHex;
        serde_cbor::from_slice(&contents.from_hex())
    }

    #[cfg(not(feature = "cbor"))]
    pub fn from_cbor(contents: &String) -> Result<(), ()> {
        Err(())
    }

    #[cfg(feature = "json")]
    pub fn from_json<T>(contents: &String) -> serde_json::Result<T> {
        serde_json::from_str(&contents)
    }

    #[cfg(not(feature = "json"))]
    pub fn from_json(contents: &String) -> Result<(), ()> {
        Err(())
    }

    #[cfg(feature = "toml-io")]
    pub fn from_toml<T>(contents: &String) -> serde_json::Result<T> {
        toml::decode_str(&contents)
    }

    #[cfg(not(feature = "toml-io"))]
    pub fn from_toml(contents: &String) -> Result<(), ()> {
        Err(())
    }

    #[cfg(feature = "yaml")]
    pub fn from_yaml<T>(contents: &String) -> serde_json::Result<T> {
        serde_yaml::from_str(&contents)
    }

    #[cfg(not(feature = "yaml"))]
    pub fn from_yaml(contents: &String) -> Result<(), ()> {
        Err(())
    }

    pub fn match_output(format: &str, languages: Languages) {
        match format {
            "cbor" => {
                let cbor: Vec<u8> = languages.to_cbor().unwrap();

                for byte in cbor {
                    print!("{:02x}", byte);
                }
            }
            "json" => print!("{}", languages.to_json().unwrap()),
            "toml" => print!("{}", languages.to_toml()),
            "yaml" => print!("{}", languages.to_yaml().unwrap()),
            _ => unreachable!(),
        }
    }

}


#[cfg(not(feature = "io"))]
#[allow(unused_variables)]
mod io {

    use std::error::Error;
    use std::io::{Write, stderr};
    use std::process;

    use tokei::Languages;

    const OUTPUT_ERROR: &'static str = "
    This version of tokei was compiled without any serialization formats, to
    enable serialization, reinstall tokei with the features flag.

        cargo install tokei --features all
";

    pub fn add_input(input: &str, map: &mut Languages) -> ! {
        if let Err(error) = write!(stderr(), "{}", OUTPUT_ERROR) {
            error!("{}", error.description());
        }
        process::exit(1);
    }

    pub fn match_output(format: &str, languages: Languages) -> ! {
        if let Err(error) = write!(stderr(), "{}", OUTPUT_ERROR) {
            error!("{}", error.description());
        }
        process::exit(1);
    }
}

