//! Module declarations.

pub mod config;
pub mod state;
pub mod power;
pub mod process;

// Re-export important types at crate root
pub use config::{Config, ConfigSection, Rule};
pub use state::DaemonState;
