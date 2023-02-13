use super::{Context, Segment, SegmentBuilder};

#[derive(Debug, Default)]
pub struct OsSegmentBuilder {}

impl SegmentBuilder for OsSegmentBuilder {
    fn build(&self, ctx: &Context) -> Option<Segment> {
        #[cfg(target_os = "linux")]
        let config = &ctx.config.os.linux;

        #[cfg(target_os = "macos")]
        let config = &ctx.config.os.mac;

        #[cfg(target_os = "windows")]
        let config = &ctx.config.os.windows;

        Some(Segment {
            content: config.content.clone(),
            style: config.style.to_ansi(),
        })
    }
}
