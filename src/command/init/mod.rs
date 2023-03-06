use crate::{command::InitArgs, shell::Shell};

const INIT_BASH: &str = include_str!("init.bash");
const INIT_FISH: &str = include_str!("init.fish");
const ASYNC_ZSH: &str = include_str!("../../../zsh-async/async.zsh");
const INIT_ZSH: &str = include_str!("init.zsh");

pub fn run(args: &InitArgs) {
    match &args.shell {
        Shell::Bash => print!("{INIT_BASH}"),
        Shell::Fish => print!("{INIT_FISH}"),
        Shell::Zsh => {
            if !args.without_async {
                println!("{ASYNC_ZSH}");
            }
            print!("{INIT_ZSH}");
        }
    }
}
