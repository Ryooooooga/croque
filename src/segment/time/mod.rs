use super::{Context, Segment, SegmentBuilder};
use chrono::{DateTime, Local, Utc};

pub struct TimeSegmentBuilder<'a> {
    utc_now: &'a dyn Fn() -> DateTime<Utc>,
    local_now: &'a dyn Fn() -> DateTime<Local>,
}

impl Default for TimeSegmentBuilder<'_> {
    fn default() -> Self {
        Self {
            utc_now: &Utc::now,
            local_now: &Local::now,
        }
    }
}

impl SegmentBuilder for TimeSegmentBuilder<'_> {
    fn build(&self, ctx: &Context) -> Option<Segment> {
        let config = &ctx.config.time;

        let format = &config.format;
        let time = if config.utc {
            (self.utc_now)().format(format).to_string()
        } else {
            (self.local_now)().format(format).to_string()
        };

        let content = config.content.replace("{{.time}}", &time);
        let style = config.style.to_ansi();

        Some(Segment { content, style })
    }
}
