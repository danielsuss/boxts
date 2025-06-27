use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoxtsConfig {
    pub window: WindowConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WindowConfig {
    pub position: String,
}

impl Default for BoxtsConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig {
                position: "topleft".to_string(),
            },
        }
    }
}

fn get_config_path() -> PathBuf {
    // Get directory where exe is located, put config there
    std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("./boxts.exe"))
        .parent()
        .unwrap_or(&PathBuf::from("."))
        .join("boxts.conf.toml")
}

pub fn load_config() -> Result<BoxtsConfig, confy::ConfyError> {
    confy::load_path(get_config_path())
}

pub fn save_config(config: &BoxtsConfig) -> Result<(), confy::ConfyError> {
    confy::store_path(get_config_path(), config)
}