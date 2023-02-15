use super::style::{Color, NamedColor, Style};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DurationConfig {
    #[serde(default = "DurationConfig::default_style")]
    pub style: Style,

    #[serde(default = "DurationConfig::default_content")]
    pub content: String,
}

impl DurationConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::White),
            background: Color::Fixed(242),
            decoration: vec![],
        }
    }

    fn default_content() -> String {
        " 祥{{.duration}} ".to_string()
    }
}

impl Default for DurationConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
            content: Self::default_content(),
        }
    }
}
