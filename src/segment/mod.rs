mod duration;
mod git_status;
mod git_user;
mod os;
mod path;
mod presenter;
mod status;
mod time;
mod user;

use self::{
    duration::DurationSegmentBuilder, git_status::GitStatusSegmentBuilder,
    git_user::GitUserSegmentBuilder, os::OsSegmentBuilder, path::PathSegmentBuilder,
    presenter::Presenter, status::StatusSegmentBuilder, time::TimeSegmentBuilder,
    user::UserSegmentBuilder,
};
use crate::{
    command::SegmentArgs,
    config::{Config, SegmentKind},
    info::git::GitInfo,
};
use ansi_term::Style;
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
}

impl<'a> Context<'a> {
    pub fn new(config: &'a Config, args: &'a SegmentArgs, git_info: Option<&'a GitInfo>) -> Self {
        Self {
            config,
            args,
            git_info,
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
}

pub fn print_segments(ctx: &Context) -> io::Result<()> {
    let builders = SegmentBuilders::default();

    let mut stdout = std::io::stdout();
    let mut presenter = Presenter::new(&mut stdout, ctx.config, &ctx.args.shell);

    for (row, line) in ctx.config.segments.iter().enumerate() {
        if row > 0 {
            presenter.next_line()?;
        }

        let left_segments = line.left.iter().flat_map(|seg| match seg {
            SegmentKind::Duration => builders.duration.build(ctx),
            SegmentKind::Os => builders.os.build(ctx),
            SegmentKind::Path => builders.path.build(ctx),
            SegmentKind::Status => builders.status.build(ctx),
            SegmentKind::Time => builders.time.build(ctx),
            SegmentKind::User => builders.user.build(ctx),
            SegmentKind::GitStatus => builders.git_status.build(ctx),
            SegmentKind::GitUser => builders.git_user.build(ctx),
        });

        presenter.display_line(left_segments)?;
    }

    presenter.finish()?;
    Ok(())
}
