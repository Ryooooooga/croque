pub mod duration;
pub mod gh_pull_request;
pub mod git_status;
pub mod git_user;
pub mod os;
pub mod path;
pub mod status;
pub mod style;
pub mod time;
pub mod user;

use self::{
    duration::DurationConfig, gh_pull_request::GhPullRequestConfig, git_status::GitStatusConfig,
    git_user::GitUserConfig, os::OsConfig, path::PathConfig, status::StatusConfig,
    time::TimeConfig, user::UserConfig,
};
use serde::Deserialize;
use std::{
    default::Default,
    fs::File,
    path::{Path, PathBuf},
};

const CROQUE_CONFIG_FILE: &str = "CROQUE_CONFIG_FILE";
const XDG_CONFIG_HOME: &str = "XDG_CONFIG_HOME";

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub os: OsConfig,

    #[serde(default)]
    pub user: UserConfig,

    #[serde(default)]
    pub path: PathConfig,

    #[serde(default)]
    pub status: StatusConfig,

    #[serde(default)]
    pub duration: DurationConfig,

    #[serde(default)]
    pub time: TimeConfig,

    #[serde(default)]
    pub git_status: GitStatusConfig,

    #[serde(default)]
    pub git_user: GitUserConfig,

    #[serde(default)]
    pub gh_pull_request: GhPullRequestConfig,

    #[serde(default)]
    pub segment_separators: SegmentSeparators,

    #[serde(default = "Config::default_segments")]
    pub segments: Vec<Line>,
}

impl Config {
    pub fn load_or_default<P: AsRef<Path>>(config_path: P) -> Config {
        let config_path = config_path.as_ref();

        if let Ok(file) = File::open(config_path) {
            match Self::load_from_file(&file) {
                Ok(config) => return config,
                Err(err) => eprintln!(
                    "croque: failed to load config file `{}`: {}",
                    config_path.to_string_lossy(),
                    err,
                ),
            }
        }

        Self::default()
    }

    fn load_from_file(file: &File) -> Result<Config, serde_yaml::Error> {
        serde_yaml::from_reader(file)
    }

    pub fn config_path() -> PathBuf {
        if let Some(path) = std::env::var_os(CROQUE_CONFIG_FILE) {
            PathBuf::from(path)
        } else if let Some(xdg_config_home) = std::env::var_os(XDG_CONFIG_HOME) {
            let mut path = PathBuf::from(xdg_config_home);
            path.push("croque/config.yaml");
            path
        } else if let Some(home_dir) = dirs::home_dir() {
            let mut path = home_dir;
            path.push(".config/croque/config.yaml");
            path
        } else {
            PathBuf::from("/etc/croque/config.yaml")
        }
    }

    fn default_segments() -> Vec<Line> {
        vec![
            Line {
                left: vec![
                    SegmentKind::Os,
                    SegmentKind::User,
                    SegmentKind::Path,
                    SegmentKind::GitStatus,
                    SegmentKind::GhPullRequest,
                    SegmentKind::GitUser,
                ],
                right: vec![SegmentKind::Time],
            },
            Line {
                left: vec![SegmentKind::Status, SegmentKind::Duration],
                right: vec![],
            },
        ]
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            os: Default::default(),
            user: Default::default(),
            path: Default::default(),
            status: Default::default(),
            duration: Default::default(),
            time: Default::default(),
            git_status: Default::default(),
            git_user: Default::default(),
            gh_pull_request: Default::default(),
            segment_separators: Default::default(),
            segments: Self::default_segments(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SegmentSeparators {
    #[serde(default = "SegmentSeparators::default_solid_left")]
    pub solid_left: String,

    #[serde(default = "SegmentSeparators::default_wire_left")]
    pub wire_left: String,

    #[serde(default = "SegmentSeparators::default_solid_right")]
    pub solid_right: String,

    #[serde(default = "SegmentSeparators::default_wire_right")]
    pub wire_right: String,
}

impl Default for SegmentSeparators {
    fn default() -> Self {
        Self {
            solid_left: Self::default_solid_left(),
            wire_left: Self::default_wire_left(),
            solid_right: Self::default_solid_right(),
            wire_right: Self::default_wire_right(),
        }
    }
}

impl SegmentSeparators {
    fn default_solid_left() -> String {
        "".to_string()
    }

    fn default_wire_left() -> String {
        "".to_string()
    }

    fn default_solid_right() -> String {
        "".to_string()
    }

    fn default_wire_right() -> String {
        "".to_string()
    }
}

#[derive(Debug, Deserialize)]
pub struct Line {
    #[serde(default)]
    pub left: Vec<SegmentKind>,

    #[serde(default)]
    pub right: Vec<SegmentKind>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentKind {
    Duration,
    Os,
    Path,
    Status,
    Time,
    User,
    GitStatus,
    GitUser,
    GhPullRequest,
}
