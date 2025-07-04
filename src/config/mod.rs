//! Configuration management for the Logswise CLI.

pub mod manager;
pub mod setup_modes;
pub mod templates;
pub mod updater;
pub mod validator;

pub use manager::ConfigManager;

// Public utility functions
use dirs::home_dir;

pub fn config_exists() -> bool {
    let mut path = home_dir().unwrap_or_default();
    path.push(".logswise/setup.json");
    path.exists()
}
