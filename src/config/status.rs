use super::style::{Color, NamedColor, Style};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StatusConfig {
    #[serde(default)]
    pub icons: StatusIcons,

    #[serde(default)]
    pub success: SuccessStatusConfig,

    #[serde(default)]
    pub error: ErrorStatusConfig,

    #[serde(default = "StatusConfig::default_content")]
    pub content: String,
}

impl StatusConfig {
    fn default_content() -> String {
        " {{.exit_status}}{{.root}}{{.jobs}} ".to_string()
    }
}

impl Default for StatusConfig {
    fn default() -> Self {
        Self {
            icons: Default::default(),
            success: Default::default(),
            error: Default::default(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StatusIcons {
    #[serde(default = "StatusIcons::default_success")]
    pub success: String,

    #[serde(default = "StatusIcons::default_error")]
    pub error: String,

    #[serde(default = "StatusIcons::default_root")]
    pub root: String,

    #[serde(default = "StatusIcons::default_jobs")]
    pub jobs: String,
}

impl StatusIcons {
    fn default_success() -> String {
        "✓".to_string()
    }
    fn default_error() -> String {
        "".to_string()
    }
    fn default_root() -> String {
        "".to_string()
    }
    fn default_jobs() -> String {
        "".to_string()
    }
}

impl Default for StatusIcons {
    fn default() -> Self {
        Self {
            success: Self::default_success(),
            error: Self::default_error(),
            root: Self::default_root(),
            jobs: Self::default_jobs(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SuccessStatusConfig {
    #[serde(default = "SuccessStatusConfig::default_style")]
    pub style: Style,
}

impl SuccessStatusConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::Green),
            background: Color::Named(NamedColor::White),
            decoration: vec![],
        }
    }
}

impl Default for SuccessStatusConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ErrorStatusConfig {
    #[serde(default = "ErrorStatusConfig::default_style")]
    pub style: Style,

    #[serde(default = "ErrorStatusConfig::default_display_exit_code")]
    pub display_exit_code: bool,
}

impl ErrorStatusConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::White),
            background: Color::Named(NamedColor::Red),
            decoration: vec![],
        }
    }

    fn default_display_exit_code() -> bool {
        true
    }
}

impl Default for ErrorStatusConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
            display_exit_code: Self::default_display_exit_code(),
        }
    }
}
