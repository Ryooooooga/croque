use super::style::{Color, HexColor, NamedColor, Style};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DirenvConfig {
    #[serde(default)]
    pub icons: DirenvIcons,

    #[serde(default)]
    pub loaded: LoadedDirenvConfig,

    #[serde(default)]
    pub found: FoundDirenvConfig,

    #[serde(default)]
    pub disallowed: DisallowedDirenvConfig,

    #[serde(default = "DirenvConfig::default_content")]
    pub content: String,
}

impl DirenvConfig {
    fn default_content() -> String {
        " .{{.status}} ".to_string()
    }
}

impl Default for DirenvConfig {
    fn default() -> Self {
        Self {
            icons: Default::default(),
            loaded: Default::default(),
            found: Default::default(),
            disallowed: Default::default(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DirenvIcons {
    #[serde(default = "DirenvIcons::default_loaded")]
    pub loaded: String,

    #[serde(default = "DirenvIcons::default_found")]
    pub found: String,

    #[serde(default = "DirenvIcons::default_disallowed")]
    pub disallowed: String,
}

impl DirenvIcons {
    fn default_loaded() -> String {
        "".to_string()
    }
    fn default_found() -> String {
        "".to_string()
    }
    fn default_disallowed() -> String {
        "".to_string()
    }
}

impl Default for DirenvIcons {
    fn default() -> Self {
        Self {
            loaded: Self::default_loaded(),
            found: Self::default_found(),
            disallowed: Self::default_disallowed(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoadedDirenvConfig {
    #[serde(default = "LoadedDirenvConfig::default_style")]
    pub style: Style,
}

impl LoadedDirenvConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::White),
            background: Color::Hex(HexColor(0x96, 0xa1, 0xb5)),
            decoration: vec![],
        }
    }
}

impl Default for LoadedDirenvConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FoundDirenvConfig {
    #[serde(default = "FoundDirenvConfig::default_style")]
    pub style: Style,
}

impl FoundDirenvConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::White),
            background: Color::Fixed(243),
            decoration: vec![],
        }
    }
}

impl Default for FoundDirenvConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DisallowedDirenvConfig {
    #[serde(default = "DisallowedDirenvConfig::default_style")]
    pub style: Style,
}

impl DisallowedDirenvConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::White),
            background: Color::Fixed(243),
            decoration: vec![],
        }
    }
}

impl Default for DisallowedDirenvConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}
