use serde::Deserialize;

use super::style::{Color, NamedColor, Style};

#[derive(Debug, Deserialize)]
pub struct GlabMergeRequestConfig {
    #[serde(default)]
    pub icons: MergeRequestIcons,

    #[serde(default)]
    pub open: OpenStateConfig,

    #[serde(default)]
    pub draft: DraftStateConfig,

    #[serde(default)]
    pub closed: ClosedStateConfig,

    #[serde(default)]
    pub merged: MergedStateConfig,

    #[serde(default = "GlabMergeRequestConfig::default_content")]
    pub content: String,
}

impl GlabMergeRequestConfig {
    fn default_content() -> String {
        "  !{{.number}}{{.state}}{{.pipeline}}{{.approved}}{{.comments}} ".to_string()
    }
}

impl Default for GlabMergeRequestConfig {
    fn default() -> Self {
        Self {
            icons: Default::default(),
            open: Default::default(),
            draft: Default::default(),
            closed: Default::default(),
            merged: Default::default(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MergeRequestIcons {
    #[serde(default = "MergeRequestIcons::default_open")]
    pub open: String,

    #[serde(default = "MergeRequestIcons::default_draft")]
    pub draft: String,

    #[serde(default = "MergeRequestIcons::default_closed")]
    pub closed: String,

    #[serde(default = "MergeRequestIcons::default_merged")]
    pub merged: String,

    #[serde(default = "MergeRequestIcons::default_approved")]
    pub approved: String,

    #[serde(default = "MergeRequestIcons::default_pipeline_pending")]
    pub pipeline_pending: String,

    #[serde(default = "MergeRequestIcons::default_pipeline_running")]
    pub pipeline_running: String,

    #[serde(default = "MergeRequestIcons::default_pipeline_success")]
    pub pipeline_success: String,

    #[serde(default = "MergeRequestIcons::default_pipeline_failed")]
    pub pipeline_failed: String,

    #[serde(default = "MergeRequestIcons::default_pipeline_canceled")]
    pub pipeline_canceled: String,

    #[serde(default = "MergeRequestIcons::default_comment")]
    pub comment: String,
}

impl MergeRequestIcons {
    fn default_open() -> String {
        "".to_string()
    }

    fn default_draft() -> String {
        "".to_string()
    }

    fn default_closed() -> String {
        "".to_string()
    }

    fn default_merged() -> String {
        "".to_string()
    }

    fn default_approved() -> String {
        "".to_string()
    }

    fn default_pipeline_pending() -> String {
        "".to_string()
    }

    fn default_pipeline_running() -> String {
        "".to_string()
    }

    fn default_pipeline_success() -> String {
        "".to_string()
    }

    fn default_pipeline_failed() -> String {
        "".to_string()
    }

    fn default_pipeline_canceled() -> String {
        "".to_string()
    }

    fn default_comment() -> String {
        " ".to_string()
    }
}

impl Default for MergeRequestIcons {
    fn default() -> Self {
        Self {
            open: Self::default_open(),
            draft: Self::default_draft(),
            closed: Self::default_closed(),
            merged: Self::default_merged(),
            pipeline_pending: Self::default_pipeline_pending(),
            pipeline_running: Self::default_pipeline_running(),
            pipeline_success: Self::default_pipeline_success(),
            pipeline_failed: Self::default_pipeline_failed(),
            pipeline_canceled: Self::default_pipeline_canceled(),
            approved: Self::default_approved(),
            comment: Self::default_comment(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenStateConfig {
    #[serde(default = "OpenStateConfig::default_style")]
    pub style: Style,
}

impl OpenStateConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::Black),
            background: Color::Fixed(214),
            decoration: vec![],
        }
    }
}

impl Default for OpenStateConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DraftStateConfig {
    #[serde(default = "DraftStateConfig::default_style")]
    pub style: Style,
}

impl DraftStateConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::Black),
            background: Color::Fixed(249),
            decoration: vec![],
        }
    }
}

impl Default for DraftStateConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ClosedStateConfig {
    #[serde(default = "ClosedStateConfig::default_style")]
    pub style: Style,
}

impl ClosedStateConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::Black),
            background: Color::Fixed(196),
            decoration: vec![],
        }
    }
}

impl Default for ClosedStateConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MergedStateConfig {
    #[serde(default = "MergedStateConfig::default_style")]
    pub style: Style,
}

impl MergedStateConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::Black),
            background: Color::Fixed(141),
            decoration: vec![],
        }
    }
}

impl Default for MergedStateConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}
