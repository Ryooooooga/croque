use serde::Deserialize;

use super::style::{Color, NamedColor, Style};

#[derive(Debug, Deserialize)]
pub struct GitUserConfig {
    #[serde(default = "GitUserConfig::default_style")]
    pub style: Style,

    #[serde(default = "GitUserConfig::default_content")]
    pub content: String,
}

impl GitUserConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::Black),
            background: Color::Fixed(117),
            decoration: vec![],
        }
    }

    fn default_content() -> String {
        "  {{.name}} ".to_string()
    }
}

impl Default for GitUserConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
            content: Self::default_content(),
        }
    }
}
