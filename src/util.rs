use crate::git_util::{GitBranch, get_current_git_branch, get_git_branches};
use crate::model::Branch;

use dialoguer::FuzzySelect;
use dialoguer::theme::ColorfulTheme;
use owo_colors::OwoColorize;
use std::collections::{HashMap, HashSet};

pub fn get_target_branch(branch_name: Option<&str>) -> anyhow::Result<String> {
    match branch_name {
        None => get_current_git_branch(),
        Some(branch_name) => Ok(branch_name.to_string()),
    }
}

pub fn print_branch_tree(branches: &[Branch], current_branch: &str) {
    let by_parent: HashMap<&str, Vec<&str>> = branches.iter().fold(HashMap::new(), |mut m, b| {
        m.entry(b.parent.as_deref().unwrap_or(""))
            .or_default()
            .push(b.name.as_str());
        m
    });

    let mut roots = by_parent.get("").cloned().unwrap_or_default();
    roots.sort_unstable();

    let mut visiting = HashSet::new();
    for (i, &r) in roots.iter().enumerate() {
        print_node_line(r, current_branch, None);

        print_subtree(
            r,
            &by_parent,
            &mut visiting,
            &mut Vec::new(),
            current_branch,
            i + 1 == roots.len(),
        );
    }
}

fn print_node_line(
    name: &str,
    current_branch: &str,
    edge: Option<bool>, // Some(true)=last(└─), Some(false)=mid(├─), None=root
) {
    // Make current branch stand out without neon text
    let styled_name: String = if name == current_branch {
        format!("{}", name.bold().green())
    } else {
        format!("{}", name)
    };

    let edge_str: String = match edge {
        None => "".to_string(),
        Some(true) => format!("{} ", "└─".bright_black()),
        Some(false) => format!("{} ", "├─".bright_black()),
    };

    // Grey supporting text
    let marker: String = if name == current_branch {
        format!(" {}", "← you are here".bright_black())
    } else {
        "".to_string()
    };

    println!("{edge_str}{styled_name}{marker}");
}

fn print_subtree<'a>(
    node: &'a str,
    by_parent: &HashMap<&'a str, Vec<&'a str>>,
    visiting: &mut HashSet<&'a str>,
    last_stack: &mut Vec<bool>,
    current_branch: &str,
    _is_last_root: bool,
) {
    if !visiting.insert(node) {
        // Print cycle notice in red
        // prefix from ancestors
        for &is_last in last_stack.iter() {
            print!("{}", if is_last { "   " } else { "│  " }.bright_black());
        }
        println!("{}", "(cycle detected)".bright_red().bold());
        return;
    }

    let mut children = match by_parent.get(node) {
        Some(k) => k.clone(),
        None => {
            visiting.remove(node);
            return;
        }
    };
    children.sort_unstable();

    for (idx, child) in children.iter().enumerate() {
        let last = idx + 1 == children.len();

        // prefix from ancestors (colored)
        for &is_last in last_stack.iter() {
            print!("{}", if is_last { "   " } else { "│  " }.bright_black());
        }

        // Print this child line
        print_node_line(child, current_branch, Some(last));

        last_stack.push(last);
        print_subtree(
            child,
            by_parent,
            visiting,
            last_stack,
            current_branch,
            false,
        );
        last_stack.pop();
    }

    visiting.remove(node);
}

pub fn execute_on_branch<F>(branch_matchers: Vec<String>, f: F) -> anyhow::Result<()>
where
    F: FnOnce(&String) -> anyhow::Result<()>,
{
    let git_branches = get_git_branches().expect("Failed to get git branches");

    // Find branches that match all `branch_matchers`
    let matches: Vec<GitBranch> = git_branches
        .into_iter()
        .filter(|b| {
            branch_matchers
                .iter()
                .all(|matcher| b.ref_name.contains(matcher))
        })
        .collect();

    if matches.is_empty() {
        anyhow::bail!(format!(
            "No branches matched the patterns {:?}",
            branch_matchers,
        ));
    }

    let selected_branch = if matches.len() > 1 {
        let idx = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Matched multiple branches. Type to filter, Enter to select")
            .items(matches.clone())
            .default(0)
            .interact()?;

        &matches[idx]
    } else {
        matches.first().expect("Failed to get first matched branch")
    };

    f(&selected_branch.ref_name)
}
