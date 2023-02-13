mod os;
mod path;
mod status;
mod style;
mod user;

use self::{os::OsConfig, path::PathConfig, status::StatusConfig, user::UserConfig};
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
    pub segment_separators: SegmentSeparators,

    #[serde(default = "Config::default_segments")]
    pub segments: Vec<Line>,
}

impl Config {
    fn default_segments() -> Vec<Line> {
        vec![
            Line {
                left: vec![SegmentKind::Os, SegmentKind::User, SegmentKind::Path],
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
            os: OsConfig::default(),
            user: UserConfig::default(),
            path: PathConfig::default(),
            status: StatusConfig::default(),
            segment_separators: SegmentSeparators::default(),
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
}
