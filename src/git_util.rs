use std::process::Command;

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
