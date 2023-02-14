use crate::{
    command::SegmentArgs,
    config::Config,
    info::git::load_git_info,
    segment::{self, Context},
};

pub fn run(args: &SegmentArgs) {
    let config = &Config::default();
    let git_info = load_git_info();
    let ctx = Context::new(config, args, git_info.as_ref());

    segment::print_segments(&ctx).unwrap();
}
