use crate::{command::SegmentArgs, config::Config, segment};

pub fn run(args: &SegmentArgs) {
    let config = &Config::default();
    segment::print_segments(config, args);
}
