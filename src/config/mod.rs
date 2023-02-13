mod os;
mod path;
mod style;
mod user;

use self::{os::OsConfig, path::PathConfig, user::UserConfig};
use serde::Deserialize;
use std::default::Default;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub os: OsConfig,

    #[serde(default)]
    pub user: UserConfig,

    #[serde(default)]
    pub path: PathConfig,

    #[serde(default)]
    pub segment_separators: SegmentSeparators,
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
