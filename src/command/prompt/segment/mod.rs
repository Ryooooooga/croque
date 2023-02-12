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
    fn build(&self) -> Result<Option<Segment>, SegmentError>;
}

pub fn print_segments() {
    let path_builder = path::PathSegmentBuilder::default();

    let segments = &["os", "path"];
    for segment in segments {
        let seg = match *segment {
            "path" => path_builder.build(),
            _ => Ok(None),
        };

        if let Ok(Some(seg)) = seg {
            print!("[{}]", seg.content);
        }
    }
}
