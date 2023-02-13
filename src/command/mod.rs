mod init;
mod prompt;

use crate::shell::Shell;

#[derive(Debug, clap::Parser)]
#[command(version, disable_version_flag = true, author, about)]
pub struct Command {
    #[command(subcommand)]
    pub subcommand: Subcommand,

    #[arg(short, long, help = "Print version information", action=clap::ArgAction::Version)]
    version: Option<bool>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    #[command(about = "Prints the initialization scripts")]
    Init(InitArgs),

    #[command(about = "Prints the prompt")]
    Prompt(SegmentArgs),
}

#[derive(Debug, clap::Args)]
pub struct InitArgs {
    #[arg()]
    pub shell: Shell,
}

#[derive(Debug, clap::Args)]
pub struct SegmentArgs {
    #[arg(short = 's', long, help = "The status code of previously run command")]
    pub exit_status: i32,

    #[arg(short, long, help = "The execution duration of the last command")]
    pub duration: f64,

    #[arg(short, long, help = "The number of currently running jobs")]
    pub jobs: i32,

    #[arg(short, long, help = "The width of terminal")]
    pub width: usize,

    #[arg()]
    pub shell: Shell,
}

pub fn run(cmd: &Command) {
    match &cmd.subcommand {
        Subcommand::Init(args) => init::run(args),
        Subcommand::Prompt(args) => prompt::run(args),
    };
}
