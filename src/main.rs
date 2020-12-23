use structopt::StructOpt;

mod commands;
mod utils;

use commands::{cd, clone};

#[derive(StructOpt)]
enum Command {
    Clone { repository: String },
    Cd { path: String },
}

fn main() {
    match Command::from_args() {
        Command::Clone { repository } => clone(repository),
        Command::Cd { path } => cd(path),
    }
}
