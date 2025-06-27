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
}

const AVAILABLE_COMMANDS: &[&str] = &["center", "exit", "nextmonitor", "topleft", "topright", "bottomleft", "bottomright", "resetconfig", "test"];

#[tauri::command]
fn get_available_commands() -> Vec<String> {
    AVAILABLE_COMMANDS.iter().map(|s| s.to_string()).collect()
}

#[tauri::command]
fn get_test_items() -> Vec<String> {
    vec!["Device 1".to_string(), "Device 2".to_string(), "Device 3".to_string()]
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
        "test" => commands::test_command(argument).await,
        _ => Err(format!("Unknown command: {}", command))
    }
}

async fn handle_text(text: String) -> Result<String, String> {
    Ok("Text processed".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            config: Mutex::new(config::load_config().unwrap_or_default()),
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
        .invoke_handler(tauri::generate_handler![process_input, get_available_commands, get_test_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}