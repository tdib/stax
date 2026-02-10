use crate::model::State;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::ops::Deref;

const STATE_FILE: &str = ".stax.json";

pub struct StateCtx {
    #[allow(dead_code)]
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

        save_state(&self.current_state).expect("Failed to save state to disk");
    }
}

fn save_state(state: &State) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(state)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(STATE_FILE)?;

    file.write_all(json.as_bytes())?;
    file.sync_all()?;

    Ok(())
}
