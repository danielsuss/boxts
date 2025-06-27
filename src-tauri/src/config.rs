use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoxtsConfig {
    pub window: WindowConfig,
    pub tts: TTSConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WindowConfig {
    pub position: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TTSConfig {
}

impl Default for BoxtsConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig {
                position: "topleft".to_string(),
            },
            tts: TTSConfig {
            },
        }
    }
}

fn get_config_path() -> PathBuf {
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

pub async fn apply_ui_config(app: tauri::AppHandle, state: State<'_, crate::AppState>) -> Result<WindowConfig, Box<dyn std::error::Error>> {
    let config = load_config()?;
    crate::utils::apply_window_position(app, state, &config.window.position).await?;
    Ok(config.window)
}

pub fn apply_tts_config() -> Result<TTSConfig, confy::ConfyError> {
    let config = load_config()?;
    Ok(config.tts)
}

pub fn get_window_position(state: &State<crate::AppState>) -> String {
    let config = state.config.lock().unwrap();
    config.window.position.clone()
}

pub fn set_window_position(state: &State<crate::AppState>, position: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = state.config.lock().unwrap();
    config.window.position = position.to_string();
    save_config(&config).map_err(|e| format!("Failed to save config: {}", e))?;
    Ok(())
}

pub async fn apply_config(app: tauri::AppHandle, state: State<'_, crate::AppState>) -> Result<(), Box<dyn std::error::Error>> {
    apply_ui_config(app.clone(), state.clone()).await?;
    apply_tts_config()?;
    Ok(())
}

pub async fn reset_config(app: tauri::AppHandle, state: State<'_, crate::AppState>) -> Result<(), Box<dyn std::error::Error>> {
    let default_config = BoxtsConfig::default();

    {
        let mut config = state.config.lock().unwrap();
        *config = default_config.clone();
        save_config(&config).map_err(|e| format!("Failed to save config: {}", e))?;
    }
    
    apply_config(app, state).await?;
    
    Ok(())
}