//! Power profile handling.

use std::process::Command;
use anyhow::{Context, Result};

pub fn check_powerprofilesctl() -> Result<()> {
    Command::new("powerprofilesctl")
        .arg("--version")
        .output()
        .context("powerprofilesctl not found")?;
    Ok(())
}

pub fn get_current_profile() -> Result<Option<String>> {
    let output = Command::new("powerprofilesctl")
        .arg("get")
        .output()
        .context("Failed to execute powerprofilesctl")?;

    let profile = String::from_utf8(output.stdout)?
        .trim()
        .to_string();

    Ok(if profile.is_empty() { None } else { Some(profile) })
}

pub fn set_profile(profile: &str) -> Result<()> {
    Command::new("powerprofilesctl")
        .arg("set")
        .arg(profile)
        .output()
        .context("Failed to set power profile")?;
    Ok(())
}
