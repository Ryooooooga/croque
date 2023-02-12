mod segment;

use crate::command::PromptArgs;

pub fn run(_args: &PromptArgs) {
    segment::print_segments();
    print!(" croque $ ");
}
