mod commands;
mod git_util;
mod model;
mod state;
mod util;

use clap::{Parser, Subcommand};
use commands::{create_child_branch, track_branch, untrack_branch};

use crate::state::StateCtx;

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
    Create { branch_name: String },
}

fn main() {
    // TODO: Ensure git is installed
    let cli = Cli::parse();
    let mut state = StateCtx::load().expect("Failed to load Stax state");

    let result = match cli.cmd {
        Cmd::Track { branch_name } => track_branch(branch_name.as_deref(), None, &mut state), // TODO: Fix None
        Cmd::Untrack { branch_name } => untrack_branch(branch_name.as_deref()),
        Cmd::Create { branch_name } => create_child_branch(&branch_name, &mut state),
    };

    match result {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e)
        }
    }
}
