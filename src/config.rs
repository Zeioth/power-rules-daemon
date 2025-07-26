//! Configuration handling with real-time reload support.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;
use anyhow::{Context, Result};
use toml::from_str;

pub const DEFAULT_POLLING_INTERVAL: u64 = 5;
pub const DEFAULT_PAUSE_MINUTES: u64 = 60;
pub const DEFAULT_DEFAULT_PROFILE: &str = "balanced";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub config: Option<ConfigSection>,
    pub rule: Vec<Rule>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigSection {
    pub polling_interval: Option<u64>,
    pub pause_on_manual_change: Option<u64>,
    pub default_profile: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    pub name: String,
    pub profile: String,
}

/// Configuration watcher for real-time reloading
#[derive(Debug)]
pub struct ConfigWatcher {
    path: PathBuf,
    last_modified: Option<SystemTime>,
}

impl ConfigWatcher {
    /// Creates a new watcher for the given config file path
    pub fn new(path: PathBuf) -> Result<Self> {
        let last_modified = if path.exists() {
            Some(std::fs::metadata(&path)?.modified()?)
        } else {
            None
        };
        Ok(Self { path, last_modified })
    }

    /// Checks if the config file has been modified since last check
    pub fn has_changed(&mut self) -> Result<bool> {
        let current_modified = if self.path.exists() {
            Some(std::fs::metadata(&self.path)?.modified()?)
        } else {
            None
        };

        let changed = current_modified != self.last_modified;
        self.last_modified = current_modified;
        Ok(changed)
    }
}

/// Validates the default power profile string.
fn validate_default_profile(profile: &str) -> bool {
    matches!(
        profile.to_lowercase().as_str(),
        "performance" | "balanced" | "power-saver" | "power_saver"
    )
}

/// Gets the default config file path
pub fn get_config_path() -> Result<PathBuf> {
    Ok(dirs::home_dir()
        .context("Could not find home directory")?
        .join(".config/power-rules/config.toml"))
}

/// Loads and validates the configuration
pub fn load_config(config_path: &PathBuf) -> Result<Config> {
    if !config_path.exists() {
        return Ok(Config {
            config: None,
            rule: Vec::new(),
        });
    }

    let config_data = std::fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

    let config: Config = from_str(&config_data).context("Failed to parse TOML configuration")?;

    // Validate default_profile if specified
    if let Some(ConfigSection { default_profile: Some(profile), .. }) = &config.config {
        if !validate_default_profile(profile) {
            return Err(anyhow::anyhow!(
                "Invalid default_profile '{}' in config. Must be one of: performance, balanced, power-saver",
                profile
            ));
        }
    }

    // Validate all rule profiles
    for rule in &config.rule {
        if !validate_default_profile(&rule.profile) {
            return Err(anyhow::anyhow!(
                "Invalid profile '{}' for rule '{}'. Must be one of: performance, balanced, power-saver",
                rule.profile,
                rule.name
            ));
        }
    }

    Ok(config)
}

/// Helper to convert config rules to a hashmap
pub fn build_rule_map(config: &Config) -> HashMap<String, String> {
    config.rule.iter().map(|r| (r.name.clone(), r.profile.clone())).collect()
}
