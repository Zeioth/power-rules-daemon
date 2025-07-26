//! Power Rules Daemon - Main Entry Point

use power_rules_daemon::{
    config::{load_config, get_config_path, DEFAULT_POLLING_INTERVAL, DEFAULT_PAUSE_MINUTES, DEFAULT_DEFAULT_PROFILE},
    power::{check_powerprofilesctl, get_current_profile, set_profile},
    process::is_process_running,
    state::{load_state, save_state, get_state_path},
};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time;
use anyhow::{Context, Result};
use chrono::Utc;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, Event};

fn main() -> Result<()> {
    // Verify powerprofilesctl is available
    check_powerprofilesctl().context("System compatibility check failed")?;

    // Load initial configuration
    let config_path = get_config_path().context("Could not determine config path")?;
    let mut config = load_config(&config_path).context("Failed to load configuration")?;

    // Create channel for config change notifications
    let (tx, rx) = mpsc::channel();

    // Create watcher thread
    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        notify::Config::default()
            .with_poll_interval(time::Duration::from_secs(1))  // Check every second
    ).context("Failed to create file watcher")?;

    watcher.watch(&config_path, RecursiveMode::NonRecursive)
        .context("Failed to watch config file")?;

    // Get initial settings
    let mut polling_interval = config
        .config
        .as_ref()
        .and_then(|c| c.polling_interval)
        .unwrap_or(DEFAULT_POLLING_INTERVAL);

    let mut pause_duration = config
        .config
        .as_ref()
        .and_then(|c| c.pause_on_manual_change)
        .unwrap_or(DEFAULT_PAUSE_MINUTES);

    let mut default_profile = config
        .config
        .as_ref()
        .and_then(|c| c.default_profile.as_ref())
        .map(|s| s.as_str())
        .unwrap_or(DEFAULT_DEFAULT_PROFILE);

    // Create initial process-to-profile mapping
    let mut rules: HashMap<String, String> = config
        .rule
        .iter()
        .map(|r| (r.name.clone(), r.profile.clone()))
        .collect();

    println!("Energy Rules Daemon initialized");
    println!("- Loaded {} rules", rules.len());
    println!("- Polling interval: {} seconds", polling_interval);
    println!("- Pause duration: {} minutes", pause_duration);
    println!("- Watching config file for changes: {}", config_path.display());

    // Initialize state
    let mut current_profile = get_current_profile()?;
    let state_path = get_state_path()?;
    let mut state = load_state(&state_path).unwrap_or_default();

    // Main control loop
    loop {
        // Check for config changes
        if let Ok(Ok(Event { kind: notify::EventKind::Modify(_), .. })) = rx.try_recv() {
            println!("Detected config file change, reloading...");
            match load_config(&config_path) {
                Ok(new_config) => {
                    config = new_config;

                    // Update settings
                    polling_interval = config
                        .config
                        .as_ref()
                        .and_then(|c| c.polling_interval)
                        .unwrap_or(DEFAULT_POLLING_INTERVAL);

                    pause_duration = config
                        .config
                        .as_ref()
                        .and_then(|c| c.pause_on_manual_change)
                        .unwrap_or(DEFAULT_PAUSE_MINUTES);

                    default_profile = config
                        .config
                        .as_ref()
                        .and_then(|c| c.default_profile.as_ref())
                        .map(|s| s.as_str())
                        .unwrap_or(DEFAULT_DEFAULT_PROFILE);

                    // Update rules
                    rules = config
                        .rule
                        .iter()
                        .map(|r| (r.name.clone(), r.profile.clone()))
                        .collect();

                    println!("Successfully reloaded configuration");
                    println!("- New polling interval: {} seconds", polling_interval);
                    println!("- New pause duration: {} minutes", pause_duration);
                    println!("- New rule count: {}", rules.len());
                }
                Err(e) => eprintln!("Error reloading config: {}", e),
            }
        }

        // Handle pause state
        if let Some(paused_until) = state.paused_until {
            if Utc::now().timestamp() < paused_until {
                thread::sleep(time::Duration::from_secs(polling_interval));
                continue;
            }
            // Pause expired
            state.paused_until = None;
            save_state(&state, &state_path)?;
            println!("Pause period ended, resuming normal operation");
        }

        // Detect manual profile changes
        let new_profile = get_current_profile()?;
        if new_profile != current_profile {
            println!(
                "Manual profile change detected ({} → {}). Pausing for {} minutes.",
                current_profile.as_deref().unwrap_or("default"),
                new_profile.as_deref().unwrap_or("default"),
                pause_duration
            );

            state.paused_until = Some(
                (Utc::now() + chrono::Duration::minutes(pause_duration as i64)).timestamp()
            );
            save_state(&state, &state_path)?;
            current_profile = new_profile;
            continue;
        }

        // Apply automatic profile rules
        if let Some(desired_profile) = get_desired_profile(&rules) {
            if Some(&desired_profile) != current_profile.as_ref() {
                println!(
                    "Applying profile '{}' (previous: '{}')",
                    desired_profile,
                    current_profile.as_deref().unwrap_or("default")
                );
                set_profile(&desired_profile)?;
                current_profile = Some(desired_profile);
            }
        } else if current_profile.as_deref() != Some(default_profile) {
            println!("No matching processes found, setting to default mode: {}", default_profile);
            set_profile(default_profile)?;
            current_profile = Some(default_profile.to_string());
        }

        thread::sleep(time::Duration::from_secs(polling_interval));
    }
}

/// Determine which profile should be active based on running processes
fn get_desired_profile(rules: &HashMap<String, String>) -> Option<String> {
    for (process_name, profile) in rules {
        if is_process_running(process_name) {
            println!("- Found matching process: {}", process_name);
            return Some(profile.clone());
        }
    }
    None
}
