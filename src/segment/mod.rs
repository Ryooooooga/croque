mod direnv;
mod duration;
mod gh_actions;
mod gh_pull_request;
mod git_status;
mod git_user;
mod glab_merge_request;
mod os;
mod path;
mod presenter;
mod status;
mod time;
mod user;

use self::{
    duration::DurationSegmentBuilder, gh_pull_request::GhPullRequestSegmentBuilder,
    git_status::GitStatusSegmentBuilder, git_user::GitUserSegmentBuilder,
    glab_merge_request::GlabMergeRequestSegmentBuilder, os::OsSegmentBuilder,
    path::PathSegmentBuilder, presenter::Presenter, status::StatusSegmentBuilder,
    time::TimeSegmentBuilder, user::UserSegmentBuilder,
};
use crate::{
    command::SegmentArgs,
    config::{Config, SegmentKind},
    info::{gh::GhInfo, git::GitInfo, glab::GlabInfo},
    segment::gh_actions::GhActionsSegmentBuilder,
};
use direnv::DirenvSegmentBuilder;
use nu_ansi_term::Style;
use std::io;

#[derive(Debug, PartialEq)]
pub struct Segment {
    pub content: String,
    pub style: Style,
}

#[derive(Debug)]
pub struct Context<'a> {
    config: &'a Config,
    args: &'a SegmentArgs,
    git_info: Option<&'a GitInfo>,
    gh_info: Option<&'a GhInfo>,
    glab_info: Option<&'a GlabInfo>,
}

impl<'a> Context<'a> {
    pub fn new(
        config: &'a Config,
        args: &'a SegmentArgs,
        git_info: Option<&'a GitInfo>,
        gh_info: Option<&'a GhInfo>,
        glab_info: Option<&'a GlabInfo>,
    ) -> Self {
        Self {
            config,
            args,
            git_info,
            gh_info,
            glab_info,
        }
    }
}

pub trait SegmentBuilder {
    fn build(&self, ctx: &Context) -> Option<Segment>;
}

#[derive(Default)]
struct SegmentBuilders<'a> {
    duration: DurationSegmentBuilder,
    os: OsSegmentBuilder,
    path: PathSegmentBuilder<'a>,
    status: StatusSegmentBuilder<'a>,
    time: TimeSegmentBuilder<'a>,
    user: UserSegmentBuilder<'a>,
    git_status: GitStatusSegmentBuilder,
    git_user: GitUserSegmentBuilder,
    gh_pull_request: GhPullRequestSegmentBuilder,
    gh_actions: GhActionsSegmentBuilder,
    glab_merge_request: GlabMergeRequestSegmentBuilder,
    direnv: DirenvSegmentBuilder,
}

impl SegmentBuilders<'_> {
    fn build_segment(&self, ctx: &Context, segment: &SegmentKind) -> Option<Segment> {
        match segment {
            SegmentKind::Duration => self.duration.build(ctx),
            SegmentKind::Os => self.os.build(ctx),
            SegmentKind::Path => self.path.build(ctx),
            SegmentKind::Status => self.status.build(ctx),
            SegmentKind::Time => self.time.build(ctx),
            SegmentKind::User => self.user.build(ctx),
            SegmentKind::GitStatus => self.git_status.build(ctx),
            SegmentKind::GitUser => self.git_user.build(ctx),
            SegmentKind::GhPullRequest => self.gh_pull_request.build(ctx),
            SegmentKind::GhActions => self.gh_actions.build(ctx),
            SegmentKind::GlabMergeRequest => self.glab_merge_request.build(ctx),
            SegmentKind::Direnv => self.direnv.build(ctx),
        }
    }

    fn build_segments(&self, ctx: &Context, segments: &[SegmentKind]) -> Vec<Segment> {
        segments
            .iter()
            .flat_map(|seg| self.build_segment(ctx, seg))
            .collect()
    }
}

pub fn print_segments(ctx: &Context) -> io::Result<()> {
    let builders = SegmentBuilders::default();

    let mut stdout = std::io::stdout().lock();
    let presenter = Presenter::new(ctx.config, &ctx.args.shell, ctx.args.width);

    let segments = &ctx.config.segments;
    if !ctx.args.right {
        for (row, line) in segments.iter().enumerate() {
            if row > 0 {
                presenter.next_line(&mut stdout)?;
            }

            let left = builders.build_segments(ctx, &line.left);
            let right = if row == segments.len() - 1 {
                vec![]
            } else {
                builders.build_segments(ctx, &line.right)
            };
            presenter.display_line(&mut stdout, &left, &right)?;
        }

        presenter.finish_left(&mut stdout)?;
    } else if let Some(last_line) = segments.last() {
        // right prompt
        let right = builders.build_segments(ctx, &last_line.right);
        presenter.display_right(&mut stdout, &right)?;
    }

    Ok(())
}
