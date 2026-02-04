use crate::model::Branch;
use crate::state::{load_tracked_branches, save_state};
use crate::util::{get_current_branch, get_target_branch};

pub fn track_branch(branch_name: Option<&str>) -> anyhow::Result<()> {
    let mut tracked = load_tracked_branches().expect("Failed to load tracked branches");

    let current_branch = get_current_branch().expect("Failed to get current branch");
    let target_branch_name = get_target_branch(branch_name).expect("Failed to get target branch");
    let parent = if branch_name.is_some() && branch_name.unwrap() != current_branch {
        Some(current_branch)
    } else {
        None
    };

    if !tracked
        .branches
        .iter()
        .any(|b| b.name == target_branch_name)
    {
        match &parent {
            Some(parent) => {
                println!("Setting up tracking for '{target_branch_name}' with parent '{parent}'")
            }
            None => println!("Setting up tracking for '{target_branch_name}' with no parent"),
        }
        tracked.branches.push(Branch {
            name: target_branch_name.clone(),
            parent: parent,
        });
    } else {
        println!("Branch '{target_branch_name}' already tracked");
    }

    match save_state(&tracked) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to save state: {e}")
        }
    }

    Ok(())
}

pub fn untrack_branch(branch_name: Option<&str>) -> anyhow::Result<()> {
    match branch_name {
        None => {
            println!("No branch passed in to untrack... Untracking current");
            let curr_branch = get_current_branch();

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
