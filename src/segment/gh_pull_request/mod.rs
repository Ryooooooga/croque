use super::{Context, Segment, SegmentBuilder};
use crate::{
    config::{gh_pull_request::GhPullRequestConfig, style::Style},
    info::gh::{PullRequest, PullRequestState},
};
use aho_corasick::AhoCorasick;

#[derive(Debug)]
pub struct GhPullRequestSegmentBuilder {
    replacer: AhoCorasick,
}

impl GhPullRequestSegmentBuilder {
    pub fn new() -> Self {
        let replacer = AhoCorasick::new(["{{.number}}", "{{.state}}", "{{.comments}}"]);
        Self { replacer }
    }

    fn build_number(&self, _config: &GhPullRequestConfig, pr: &PullRequest) -> String {
        pr.number.to_string()
    }

    fn build_state(&self, config: &GhPullRequestConfig, pr: &PullRequest) -> Option<String> {
        let icon: &str = match (&pr.state, pr.is_draft) {
            (PullRequestState::Open, false) => &config.icons.open,
            (PullRequestState::Open, true) => &config.icons.draft,
            (PullRequestState::Closed, _) => &config.icons.closed,
            (PullRequestState::Merged, _) => &config.icons.merged,
        };
        Some(icon)
            .filter(|icon| !icon.is_empty())
            .map(|icon| format!(" {icon}"))
    }

    fn build_comments(&self, config: &GhPullRequestConfig, pr: &PullRequest) -> Option<String> {
        if pr.comments > 0 {
            Some(format!(" {}{}", config.icons.comment, pr.comments))
        } else {
            None
        }
    }

    fn style<'a>(&self, config: &'a GhPullRequestConfig, pr: &PullRequest) -> &'a Style {
        match (&pr.state, pr.is_draft) {
            (PullRequestState::Open, false) => &config.open.style,
            (PullRequestState::Open, true) => &config.draft.style,
            (PullRequestState::Closed, _) => &config.closed.style,
            (PullRequestState::Merged, _) => &config.merged.style,
        }
    }
}

impl Default for GhPullRequestSegmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SegmentBuilder for GhPullRequestSegmentBuilder {
    fn build(&self, ctx: &Context) -> Option<Segment> {
        let config = &ctx.config.gh_pull_request;
        let pr = &ctx.gh_info?.pull_request;

        let number = self.build_number(config, pr);
        let state = self.build_state(config, pr).unwrap_or_default();
        let comments = self.build_comments(config, pr).unwrap_or_default();

        let content = self.replacer.replace_all(
            &config.content,
            &[number.as_str(), state.as_ref(), comments.as_ref()],
        );

        let style = self.style(config, pr).to_ansi();

        Some(Segment { content, style })
    }
}
