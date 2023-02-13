mod command;
mod segment;
mod shell;

use clap::Parser;
use command::{run, Command};

fn main() {
    let cmd = Command::parse();
    run(&cmd);
}
