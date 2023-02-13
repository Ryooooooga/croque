mod duration;
mod os;
mod path;
mod presenter;
mod status;
mod time;
mod user;

use self::{
    os::OsSegmentBuilder, path::PathSegmentBuilder, presenter::Presenter,
    status::StatusSegmentBuilder, user::UserSegmentBuilder,
};
use crate::{
    command::SegmentArgs,
    config::{Config, SegmentKind},
};
use ansi_term::Style;

#[derive(Debug, PartialEq)]
pub struct Segment {
    pub content: String,
    pub style: Style,
}

#[derive(Debug)]
pub struct Context<'a> {
    config: &'a Config,
    args: &'a SegmentArgs,
}

impl<'a> Context<'a> {
    pub fn new(config: &'a Config, args: &'a SegmentArgs) -> Self {
        Self { config, args }
    }
}

pub trait SegmentBuilder {
    fn build(&self, ctx: &Context) -> Option<Segment>;
}

#[derive(Default)]
struct SegmentBuilders<'a> {
    os: OsSegmentBuilder,
    path: PathSegmentBuilder<'a>,
    status: StatusSegmentBuilder<'a>,
    user: UserSegmentBuilder<'a>,
}

pub fn print_segments(config: &Config, args: &SegmentArgs) {
    let builders = SegmentBuilders::default();

    let mut stdout = std::io::stdout();
    let mut presenter = Presenter::new(&mut stdout, config, &args.shell);

    let ctx = Context::new(config, args);

    for (row, line) in ctx.config.segments.iter().enumerate() {
        if row > 0 {
            presenter.next_line().unwrap();
        }

        let left_segments = line.left.iter().flat_map(|seg| match seg {
            SegmentKind::Duration => None,
            SegmentKind::Os => builders.os.build(&ctx),
            SegmentKind::Path => builders.path.build(&ctx),
            SegmentKind::Status => builders.status.build(&ctx),
            SegmentKind::Time => None,
            SegmentKind::User => builders.user.build(&ctx),
        });

        presenter.display_line(left_segments).unwrap();
    }
}
