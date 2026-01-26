use serde::Deserialize;

use super::style::{Color, NamedColor, Style};

#[derive(Debug, Deserialize)]
pub struct GhActionsConfig {
    #[serde(default)]
    pub icons: ActionsIcons,

    #[serde(default)]
    pub in_progress: InProgressConfig,

    #[serde(default)]
    pub success: SuccessConfig,

    #[serde(default)]
    pub failure: FailureConfig,

    #[serde(default)]
    pub cancelled: CancelledConfig,

    #[serde(default)]
    pub skipped: SkippedConfig,

    #[serde(default = "GhActionsConfig::default_content")]
    pub content: String,
}

impl GhActionsConfig {
    fn default_content() -> String {
        " {{.status}} ".to_string()
    }
}

impl Default for GhActionsConfig {
    fn default() -> Self {
        Self {
            icons: Default::default(),
            in_progress: Default::default(),
            success: Default::default(),
            failure: Default::default(),
            cancelled: Default::default(),
            skipped: Default::default(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ActionsIcons {
    #[serde(default = "ActionsIcons::default_in_progress")]
    pub in_progress: String,

    #[serde(default = "ActionsIcons::default_success")]
    pub success: String,

    #[serde(default = "ActionsIcons::default_failure")]
    pub failure: String,

    #[serde(default = "ActionsIcons::default_cancelled")]
    pub cancelled: String,

    #[serde(default = "ActionsIcons::default_skipped")]
    pub skipped: String,
}

impl ActionsIcons {
    fn default_in_progress() -> String {
        "".to_string()
    }

    fn default_success() -> String {
        "".to_string()
    }

    fn default_failure() -> String {
        "".to_string()
    }

    fn default_cancelled() -> String {
        "".to_string()
    }

    fn default_skipped() -> String {
        "".to_string()
    }
}

impl Default for ActionsIcons {
    fn default() -> Self {
        Self {
            in_progress: Self::default_in_progress(),
            success: Self::default_success(),
            failure: Self::default_failure(),
            cancelled: Self::default_cancelled(),
            skipped: Self::default_skipped(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct InProgressConfig {
    #[serde(default = "InProgressConfig::default_style")]
    pub style: Style,
}

impl InProgressConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::Black),
            background: Color::Fixed(226),
            decoration: vec![],
        }
    }
}

impl Default for InProgressConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SuccessConfig {
    #[serde(default = "SuccessConfig::default_style")]
    pub style: Style,
}

impl SuccessConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::White),
            background: Color::Fixed(34),
            decoration: vec![],
        }
    }
}

impl Default for SuccessConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FailureConfig {
    #[serde(default = "FailureConfig::default_style")]
    pub style: Style,
}

impl FailureConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::White),
            background: Color::Fixed(196),
            decoration: vec![],
        }
    }
}

impl Default for FailureConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CancelledConfig {
    #[serde(default = "CancelledConfig::default_style")]
    pub style: Style,
}

impl CancelledConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::White),
            background: Color::Fixed(244),
            decoration: vec![],
        }
    }
}

impl Default for CancelledConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SkippedConfig {
    #[serde(default = "SkippedConfig::default_style")]
    pub style: Style,
}

impl SkippedConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::White),
            background: Color::Fixed(244),
            decoration: vec![],
        }
    }
}

impl Default for SkippedConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}
