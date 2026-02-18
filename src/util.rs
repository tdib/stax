use crate::git_util::{GitBranch, get_current_git_branch, get_git_branches};
use crate::model::Branch;
use std::collections::{HashMap, HashSet};

pub fn get_target_branch(branch_name: Option<&str>) -> anyhow::Result<String> {
    match branch_name {
        None => get_current_git_branch(),
        Some(branch_name) => Ok(branch_name.to_string()),
    }
}

pub fn print_branch_tree(branches: &[Branch]) {
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
        println!("{r}");
        print_subtree(
            r,
            &by_parent,
            &mut visiting,
            &mut Vec::new(),
            i + 1 == roots.len(),
        );
    }
}

fn print_subtree<'a>(
    node: &'a str,
    by_parent: &HashMap<&'a str, Vec<&'a str>>,
    visiting: &mut HashSet<&'a str>,
    last_stack: &mut Vec<bool>,
    _is_last_root: bool,
) {
    if !visiting.insert(node) {
        println!("(cycle detected)");
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

        // prefix from ancestors
        for &is_last in last_stack.iter() {
            print!("{}", if is_last { "   " } else { "│  " });
        }
        println!(
            "{}",
            if last {
                format!("└─ {child}")
            } else {
                format!("├─ {child}")
            }
        );

        last_stack.push(last);
        print_subtree(child, by_parent, visiting, last_stack, false);
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

    if matches.len() > 1 {
        // TODO: If branch matches directly, checkout?
        println!("Matched multiple branches:");
        matches.iter().for_each(|m| println!("  {}", m.ref_name));
        anyhow::bail!("Matched multiple branches")
        // TODO: Implement multiple branch match selection
    }

    f(&matches
        .first()
        .expect("Failed to get first matched branch")
        .ref_name)
}
