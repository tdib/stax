use std::process::Command;

#[derive(Debug, Clone)]
pub struct GitBranch {
    pub ref_name: String,
    directory: Option<String>, // TODO: Figure out directories for worktree support
}

impl ToString for GitBranch {
    fn to_string(&self) -> String {
        self.ref_name.clone()
    }
}

pub fn get_current_git_branch() -> anyhow::Result<String> {
    let out = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;

    if !out.status.success() {
        anyhow::bail!("git error: {}", String::from_utf8_lossy(&out.stderr));
    }

    Ok(String::from_utf8(out.stdout)?.trim().to_string())
}

pub fn create_git_branch(branch_name: &str) -> anyhow::Result<()> {
    let out = Command::new("git")
        .args(["checkout", "-b", branch_name])
        .output()?;

    if !out.status.success() {
        anyhow::bail!("git error: {}", String::from_utf8_lossy(&out.stderr));
    }

    Ok(())
}

pub fn git_branch_exists(branch_name: &str) -> anyhow::Result<bool> {
    let out = Command::new("git")
        .args(["rev-parse", "--verify", "--quiet", branch_name])
        .output()?;

    Ok(!out.stdout.is_empty())
}

pub fn get_git_branches() -> anyhow::Result<Vec<GitBranch>> {
    let out = Command::new("git")
        .args(["for-each-ref", "--format=%(refname:short)", "refs/heads/"])
        .output()?;

    let out_string = String::from_utf8(out.stdout)?;
    let git_branches: Vec<&str> = out_string.lines().collect();

    if !out.status.success() {
        anyhow::bail!("git error: {}", String::from_utf8_lossy(&out.stderr));
    }

    Ok(git_branches
        .into_iter()
        .map(|b| GitBranch {
            ref_name: b.to_string(),
            directory: None,
        })
        .collect())
}

pub fn git_checkout(branch_name: &str) -> anyhow::Result<()> {
    // TODO: Add support for checkout to worktrees
    println!("Checking out branch '{branch_name}'");
    let out = Command::new("git")
        .args(["checkout", branch_name])
        .output()?;

    if !out.status.success() {
        anyhow::bail!("git error: {}", String::from_utf8_lossy(&out.stderr));
    }

    Ok(())
}

pub fn git_rebase(onto: &str, since: &str) -> anyhow::Result<()> {
    println!(
        "Rebasing current branch onto {} (all commits since {})",
        onto, since
    );
    let out = Command::new("git")
        .args(["rebase", "--onto", onto, since])
        .output()?;

    if !out.status.success() {
        anyhow::bail!("git error: {}", String::from_utf8_lossy(&out.stderr));
    }

    Ok(())
}
