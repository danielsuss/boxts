use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

#[tauri::command]
async fn process_input(text: String) -> Result<String, String> {
    if text.starts_with('/') {
        handle_command(&text[1..]).await
    } else {
        handle_text(text).await
    }
}

async fn handle_command(command_str: &str) -> Result<String, String> {
    let parts: Vec<&str> = command_str.split_whitespace().collect();
    
    if parts.is_empty() {
        return Err("Empty command".to_string());
    }
    
    let command = parts[0];
    println!("Command: {}", command);
    
    match command {
        _ => Ok(format!("Unknown command: {}", command))
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
        .setup(|app| {
            // Create tray menu
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit])?;

            // Create system tray icon
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
        .invoke_handler(tauri::generate_handler![process_input])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
