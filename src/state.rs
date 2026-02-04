use crate::model::State;
use std::fs;
use std::path::Path;

const STATE_FILE: &str = ".stax.json";

pub fn load_tracked_branches() -> anyhow::Result<State> {
    let path = Path::new(STATE_FILE);

    if !path.exists() {
        return Ok(State { branches: vec![] });
    }

    let data = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data)?)
}

pub fn load_state() -> anyhow::Result<State> {
    if let Ok(data) = fs::read_to_string(STATE_FILE) {
        Ok(serde_json::from_str(&data)?)
    } else {
        Ok(State::default())
    }
}

pub fn save_state(state: &State) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(state)?;
    fs::write(STATE_FILE, json)?;
    Ok(())
}
