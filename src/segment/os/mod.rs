use crate::config::Config;

use super::{Segment, SegmentBuilder, SegmentError};

#[derive(Debug, Default)]
pub struct OsSegmentBuilder {}

impl SegmentBuilder for OsSegmentBuilder {
    fn build(&self, config: &Config) -> Result<Option<Segment>, SegmentError> {
        #[cfg(target_os = "linux")]
        let os = "linux";

        #[cfg(target_os = "macos")]
        let os = "mac";

        #[cfg(target_os = "windows")]
        let os = "windows";

        Ok(Some(Segment {
            content: os.to_string(),
        }))
    }
}
