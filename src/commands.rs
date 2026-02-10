use crate::git_util::{create_git_branch, get_current_git_branch};
use crate::model::Branch;
use crate::state::StateCtx;
use crate::util::get_target_branch;

pub fn track_branch(
    branch_name: Option<&str>,
    parent_branch_name: Option<&str>,
    state: &mut StateCtx,
) -> anyhow::Result<()> {
    let target_branch_name = get_target_branch(branch_name).expect("Failed to get target branch");

    if !state.branches.iter().any(|b| b.name == target_branch_name) {
        match &parent_branch_name {
            Some(parent) => {
                println!("Setting up tracking for '{target_branch_name}' with parent '{parent}'")
            }
            None => println!("Setting up tracking for '{target_branch_name}' with no parent"),
        }
        state.modify(|s| {
            s.branches.push(Branch {
                name: target_branch_name.clone(),
                parent: parent_branch_name.map(|s| s.to_string()),
                children: Vec::new(),
            })
        });
    } else {
        println!("Branch '{target_branch_name}' already tracked");
    }

    Ok(())
}

pub fn untrack_branch(branch_name: Option<&str>) -> anyhow::Result<()> {
    match branch_name {
        None => {
            println!("No branch passed in to untrack... Untracking current");
            let curr_branch = get_current_git_branch();

            match curr_branch {
                Ok(curr_branch) => {
                    println!("Current branch: {}", curr_branch)
                }
                Err(e) => {
                    println!("Error: {}", e)
                }
            };
        }
        Some(branch_name) => {
            println!("Passed branch name: {}", branch_name);
        }
    };
    Ok(())
}

pub fn create_child_branch(child_branch_name: &str, state: &mut StateCtx) -> anyhow::Result<()> {
    let parent_branch_name = get_current_git_branch().expect("Failed to get current branch name");
    create_git_branch(child_branch_name).expect("Failed to create branch {branch_name}");

    state.modify(|s| {
        s.branches
            .iter_mut()
            .find(|b| b.name == parent_branch_name)
            .expect("Failed to find current branch in Stax state. Is the current branch tracked?")
            .children
            .push(child_branch_name.to_string());
    });

    track_branch(Some(&child_branch_name), Some(&parent_branch_name), state)
        .expect("Failed to track branch {branch_name}");
    Ok(())
}
