use crate::git_util::get_current_git_branch;
use crate::model::{Branch, State};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::Deref;

const STATE_FILE: &str = ".stax.json";

pub struct StateCtx {
    file: File,
    current_state: State,
}

impl Deref for StateCtx {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.current_state
    }
}

impl StateCtx {
    pub fn load() -> anyhow::Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(STATE_FILE)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let state = if contents.is_empty() {
            State::default()
        } else {
            serde_json::from_str(&contents)?
        };

        Ok(StateCtx {
            file,
            current_state: state,
        })
    }

    /// Modify state, ensuring that it's written back to disk.
    pub fn modify(&mut self, f: impl FnOnce(&mut State)) {
        f(&mut self.current_state);

        self.save_state().expect("Failed to save state to disk");
    }

    fn save_state(&mut self) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(&self.current_state)?;

        self.file.set_len(0)?;
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(json.as_bytes())?;
        self.file.sync_all()?;

        Ok(())
    }
}

impl State {
    pub fn get_current_branch(&self) -> Option<&Branch> {
        let current_git_branch_name =
            get_current_git_branch().expect("Failed to get current git branch");

        if let Some(current_branch) = self
            .branches
            .iter()
            .find(|b| b.name == current_git_branch_name)
        {
            Some(current_branch)
        } else {
            None
        }
    }

    pub fn get_current_branch_mut(&mut self) -> Option<&mut Branch> {
        let name = get_current_git_branch().ok()?;
        self.branches.iter_mut().find(|b| b.name == name)
    }
}
