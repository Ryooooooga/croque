use super::{Context, Segment, SegmentBuilder};

#[derive(Debug, Default)]
pub struct GitUserSegmentBuilder {}

impl SegmentBuilder for GitUserSegmentBuilder {
    fn build(&self, ctx: &Context) -> Option<Segment> {
        let config = &ctx.config.git_user;

        let user_name = ctx.git_info?.user.as_ref()?;

        let content = config.content.replace("{{.name}}", user_name);
        let style = config.style.to_ansi();

        Some(Segment { content, style })
    }
}
