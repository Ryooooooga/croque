use super::{Context, Segment, SegmentBuilder};
use crate::{
    config::{glab_merge_request::GlabMergeRequestConfig, style::Style},
    info::glab::{MergeRequest, MergeRequestState},
};
use aho_corasick::AhoCorasick;

#[derive(Debug)]
pub struct GlabMergeRequestSegmentBuilder {
    replacer: AhoCorasick,
}

impl GlabMergeRequestSegmentBuilder {
    pub fn new() -> Self {
        let replacer = AhoCorasick::new(["{{.number}}", "{{.state}}", "{{.comments}}"]).unwrap();
        Self { replacer }
    }

    fn build_number(&self, _config: &GlabMergeRequestConfig, mr: &MergeRequest) -> String {
        mr.number.to_string()
    }

    fn build_state(&self, config: &GlabMergeRequestConfig, mr: &MergeRequest) -> Option<String> {
        let icon: &str = match (&mr.state, mr.is_draft) {
            (MergeRequestState::Open, false) => &config.icons.open,
            (MergeRequestState::Open, true) => &config.icons.draft,
            (MergeRequestState::Closed, _) => &config.icons.closed,
            (MergeRequestState::Merged, _) => &config.icons.merged,
        };
        Some(icon)
            .filter(|icon| !icon.is_empty())
            .map(|icon| format!(" {icon}"))
    }

    fn build_comments(&self, config: &GlabMergeRequestConfig, mr: &MergeRequest) -> Option<String> {
        if mr.comments > 0 {
            Some(format!(" {}{}", config.icons.comment, mr.comments))
        } else {
            None
        }
    }

    fn style<'a>(&self, config: &'a GlabMergeRequestConfig, mr: &MergeRequest) -> &'a Style {
        match (&mr.state, mr.is_draft) {
            (MergeRequestState::Open, false) => &config.open.style,
            (MergeRequestState::Open, true) => &config.draft.style,
            (MergeRequestState::Closed, _) => &config.closed.style,
            (MergeRequestState::Merged, _) => &config.merged.style,
        }
    }
}

impl Default for GlabMergeRequestSegmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SegmentBuilder for GlabMergeRequestSegmentBuilder {
    fn build(&self, ctx: &Context) -> Option<Segment> {
        let config = &ctx.config.glab_merge_request;
        let mr = &ctx.glab_info?.merge_request;

        let number = self.build_number(config, mr);
        let state = self.build_state(config, mr).unwrap_or_default();
        let comments = self.build_comments(config, mr).unwrap_or_default();

        let content = self.replacer.replace_all(
            &config.content,
            &[number.as_str(), state.as_ref(), comments.as_ref()],
        );

        let style = self.style(config, mr).to_ansi();

        Some(Segment { content, style })
    }
}
