mod commands;
mod model;
mod state;
mod util;

use clap::{Parser, Subcommand};
use commands::{track_branch, untrack_branch};

#[derive(Parser)]
#[command(name = "stax")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Track { branch_name: Option<String> },
    Untrack { branch_name: Option<String> },
}

fn main() {
    // TODO: Ensure git is installed
    let cli = Cli::parse();

    let result = match cli.cmd {
        Cmd::Track { branch_name } => track_branch(branch_name.as_deref()),
        Cmd::Untrack { branch_name } => untrack_branch(branch_name.as_deref()),
    };

    match result {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e)
        }
    }
}
