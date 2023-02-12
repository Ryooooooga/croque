use crate::{command::InitArgs, shell::Shell};

const INIT_BASH: &str = include_str!("init.bash");
const INIT_FISH: &str = include_str!("init.fish");
const INIT_ZSH: &str = include_str!("init.zsh");

fn init_script(shell: &Shell) -> &'static str {
    match shell {
        Shell::Bash => INIT_BASH,
        Shell::Fish => INIT_FISH,
        Shell::Zsh => INIT_ZSH,
    }
}

pub fn run(args: &InitArgs) {
    print!("{}", init_script(&args.shell));
}
