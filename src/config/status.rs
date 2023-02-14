use super::style::{Color, NamedColor, Style};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StatusConfig {
    #[serde(default)]
    pub icons: StatusIcons,

    #[serde(default)]
    pub succeeded: SucceededStatusConfig,

    #[serde(default)]
    pub failed: FailedStatusConfig,

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
            succeeded: Default::default(),
            failed: Default::default(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StatusIcons {
    pub succeeded: String,
    pub failed: String,
    pub root: String,
    pub jobs: String,
}

impl StatusIcons {
    fn default_succeeded() -> String {
        "✓".to_string()
    }
    fn default_failed() -> String {
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
            succeeded: Self::default_succeeded(),
            failed: Self::default_failed(),
            root: Self::default_root(),
            jobs: Self::default_jobs(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SucceededStatusConfig {
    #[serde(default = "SucceededStatusConfig::default_style")]
    pub style: Style,
}

impl SucceededStatusConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::Green),
            background: Color::Named(NamedColor::White),
            decoration: vec![],
        }
    }
}

impl Default for SucceededStatusConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FailedStatusConfig {
    #[serde(default = "FailedStatusConfig::default_style")]
    pub style: Style,

    #[serde(default = "FailedStatusConfig::default_display_exit_code")]
    pub display_exit_code: bool,
}

impl FailedStatusConfig {
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

impl Default for FailedStatusConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
            display_exit_code: Self::default_display_exit_code(),
        }
    }
}
