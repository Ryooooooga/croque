use serde::Deserialize;

use super::style::{Color, NamedColor, Style};

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    #[serde(default = "UserConfig::default_style")]
    pub style: Style,

    #[serde(default = "UserConfig::default_content")]
    pub content: String,
}

impl UserConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::White),
            background: Color::Fixed(8),
            decoration: vec![],
        }
    }

    fn default_content() -> String {
        " {{.user}}@{{.host}} ".to_string()
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
            content: Self::default_content(),
        }
    }
}
