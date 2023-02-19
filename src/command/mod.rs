mod config;
mod init;
mod prepare;
mod prompt;

use crate::shell::Shell;
use std::fmt;

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

    #[command(about = "Serialize info for lazy segments")]
    Prepare(PrepareArgs),

    #[command(about = "Prints the configuration presets")]
    Config(ConfigArgs),
}

#[derive(Debug, clap::Args)]
pub struct InitArgs {
    #[arg()]
    pub shell: Shell,
}

#[derive(Debug, clap::Args)]
pub struct SegmentArgs {
    #[arg(short, long, help = "Prints the right prompt", default_value_t = false)]
    pub right: bool,

    #[arg(short = 's', long, help = "The status code of previously run command")]
    pub exit_status: i32,

    #[arg(short, long, help = "The execution duration of the last command")]
    pub duration: f64,

    #[arg(short, long, help = "The number of currently running jobs")]
    pub jobs: i32,

    #[arg(short, long, help = "The width of terminal")]
    pub width: usize,

    #[arg(long = "data.git")]
    pub encoded_git_info: Option<String>,

    #[arg(long = "data.gh")]
    pub encoded_gh_info: Option<String>,

    #[arg()]
    pub shell: Shell,
}

#[derive(Debug, clap::Args)]
pub struct PrepareArgs {
    #[arg()]
    pub source: DataSource,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum DataSource {
    Git,
    Gh,
}

#[derive(Debug, clap::Args)]
pub struct ConfigArgs {
    #[arg(short, long, help = "Theme name of the config", default_value_t = ConfigTheme::Agnoster)]
    pub theme: ConfigTheme,

    #[arg(short, long, help = "Prints the config path", default_value_t = false)]
    pub print_config_path: bool,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ConfigTheme {
    Agnoster,
}

impl fmt::Display for ConfigTheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Agnoster => write!(f, "agnoster"),
        }
    }
}

pub fn run(cmd: &Command) {
    match &cmd.subcommand {
        Subcommand::Init(args) => init::run(args),
        Subcommand::Prompt(args) => prompt::run(args),
        Subcommand::Prepare(args) => prepare::run(args),
        Subcommand::Config(args) => config::run(args),
    };
}
