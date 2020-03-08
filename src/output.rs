use num_format::{CustomFormat, Grouping};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum NumberFormatStyle {
    // 1234 (Default)
    Plain,
    // 1,234
    Commas,
    // 1.234
    Dots,
}

impl NumberFormatStyle {
    pub fn all() -> &'static [&'static str] {
        &["plain", "commas", "dots"]
    }

    pub fn from_str(input: &str) -> Option<NumberFormatStyle> {
        Some(match input {
            "plain" => NumberFormatStyle::Plain,
            "commas" => NumberFormatStyle::Commas,
            "dots" => NumberFormatStyle::Dots,
            _ => return None,
        })
    }

    pub fn get_format(self) -> CustomFormat {
        CustomFormat::builder()
            .grouping(Grouping::Standard)
            .separator(self.separator())
            .build()
            .unwrap_or_else(|_| panic!("Could not construct format from variant {:?}", self))
    }

    fn separator(self) -> &'static str {
        match self {
            NumberFormatStyle::Plain => "",
            NumberFormatStyle::Commas => ",",
            NumberFormatStyle::Dots => ".",
        }
    }
}
