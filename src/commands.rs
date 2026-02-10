use crate::git_util::{create_git_branch, get_current_git_branch};
use crate::model::Branch;
use crate::state::{load_state, load_tracked_branches, save_state};
use crate::util::get_target_branch;

pub fn track_branch(
    branch_name: Option<&str>,
    parent_branch_name: Option<&str>,
) -> anyhow::Result<()> {
    println!("Tracking branch with parent {:?}", &branch_name);
    let mut tracked = load_tracked_branches().expect("Failed to load tracked branches");
    let target_branch_name = get_target_branch(branch_name).expect("Failed to get target branch");

    if !tracked
        .branches
        .iter()
        .any(|b| b.name == target_branch_name)
    {
        match &parent_branch_name {
            Some(parent) => {
                println!("Setting up tracking for '{target_branch_name}' with parent '{parent}'")
            }
            None => println!("Setting up tracking for '{target_branch_name}' with no parent"),
        }
        tracked.branches.push(Branch {
            name: target_branch_name.clone(),
            parent: parent_branch_name.map(|s| s.to_string()),
            children: Vec::new(),
        });
    } else {
        println!("Branch '{target_branch_name}' already tracked");
    }

    if let Err(e) = save_state(&tracked) {
        println!("Failed to save state: {e}");
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

pub fn create_child_branch(child_branch_name: &str) -> anyhow::Result<()> {
    let parent_branch_name = get_current_git_branch().expect("Failed to get current branch name");
    create_git_branch(child_branch_name).expect("Failed to create branch {branch_name}");
    let mut state = load_state().expect("Failed to load state");

    let curr_branch = state
        .branches
        .iter_mut()
        .find(|branch| branch.name == parent_branch_name)
        .expect("Failed to find current branch in Stax state. Is the current branch tracked?");
    curr_branch.children.push(child_branch_name.to_string());
    save_state(&state).expect("Failed to save state");

    track_branch(Some(&child_branch_name), Some(&parent_branch_name))
        .expect("Failed to track branch {branch_name}");
    Ok(())
}
