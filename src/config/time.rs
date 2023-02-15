use serde::Deserialize;

use super::style::{Color, NamedColor, Style};

#[derive(Debug, Deserialize)]
pub struct TimeConfig {
    #[serde(default = "TimeConfig::default_format")]
    pub format: String,

    #[serde(default = "TimeConfig::default_utc")]
    pub utc: bool,

    #[serde(default = "TimeConfig::default_style")]
    pub style: Style,

    #[serde(default = "TimeConfig::default_content")]
    pub content: String,
}

impl TimeConfig {
    fn default_format() -> String {
        "%Y/%m/%d %H:%M:%S".to_string()
    }

    fn default_utc() -> bool {
        false
    }

    fn default_style() -> Style {
        Style {
            foreground: Color::Fixed(8),
            background: Color::Named(NamedColor::White),
            decoration: vec![],
        }
    }

    fn default_content() -> String {
        "  {{.time}} ".to_string()
    }
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            format: Self::default_format(),
            utc: Self::default_utc(),
            style: Self::default_style(),
            content: Self::default_content(),
        }
    }
}
