use super::{Context, Segment, SegmentBuilder};
use crate::{
    config::{glab_merge_request::GlabMergeRequestConfig, style::Style},
    info::glab::{MergeRequest, MergeRequestState, MrPipelineState},
};
use aho_corasick::AhoCorasick;

#[derive(Debug)]
pub struct GlabMergeRequestSegmentBuilder {
    replacer: AhoCorasick,
}

impl GlabMergeRequestSegmentBuilder {
    pub fn new() -> Self {
        let replacer = AhoCorasick::new([
            "{{.number}}",
            "{{.state}}",
            "{{.pipeline}}",
            "{{.approved}}",
            "{{.comments}}",
        ])
        .unwrap();
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

    fn build_pipeline(&self, config: &GlabMergeRequestConfig, mr: &MergeRequest) -> Option<String> {
        match mr.pipeline {
            MrPipelineState::None => None,
            MrPipelineState::Pending => Some(format!(" {}", config.icons.pipeline_pending)),
            MrPipelineState::Running => Some(format!(" {}", config.icons.pipeline_running)),
            MrPipelineState::Success => Some(format!(" {}", config.icons.pipeline_success)),
            MrPipelineState::Failed => Some(format!(" {}", config.icons.pipeline_failed)),
            MrPipelineState::Canceled => Some(format!(" {}", config.icons.pipeline_canceled)),
        }
    }

    fn build_approved(&self, config: &GlabMergeRequestConfig, mr: &MergeRequest) -> Option<String> {
        if mr.is_approved {
            Some(format!(" {}", config.icons.approved))
        } else {
            None
        }
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
        let state = self.build_state(config, mr);
        let pipeline = self.build_pipeline(config, mr);
        let approved = self.build_approved(config, mr);
        let comments = self.build_comments(config, mr);

        let content = self.replacer.replace_all(
            &config.content,
            &[
                number.as_str(),
                state.as_deref().unwrap_or_default(),
                pipeline.as_deref().unwrap_or_default(),
                approved.as_deref().unwrap_or_default(),
                comments.as_deref().unwrap_or_default(),
            ],
        );

        let style = self.style(config, mr).to_ansi();

        Some(Segment { content, style })
    }
}
