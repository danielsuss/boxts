use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    State,
};
use std::sync::Mutex;

mod commands;
mod config;

#[derive(Debug, Clone)]
enum WindowPosition {
    FreePos,
    Center,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

struct AppState {
    window_position: Mutex<WindowPosition>,
    config: Mutex<config::BoxtsConfig>,
}

const AVAILABLE_COMMANDS: &[&str] = &["center", "exit", "nextmonitor", "topleft", "topright", "bottomleft", "bottomright"];

#[tauri::command]
fn get_available_commands() -> Vec<String> {
    AVAILABLE_COMMANDS.iter().map(|s| s.to_string()).collect()
}

#[tauri::command]
async fn process_input(text: String, app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    if text.starts_with('/') {
        handle_command(&text[1..], app, state).await
    } else {
        *state.window_position.lock().unwrap() = WindowPosition::FreePos;
        handle_text(text).await
    }
}

async fn handle_command(command_str: &str, app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    let parts: Vec<&str> = command_str.split_whitespace().collect();
    
    if parts.is_empty() {
        return Err("Empty command".to_string());
    }
    
    let command = parts[0];
    println!("Command: {}", command);
    
    match command {
        "center" => commands::center_command(app, state).await,
        "exit" => commands::exit_command(app).await,
        "nextmonitor" => commands::nextmonitor_command(app, state).await,
        "topleft" => commands::topleft_command(app, state).await,
        "topright" => commands::topright_command(app, state).await,
        "bottomleft" => commands::bottomleft_command(app, state).await,
        "bottomright" => commands::bottomright_command(app, state).await,
        _ => Err(format!("Unknown command: {}", command))
    }
}

async fn handle_text(text: String) -> Result<String, String> {
    println!("Text: {}", text);
    Ok("Text processed".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            window_position: Mutex::new(WindowPosition::FreePos),
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

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![process_input, get_available_commands])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
