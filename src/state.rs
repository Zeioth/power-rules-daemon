//! State management

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::{Context, Result};
use toml::{from_str, to_string};

const STATE_FILE_PATH: &str = ".local/state/power-rules/state.toml";

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DaemonState {
    pub paused_until: Option<i64>,
}

pub fn load_state(state_path: &PathBuf) -> Result<DaemonState> {
    if !state_path.exists() {
        return Ok(DaemonState::default());
    }

    let data = std::fs::read_to_string(state_path)
        .context("Failed to read state file")?;
    from_str(&data).context("Failed to parse state file")
}

pub fn save_state(state: &DaemonState, state_path: &PathBuf) -> Result<()> {
    if let Some(parent) = state_path.parent() {
        std::fs::create_dir_all(parent).context("Failed to create config directory")?;
    }

    let data = to_string(state).context("Failed to serialize state")?;
    std::fs::write(state_path, data).context("Failed to write state file")?;
    Ok(())
}

pub fn get_state_path() -> Result<PathBuf> {
    Ok(dirs::home_dir()
        .context("Could not find home directory")?
        .join(STATE_FILE_PATH))
}
