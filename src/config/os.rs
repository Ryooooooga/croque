use serde::Deserialize;

use super::style::{Color, NamedColor, Style};

#[derive(Debug, Default, Deserialize)]
pub struct OsConfig {
    #[serde(default)]
    pub linux: LinuxOsConfig,

    #[serde(default)]
    pub mac: MacOsConfig,

    #[serde(default)]
    pub windows: WindowsOsConfig,
}

#[derive(Debug, Deserialize)]
pub struct LinuxOsConfig {
    #[serde(default = "LinuxOsConfig::default_style")]
    pub style: Style,

    #[serde(default = "LinuxOsConfig::default_content")]
    pub content: String,
}

impl LinuxOsConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::White),
            background: Color::Fixed(33),
            decoration: vec![],
        }
    }

    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for LinuxOsConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MacOsConfig {
    #[serde(default = "MacOsConfig::default_style")]
    pub style: Style,

    #[serde(default = "MacOsConfig::default_content")]
    pub content: String,
}

impl MacOsConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::White),
            background: Color::Fixed(33),
            decoration: vec![],
        }
    }

    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for MacOsConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct WindowsOsConfig {
    #[serde(default = "WindowsOsConfig::default_style")]
    pub style: Style,

    #[serde(default = "WindowsOsConfig::default_content")]
    pub content: String,
}

impl WindowsOsConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::White),
            background: Color::Fixed(33),
            decoration: vec![],
        }
    }

    fn default_content() -> String {
        "  ".to_string()
    }
}

impl Default for WindowsOsConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
            content: Self::default_content(),
        }
    }
}
