use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
    State,
};
use std::sync::Mutex;

mod commands;
mod config;
mod utils;

struct AppState {
    config: Mutex<config::BoxtsConfig>,
    dialog_active: Mutex<bool>,
}

const AVAILABLE_COMMANDS: &[&str] = &["center", "exit", "nextmonitor", "topleft", "topright", "bottomleft", "bottomright", "resetconfig", "outputdevice", "volume", "trainmodel"];

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
        "exit" => commands::exit_command(app).await,
        "nextmonitor" => commands::nextmonitor_command(app, state).await,
        "topleft" => commands::topleft_command(app, state).await,
        "topright" => commands::topright_command(app, state).await,
        "bottomleft" => commands::bottomleft_command(app, state).await,
        "bottomright" => commands::bottomright_command(app, state).await,
        "resetconfig" => commands::resetconfig_command(app, state).await,
        "outputdevice" => commands::outputdevice_command(argument, state).await,
        "volume" => commands::volume_command(argument, state).await,
        "trainmodel" => commands::trainmodel_command(app, state).await,
        _ => Err(format!("Unknown command: {}", command))
    }
}

async fn handle_text(_text: String) -> Result<String, String> {
    Ok("Text processed".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            config: Mutex::new(config::load_config().unwrap_or_default()),
            dialog_active: Mutex::new(false),
        })
        .setup(|app| {
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| {
                    if event.id() == "quit" {
                        app.exit(0);
                    }
                })
                .build(app)?;

            // Apply config after app setup
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let state = app_handle.state::<AppState>();
                if let Err(e) = config::apply_config(app_handle.clone(), state).await {
                    eprintln!("Failed to apply config on startup: {}", e);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![process_input, get_available_commands, get_output_devices, get_volume_values, is_dialog_active])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}