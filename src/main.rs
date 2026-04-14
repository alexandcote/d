use clap::Parser;

mod commands;
mod utils;

use commands::{cd, clone};

#[derive(Parser)]
enum Command {
    Clone { repository: String },
    Cd { path: String },
}

fn main() {
    match Command::parse() {
        Command::Clone { repository } => clone(repository),
        Command::Cd { path } => cd(path),
    }
}
