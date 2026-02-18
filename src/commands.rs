use anyhow::Context;
use std::collections::HashSet;

use crate::git_util::{
    GitBranch, create_git_branch, get_current_git_branch, get_git_branches, git_branch_exists,
    git_checkout, git_rebase,
};
use crate::model::Branch;
use crate::state::StateCtx;
use crate::util::{execute_on_branch, get_target_branch, print_branch_tree};

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

pub fn untrack_branch(branch_name: Option<&str>, state: &mut StateCtx) -> anyhow::Result<()> {
    let target_branch_name = get_target_branch(branch_name).expect("Failed to get target branch");

    let target_branch = state
        .branches
        .iter()
        .find(|b| b.name == target_branch_name)
        .with_context(|| format!("Branch '{target_branch_name}' is not currently tracked"))?;

    if !target_branch.children.is_empty() {
        anyhow::bail!("Cannot untrack '{target_branch_name}', it has children");
    }

    state.modify(|s| {
        // Remove the branch itself
        s.branches.retain(|b| b.name != target_branch_name);

        // Remove any references to the branch
        for b in &mut s.branches {
            b.children.retain(|child| child != &target_branch_name);
        }
    });

    Ok(())
}

pub fn create_child_branch(child_branch_name: &str, state: &mut StateCtx) -> anyhow::Result<()> {
    let parent_branch_name = get_current_git_branch().expect("Failed to get current branch name");
    if git_branch_exists(child_branch_name).expect("Failed to check if branch exists") {
        anyhow::bail!(format!("{child_branch_name} already exists"))
    } else {
        create_git_branch(child_branch_name)
            .expect(&format!("Failed to create branch {child_branch_name}"));
    }

    state.modify(|s| {
        s.branches
            .iter_mut()
            .find(|b| b.name == parent_branch_name)
            .expect("Failed to find current branch in Stax state. Is the current branch tracked?")
            .children
            .push(child_branch_name.to_string());
    });

    track_branch(Some(&child_branch_name), Some(&parent_branch_name), state)
        .expect(&format!("Failed to track branch {child_branch_name}"));
    Ok(())
}

pub fn command_print_branch_tree(state: &StateCtx) -> anyhow::Result<()> {
    print_branch_tree(
        &state.branches,
        &get_current_git_branch().expect("Failed to get current git branch"),
    );
    Ok(())
}

pub fn rebase(onto: String, state: &mut StateCtx) -> anyhow::Result<()> {
    // TODO: Handle conflicts
    let current_branch = state
        .get_current_branch()
        .expect("Failed to read current branch from Stax state");

    git_rebase(
        &onto,
        current_branch.parent.as_deref().expect(&format!(
            "Failed to rebase onto {onto}; {} has no parent",
            current_branch.name
        )),
    )
    .expect(&format!(
        "Failed to rebase {} onto {}",
        current_branch.name, onto
    ));

    state.modify(|s| {
        s.get_current_branch_mut()
            .expect("Failed to read current branch from Stax state")
            .parent = Some(onto);
    });

    // TODO: Rebase children?

    Ok(())
}

pub fn checkout(branch_matchers: Vec<String>) -> anyhow::Result<()> {
    execute_on_branch(branch_matchers, |branch_name| git_checkout(branch_name))
}

pub fn prune(state: &mut StateCtx) -> anyhow::Result<()> {
    let git_branches = get_git_branches().expect("Failed to get git branches");
    let git_branch_names: Vec<&String> = git_branches.iter().map(|b| &b.ref_name).collect();

    let branches_to_remove = state
        .branches
        .iter()
        .filter(|b| !&git_branch_names.contains(&&b.name))
        .collect::<Vec<&Branch>>();

    if branches_to_remove.is_empty() {
        println!("Stax state is clean, no branches to prune");
        return Ok(());
    }

    let branch_names_to_remove: HashSet<String> =
        branches_to_remove.iter().map(|b| b.name.clone()).collect();

    state.modify(|s| {
        for name in &branch_names_to_remove {
            println!("Pruning {name}");
        }

        for tracked_branch in &mut s.branches {
            // Delete any parents that refer to now deleted branches
            if tracked_branch
                .parent
                .as_ref()
                .is_some_and(|parent_name| branch_names_to_remove.contains(parent_name))
            {
                tracked_branch.parent = None;
            }

            // Delete any children that refer to now deleted branches
            tracked_branch
                .children
                .retain(|child| !branch_names_to_remove.contains(child));
        }

        s.branches
            .retain(|b| !branch_names_to_remove.contains(&b.name));
    });

    Ok(())
}
