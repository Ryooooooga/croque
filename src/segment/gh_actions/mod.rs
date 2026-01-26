use crate::{
    config::{gh_actions::GhActionsConfig, style::Style},
    info::gh::{Actions, ActionsStatus},
};

use super::{Context, Segment, SegmentBuilder};
use aho_corasick::AhoCorasick;

#[derive(Debug)]
pub struct GhActionsSegmentBuilder {
    replacer: AhoCorasick,
}

impl GhActionsSegmentBuilder {
    pub fn new() -> Self {
        let replacer = AhoCorasick::new(["{{.status}}"]).unwrap();
        Self { replacer }
    }

    fn icon<'a>(&self, config: &'a GhActionsConfig, actions: &Actions) -> &'a str {
        match &actions.status {
            ActionsStatus::InProgress => &config.icons.in_progress,
            ActionsStatus::Success => &config.icons.success,
            ActionsStatus::Failure => &config.icons.failure,
            ActionsStatus::Cancelled => &config.icons.cancelled,
            ActionsStatus::Skipped => &config.icons.skipped,
        }
    }

    fn style<'a>(&self, config: &'a GhActionsConfig, actions: &Actions) -> &'a Style {
        match &actions.status {
            ActionsStatus::InProgress => &config.in_progress.style,
            ActionsStatus::Success => &config.success.style,
            ActionsStatus::Failure => &config.failure.style,
            ActionsStatus::Cancelled => &config.cancelled.style,
            ActionsStatus::Skipped => &config.skipped.style,
        }
    }
}

impl Default for GhActionsSegmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SegmentBuilder for GhActionsSegmentBuilder {
    fn build(&self, ctx: &Context) -> Option<Segment> {
        let config = &ctx.config.gh_actions;
        let actions = ctx.gh_info?.actions.as_ref()?;

        let icon = self.icon(config, actions);

        let content = self.replacer.replace_all(&config.content, &[icon]);
        let style = self.style(config, actions).to_ansi();

        Some(Segment { content, style })
    }
}
