use crate::config::Config;

mod duration;
mod os;
mod path;
mod status;
mod time;
mod user;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum SegmentError {}

#[derive(Debug, PartialEq)]
pub struct Segment {
    pub content: String,
}

pub trait SegmentBuilder {
    fn build(&self, config: &Config) -> Result<Option<Segment>, SegmentError>;
}

pub fn print_segments(config: &Config) {
    let os_builder = os::OsSegmentBuilder::default();
    let path_builder = path::PathSegmentBuilder::default();
    let user_builder = user::UserSegmentBuilder::default();

    let segments = ["os", "user", "path"];
    for segment in segments {
        let seg = match segment {
            "os" => os_builder.build(config),
            "path" => path_builder.build(config),
            "user" => user_builder.build(config),
            _ => Ok(None),
        };

        if let Ok(Some(seg)) = seg {
            print!("[{}]", seg.content);
        }
    }
}
