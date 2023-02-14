use serde::Deserialize;

use super::style::{Color, NamedColor, Style};

#[derive(Debug, Deserialize)]
pub struct PathConfig {
    #[serde(default)]
    pub normal: NormalPathConfig,

    #[serde(default)]
    pub error: ErrorPathConfig,

    #[serde(default)]
    pub shrink: ShrinkPathConfig,

    #[serde(default = "PathConfig::default_aliases")]
    pub aliases: Vec<PathAlias>,

    #[serde(default = "PathConfig::default_content")]
    pub content: String,
}

impl PathConfig {
    fn default_aliases() -> Vec<PathAlias> {
        vec![PathAlias {
            path: "~".to_string(),
            alias: "~".to_string(),
        }]
    }

    fn default_content() -> String {
        " {{.path}} ".to_string()
    }
}

impl Default for PathConfig {
    fn default() -> Self {
        Self {
            normal: NormalPathConfig::default(),
            error: ErrorPathConfig::default(),
            shrink: ShrinkPathConfig::default(),
            aliases: Self::default_aliases(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NormalPathConfig {
    #[serde(default = "NormalPathConfig::default_style")]
    pub style: Style,
}

impl NormalPathConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::Black),
            background: Color::Named(NamedColor::Blue),
            decoration: vec![],
        }
    }
}

impl Default for NormalPathConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ErrorPathConfig {
    #[serde(default = "ErrorPathConfig::default_style")]
    pub style: Style,
}

impl ErrorPathConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::Black),
            background: Color::Named(NamedColor::Red),
            decoration: vec![],
        }
    }
}

impl Default for ErrorPathConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ShrinkPathConfig {
    #[serde(default = "ShrinkPathConfig::default_enabled")]
    pub enabled: bool,

    #[serde(default = "ShrinkPathConfig::default_length")]
    pub length: usize,
}

impl ShrinkPathConfig {
    fn default_enabled() -> bool {
        true
    }

    fn default_length() -> usize {
        1
    }
}

impl Default for ShrinkPathConfig {
    fn default() -> Self {
        Self {
            enabled: Self::default_enabled(),
            length: Self::default_length(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PathAlias {
    pub path: String,
    pub alias: String,
}
