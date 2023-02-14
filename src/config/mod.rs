pub mod git_status;
pub mod git_user;
pub mod os;
pub mod path;
pub mod status;
pub mod style;
pub mod user;

use self::{
    git_status::GitStatusConfig, git_user::GitUserConfig, os::OsConfig, path::PathConfig,
    status::StatusConfig, user::UserConfig,
};
use serde::Deserialize;
use std::default::Default;

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
    pub git_status: GitStatusConfig,

    #[serde(default)]
    pub git_user: GitUserConfig,

    #[serde(default)]
    pub segment_separators: SegmentSeparators,

    #[serde(default = "Config::default_segments")]
    pub segments: Vec<Line>,
}

impl Config {
    fn default_segments() -> Vec<Line> {
        vec![
            Line {
                left: vec![
                    SegmentKind::Os,
                    SegmentKind::User,
                    SegmentKind::Path,
                    SegmentKind::GitStatus,
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
            git_status: Default::default(),
            git_user: Default::default(),
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
}
