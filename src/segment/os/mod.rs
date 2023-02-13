use crate::config::Config;

use super::{Segment, SegmentBuilder};

#[derive(Debug, Default)]
pub struct OsSegmentBuilder {}

impl SegmentBuilder for OsSegmentBuilder {
    fn build(&self, config: &Config) -> Option<Segment> {
        #[cfg(target_os = "linux")]
        let config = &config.os.linux;

        #[cfg(target_os = "macos")]
        let config = &config.os.mac;

        #[cfg(target_os = "windows")]
        let config = &config.os.windows;

        Some(Segment {
            content: config.content.clone(),
            style: config.style.to_ansi(),
        })
    }
}
