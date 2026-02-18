mod commands;
mod git_util;
mod model;
mod state;
mod util;

use clap::{Parser, Subcommand};
use commands::{
    checkout, command_print_branch_tree, create_child_branch, prune, track_branch, untrack_branch,
};

use crate::{commands::rebase, state::StateCtx};

#[derive(Parser)]
#[command(name = "stax")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    #[command(aliases = ["ck"])]
    Checkout {
        branch_name: Vec<String>,
    },
    Prune,
    Rebase {
        onto: String,
    },
    Stack {
        branch_name: String,
    },
    Track {
        branch_name: Option<String>,
    },
    Tree,
    Untrack {
        branch_name: Option<String>,
    },
}

fn main() {
    // TODO: Ensure git is installed
    let cli = Cli::parse();
    let mut state = StateCtx::load().expect("Failed to load Stax state");

    let result = match cli.cmd {
        Cmd::Checkout { branch_name } => checkout(branch_name, &state),
        Cmd::Prune => prune(&mut state),
        Cmd::Rebase { onto } => rebase(onto, &mut state),
        Cmd::Stack { branch_name } => create_child_branch(&branch_name, &mut state),
        Cmd::Track { branch_name } => track_branch(branch_name.as_deref(), None, &mut state), // TODO: Fix None
        Cmd::Tree => command_print_branch_tree(&state),
        Cmd::Untrack { branch_name } => untrack_branch(branch_name.as_deref(), &mut state),
    };

    match result {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e)
        }
    }
}
