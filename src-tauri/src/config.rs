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
    pub monitor_id: u32,
    pub lost_focus_behaviour: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TTSConfig {
    pub output_device: String,
    pub volume: f32,
    pub voice: String,
}

impl Default for BoxtsConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig {
                position: "topleft".to_string(),
                monitor_id: 0,
                lost_focus_behaviour: "hide".to_string(),
            },
            tts: TTSConfig {
                output_device: "Default".to_string(),
                volume: 0.5,
                voice: "Default".to_string(),
            },
        }
    }
}

fn get_config_path() -> PathBuf {
    if cfg!(debug_assertions) {
        PathBuf::from("../boxts.conf.toml")
    } else {
        std::env::current_exe()
            .unwrap_or_else(|_| PathBuf::from("./boxts.exe"))
            .parent()
            .unwrap_or(&PathBuf::from("."))
            .join("boxts.conf.toml")
    }
}

pub fn load_config() -> Result<BoxtsConfig, confy::ConfyError> {
    confy::load_path(get_config_path())
}

pub fn save_config(config: &BoxtsConfig) -> Result<(), confy::ConfyError> {
    confy::store_path(get_config_path(), config)
}

pub async fn apply_ui_config(app: tauri::AppHandle, state: State<'_, crate::AppState>) -> Result<WindowConfig, Box<dyn std::error::Error>> {
    let config = load_config()?;
    // Switch to the saved monitor first
    let (_switch_result, new_monitor_id) = crate::utils::switch_to_monitor(app.clone(), config.window.monitor_id).await?;
    let _ = set_monitor_id(&state, new_monitor_id);
    // Then apply the saved position
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

pub fn set_monitor_id(state: &State<crate::AppState>, monitor_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = state.config.lock().unwrap();
    config.window.monitor_id = monitor_id;
    save_config(&config).map_err(|e| format!("Failed to save config: {}", e))?;
    Ok(())
}


pub fn get_output_device(state: &State<crate::AppState>) -> String {
    let config = state.config.lock().unwrap();
    config.tts.output_device.clone()
}

pub fn set_output_device(state: &State<crate::AppState>, device_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = state.config.lock().unwrap();
    config.tts.output_device = device_name.to_string();
    save_config(&config).map_err(|e| format!("Failed to save config: {}", e))?;
    Ok(())
}

pub fn get_volume(state: &State<crate::AppState>) -> f32 {
    let config = state.config.lock().unwrap();
    config.tts.volume
}

pub fn set_volume(state: &State<crate::AppState>, volume: f32) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = state.config.lock().unwrap();
    config.tts.volume = volume;
    save_config(&config).map_err(|e| format!("Failed to save config: {}", e))?;
    Ok(())
}

pub fn get_voice(state: &State<crate::AppState>) -> Result<String, Box<dyn std::error::Error>> {
    let config = state.config.lock().unwrap();
    Ok(config.tts.voice.clone())
}

pub fn set_voice(state: &State<crate::AppState>, voice: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = state.config.lock().unwrap();
    config.tts.voice = voice.to_string();
    save_config(&config).map_err(|e| format!("Failed to save config: {}", e))?;
    Ok(())
}

pub async fn apply_config(app: tauri::AppHandle, state: State<'_, crate::AppState>) -> Result<(), Box<dyn std::error::Error>> {
    apply_ui_config(app.clone(), state.clone()).await?;
    apply_tts_config()?;
    Ok(())
}

pub fn get_lostfocus_behaviour(state: &State<crate::AppState>) -> String {
    let config = state.config.lock().unwrap();
    config.window.lost_focus_behaviour.clone()
}

pub fn set_lostfocus_behaviour(state: &State<crate::AppState>, behaviour: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = state.config.lock().unwrap();
    config.window.lost_focus_behaviour = behaviour.to_string();
    save_config(&config).map_err(|e| format!("Failed to save config: {}", e))?;
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