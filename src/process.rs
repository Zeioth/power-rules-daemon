//! Process detection using procfs (no pgrep dependency)

use procfs::process::all_processes;
use anyhow::{Context, Result};

/// Checks if a process is running by searching command lines (like pgrep -f)
pub fn is_process_running(process_name: &str) -> bool {
    match try_is_process_running(process_name) {
        Ok(running) => running,
        Err(e) => {
            eprintln!("Process check error: {}", e);
            false
        }
    }
}

/// Inner function that returns Result for proper error handling
fn try_is_process_running(process_name: &str) -> Result<bool> {
    let processes = all_processes().context("Failed to list processes")?;

    for process in processes {
        let process = process.context("Failed to read process info")?;

        // Check the process name (from stat)
        if let Ok(stat) = process.stat() {
            if stat.comm.contains(process_name) {
                return Ok(true);
            }
        }

        // Check full command line
        if let Ok(cmdline) = process.cmdline() {
            if cmdline.iter().any(|arg| arg.contains(process_name)) {
                return Ok(true);
            }
        }
    }

    Ok(false)
}
