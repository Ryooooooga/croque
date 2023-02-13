use crate::{command::PromptArgs, config::Config, segment};

pub fn run(_args: &PromptArgs) {
    let config = &Config::default();
    segment::print_segments(config);
    print!(" croque $ ");
}
