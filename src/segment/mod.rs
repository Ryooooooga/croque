use self::{os::OsSegmentBuilder, path::PathSegmentBuilder, user::UserSegmentBuilder};
use crate::config::Config;
use ansi_term::Style;

mod duration;
mod os;
mod path;
mod status;
mod time;
mod user;

#[derive(Debug, PartialEq)]
pub struct Segment {
    pub content: String,
    pub style: Style,
}

pub trait SegmentBuilder {
    fn build(&self, config: &Config) -> Option<Segment>;
}

#[derive(Default)]
struct SegmentBuilders<'a> {
    os: OsSegmentBuilder,
    path: PathSegmentBuilder<'a>,
    user: UserSegmentBuilder<'a>,
}

pub fn print_segments(config: &Config) {
    let builders = SegmentBuilders::default();

    let segments = ["os", "user", "path"];
    for segment in segments {
        let seg = match segment {
            "os" => builders.os.build(config),
            "path" => builders.path.build(config),
            "user" => builders.user.build(config),
            _ => None,
        };

        if let Some(seg) = seg {
            print!("[{}]", seg.content);
        }
    }
}
