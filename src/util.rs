use std::process::Command;

pub fn get_target_branch(branch_name: Option<&str>) -> anyhow::Result<String> {
    match branch_name {
        None => get_current_branch(),
        Some(branch_name) => Ok(branch_name.to_string()),
    }
}

pub fn get_current_branch() -> anyhow::Result<String> {
    let out = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;

    if !out.status.success() {
        anyhow::bail!("git error: {}", String::from_utf8_lossy(&out.stderr));
    }

    Ok(String::from_utf8(out.stdout)?.trim().to_string())
}
