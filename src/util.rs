use crate::git_util::get_current_git_branch;

pub fn get_target_branch(branch_name: Option<&str>) -> anyhow::Result<String> {
    match branch_name {
        None => get_current_git_branch(),
        Some(branch_name) => Ok(branch_name.to_string()),
    }
}
