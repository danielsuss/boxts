use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
    State,
};
use std::sync::Mutex;
use std::process::Child;

mod bridge;
mod commands;
mod config;
mod initial_setup;
mod log;
mod server_utils;
mod utils;

struct AppState {
    config: Mutex<config::BoxtsConfig>,
    dialog_active: Mutex<bool>,
    server_process: Mutex<Option<Child>>,
}

const AVAILABLE_COMMANDS: &[&str] = &["center", "exit", "nextmonitor", "topleft", "topright", "bottomleft", "bottomright", "resetconfig", "outputdevice", "volume", "clonevoice", "restartserver", "start", "listdevices", "stop", "changevoice"];

#[tauri::command]
fn get_available_commands() -> Vec<String> {
    AVAILABLE_COMMANDS.iter().map(|s| s.to_string()).collect()
}

#[tauri::command]
fn get_output_devices() -> Vec<String> {
    use cpal::traits::{HostTrait, DeviceTrait};
    
    let host = cpal::default_host();
    match host.output_devices() {
        Ok(devices) => {
            devices
                .filter_map(|device| device.name().ok())
                .collect()
        }
        Err(_) => vec!["No output devices found".to_string()]
    }
}

#[tauri::command]
fn get_volume_values(state: State<AppState>) -> Vec<String> {
    let current_volume = config::get_volume(&state);
    let mut all_volumes: Vec<f32> = (0..=100).map(|i| i as f32 * 0.01).collect();
    
    if let Some(current_index) = all_volumes.iter().position(|&v| (v - current_volume).abs() < 0.001) {
        all_volumes.rotate_left(current_index);
    }
    
    all_volumes.into_iter().map(|v| format!("{:.2}", v)).collect()
}

#[tauri::command]
fn get_voices(state: State<AppState>) -> Vec<String> {
    use std::fs;
    
    let voices_path = if cfg!(debug_assertions) {
        "../realtimetts-resources/voices"
    } else {
        "./realtimetts-resources/voices"
    };
    
    let mut voices = match fs::read_dir(voices_path) {
        Ok(entries) => {
            entries
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("json") {
                        path.file_name().and_then(|s| s.to_str()).map(|s| s.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
        }
        Err(_) => vec!["No voices found".to_string()]
    };
    
    if voices.is_empty() {
        return vec!["No voices found".to_string()];
    }
    
    // Rotate list to put current voice first
    if let Ok(current_voice) = config::get_voice(&state) {
        if let Some(current_index) = voices.iter().position(|v| v == &current_voice) {
            voices.rotate_left(current_index);
        }
    }
    
    voices
}

#[tauri::command]
fn is_dialog_active(state: State<AppState>) -> bool {
    *state.dialog_active.lock().unwrap()
}

#[tauri::command]
async fn process_input(text: String, app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    if text.starts_with('/') {
        handle_command(&text[1..], app, state).await
    } else {
        handle_text(text).await
    }
}

async fn handle_command(command_str: &str, app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    let (command, argument) = if let Some(index) = command_str.find(' ') {
        (&command_str[..index], Some(command_str[index + 1..].to_string()))
    } else {
        (command_str, None)
    };

    match command {
        "center" => commands::center_command(app, state).await,
        "exit" => commands::exit_command(app, state).await,
        "nextmonitor" => commands::nextmonitor_command(app, state).await,
        "topleft" => commands::topleft_command(app, state).await,
        "topright" => commands::topright_command(app, state).await,
        "bottomleft" => commands::bottomleft_command(app, state).await,
        "bottomright" => commands::bottomright_command(app, state).await,
        "resetconfig" => commands::resetconfig_command(app, state).await,
        "outputdevice" => commands::outputdevice_command(argument, state).await,
        "volume" => commands::volume_command(argument, state).await,
        "clonevoice" => commands::clonevoice_command(app, state).await,
        "restartserver" => commands::restartserver_command(state).await,
        "start" => commands::start_command(argument, state).await,
        "listdevices" => commands::listdevices_command().await,
        "stop" => commands::stop_command().await,
        "changevoice" => commands::changevoice_command(argument, state).await,
        _ => Err(format!("Unknown command: {}", command))
    }
}

async fn handle_text(text: String) -> Result<String, String> {
    match bridge::send_speak_request(text).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Failed to send text to TTS: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Run setup first, before creating any UI
    if !initial_setup::is_setup_complete() {
        log::tauri_log("Python environment not found, running setup...");
        if let Err(e) = tauri::async_runtime::block_on(initial_setup::run_setup()) {
            eprintln!("Failed to run Python setup: {}", e);
            std::process::exit(1);
        }
    }
    
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            config: Mutex::new(config::load_config().unwrap_or_default()),
            dialog_active: Mutex::new(false),
            server_process: Mutex::new(None),
        })
        .setup(|app| {
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| {
                    if event.id() == "quit" {
                        let state = app.state::<AppState>();
                        server_utils::stop_server(state);
                        app.exit(0);
                    }
                })
                .build(app)?;

            // Apply config after window setup
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state = app_handle.state::<AppState>();
                if let Err(e) = config::apply_config(app_handle.clone(), state.clone()).await {
                    eprintln!("Failed to apply config on startup: {}", e);
                }
            });

            // Start server after window is ready
            if let Some(_window) = app.get_webview_window("main") {
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    if initial_setup::is_setup_complete() {
                        match server_utils::start_server() {
                            Ok(child) => {
                                let state = app_handle.state::<AppState>();
                                let mut server_process = state.server_process.lock().unwrap();
                                *server_process = Some(child);
                            },
                            Err(e) => {
                                eprintln!("Failed to start Python server: {}", e);
                            }
                        }
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![process_input, get_available_commands, get_output_devices, get_volume_values, get_voices, is_dialog_active])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}