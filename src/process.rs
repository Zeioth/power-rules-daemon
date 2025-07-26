//! Process detection

use std::process::Command;

pub fn is_process_running(process_name: &str) -> bool {
    Command::new("pgrep")
        .arg("-f")
        .arg(process_name)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
