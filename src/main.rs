mod command;
mod config;
mod info;
mod segment;
mod shell;

use clap::Parser;
use command::{Command, run};

fn main() {
    let cmd = Command::parse();
    run(&cmd);
}
