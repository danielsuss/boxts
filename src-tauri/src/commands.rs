use tauri::{Manager, PhysicalPosition, Position};

pub async fn center_command(app: tauri::AppHandle) -> Result<String, String> {
    let window = app.get_webview_window("main")
        .ok_or("Failed to get main window")?;
    
    window.center()
        .map_err(|e| format!("Failed to center window: {}", e))?;
    
    Ok("Window centered".to_string())
}

pub async fn exit_command(app: tauri::AppHandle) -> Result<String, String> {
    app.cleanup_before_exit();
    app.exit(0);
    Ok("Application exited".to_string())
}

pub async fn nextmonitor_command(app: tauri::AppHandle) -> Result<String, String> {
    let window = app.get_webview_window("main")
        .ok_or("Failed to get main window")?;
    
    let current_monitor = window.current_monitor()
        .map_err(|e| format!("Failed to get current monitor: {}", e))?
        .ok_or("No current monitor detected")?;
    let current_pos = window.outer_position()
        .map_err(|e| format!("Failed to get window position: {}", e))?;
    let all_monitors = app.available_monitors()
        .map_err(|e| format!("Failed to get available monitors: {}", e))?;
    
    if all_monitors.len() <= 1 {
        return Ok("Only one monitor available".to_string());
    }
    
    let current_idx = all_monitors.iter().position(|m| 
        m.name() == current_monitor.name()
    ).ok_or("Current monitor not found in available monitors")?;
    
    let next_idx = (current_idx + 1) % all_monitors.len();
    let next_monitor = &all_monitors[next_idx];
    
    let current_monitor_pos = current_monitor.position();
    let relative_x = current_pos.x - current_monitor_pos.x;
    let relative_y = current_pos.y - current_monitor_pos.y;
    
    let next_monitor_pos = next_monitor.position();
    let new_pos = PhysicalPosition::new(
        next_monitor_pos.x + relative_x,
        next_monitor_pos.y + relative_y
    );
    
    window.set_position(Position::Physical(new_pos))
        .map_err(|e| format!("Failed to set window position: {}", e))?;
    
    Ok(format!("Moved to monitor: {}", 
        next_monitor.name().map_or("Unknown", |v| v)))
}