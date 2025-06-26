use tauri::Manager;

pub async fn center_command(app: tauri::AppHandle) -> Result<String, String> {
    let window = app.get_webview_window("main")
        .ok_or("Failed to get main window")?;
    
    window.center()
        .map_err(|e| format!("Failed to center window: {}", e))?;
    
    Ok("Window centered".to_string())
}