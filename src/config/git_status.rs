use super::style::{Color, NamedColor, Style};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitStatusConfig {
    #[serde(default)]
    pub icons: GitStatusIcons,

    #[serde(default)]
    pub clean: CleanStatusConfig,

    #[serde(default)]
    pub unstaged: UnstagedStatusConfig,

    #[serde(default)]
    pub staged: StagedStatusConfig,

    #[serde(default)]
    pub conflicted: ConflictedStatusConfig,

    #[serde(default = "GitStatusConfig::default_remotes")]
    pub remotes: Vec<RemoteConfig>,

    #[serde(default = "GitStatusConfig::default_display_master")]
    pub display_master: bool,

    #[serde(default = "GitStatusConfig::default_commit_hash_length")]
    pub commit_hash_length: usize,

    #[serde(default = "GitStatusConfig::default_content")]
    pub content: String,
}

impl GitStatusConfig {
    fn default_remotes() -> Vec<RemoteConfig> {
        vec![
            RemoteConfig {
                pattern: "github.com".to_string(),
                icon: " ".to_string(),
            },
            RemoteConfig {
                pattern: "".to_string(),
                icon: " ".to_string(),
            },
        ]
    }

    fn default_display_master() -> bool {
        true
    }

    fn default_commit_hash_length() -> usize {
        7
    }

    fn default_content() -> String {
        " {{.remote}}{{.head}}{{.working_tree}}{{.upstream}} ".to_string()
    }
}

impl Default for GitStatusConfig {
    fn default() -> Self {
        Self {
            icons: Default::default(),
            clean: Default::default(),
            unstaged: Default::default(),
            staged: Default::default(),
            conflicted: Default::default(),
            remotes: Self::default_remotes(),
            display_master: Self::default_display_master(),
            commit_hash_length: Self::default_commit_hash_length(),
            content: Self::default_content(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GitStatusIcons {
    #[serde(default = "GitStatusIcons::default_branch")]
    pub branch: String,

    #[serde(default = "GitStatusIcons::default_tag")]
    pub tag: String,

    #[serde(default = "GitStatusIcons::default_commit")]
    pub commit: String,

    #[serde(default = "GitStatusIcons::default_modified")]
    pub modified: String,

    #[serde(default = "GitStatusIcons::default_added")]
    pub added: String,

    #[serde(default = "GitStatusIcons::default_deleted")]
    pub deleted: String,

    #[serde(default = "GitStatusIcons::default_renamed")]
    pub renamed: String,

    #[serde(default = "GitStatusIcons::default_conflicted")]
    pub conflicted: String,

    #[serde(default = "GitStatusIcons::default_behind")]
    pub behind: String,

    #[serde(default = "GitStatusIcons::default_ahead")]
    pub ahead: String,
}

impl GitStatusIcons {
    fn default_branch() -> String {
        "".to_string()
    }
    fn default_tag() -> String {
        "".to_string()
    }
    fn default_commit() -> String {
        "".to_string()
    }
    fn default_modified() -> String {
        "…".to_string()
    }
    fn default_added() -> String {
        "+".to_string()
    }
    fn default_deleted() -> String {
        "-".to_string()
    }
    fn default_renamed() -> String {
        "→".to_string()
    }
    fn default_conflicted() -> String {
        "".to_string()
    }
    fn default_behind() -> String {
        "".to_string()
    }
    fn default_ahead() -> String {
        "".to_string()
    }
}

impl Default for GitStatusIcons {
    fn default() -> Self {
        Self {
            branch: Self::default_branch(),
            tag: Self::default_tag(),
            commit: Self::default_commit(),
            modified: Self::default_modified(),
            added: Self::default_added(),
            deleted: Self::default_deleted(),
            renamed: Self::default_renamed(),
            conflicted: Self::default_conflicted(),
            behind: Self::default_behind(),
            ahead: Self::default_ahead(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CleanStatusConfig {
    #[serde(default = "CleanStatusConfig::default_style")]
    pub style: Style,
}

impl CleanStatusConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::Black),
            background: Color::Named(NamedColor::Green),
            decoration: vec![],
        }
    }
}

impl Default for CleanStatusConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UnstagedStatusConfig {
    #[serde(default = "UnstagedStatusConfig::default_style")]
    pub style: Style,
}

impl UnstagedStatusConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::Black),
            background: Color::Named(NamedColor::Yellow),
            decoration: vec![],
        }
    }
}

impl Default for UnstagedStatusConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StagedStatusConfig {
    #[serde(default = "StagedStatusConfig::default_style")]
    pub style: Style,
}

impl StagedStatusConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::Black),
            background: Color::Named(NamedColor::Green),
            decoration: vec![],
        }
    }
}

impl Default for StagedStatusConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ConflictedStatusConfig {
    #[serde(default = "ConflictedStatusConfig::default_style")]
    pub style: Style,
}

impl ConflictedStatusConfig {
    fn default_style() -> Style {
        Style {
            foreground: Color::Named(NamedColor::Black),
            background: Color::Named(NamedColor::Red),
            decoration: vec![],
        }
    }
}

impl Default for ConflictedStatusConfig {
    fn default() -> Self {
        Self {
            style: Self::default_style(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RemoteConfig {
    pub pattern: String,
    pub icon: String,
}
