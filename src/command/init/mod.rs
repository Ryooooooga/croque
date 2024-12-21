// License: MIT
// Authors:
// - Ryooooooga <eial5q265e5@gmail.com>
// - Alex Mullen <alex@xela.foo>
use crate::{command::InitArgs, shell::Shell};

const INIT_BASH: &str = include_str!("init.bash");
const INIT_FISH: &str = include_str!("init.fish");
const INIT_ZSH: &str = include_str!("init.zsh");

pub fn run(args: &InitArgs) {
    match &args.shell {
        Shell::Bash => print!("{INIT_BASH}"),
        Shell::Fish => print!("{INIT_FISH}"),
        Shell::Zsh => print!("{INIT_ZSH}"),
    }
}
