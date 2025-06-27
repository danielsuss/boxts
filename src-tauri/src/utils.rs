use tauri::{Manager, PhysicalPosition, Position, State};
use crate::AppState;

pub async fn move_window_center(app: tauri::AppHandle) -> Result<(), String> {
    let window = app.get_webview_window("main")
        .ok_or("Failed to get main window")?;
    
    window.center()
        .map_err(|e| format!("Failed to center window: {}", e))?;
    
    Ok(())
}

pub async fn move_window_topleft(app: tauri::AppHandle) -> Result<(), String> {
    let window = app.get_webview_window("main")
        .ok_or("Failed to get main window")?;
    
    let current_monitor = window.current_monitor()
        .map_err(|e| format!("Failed to get current monitor: {}", e))?
        .ok_or("No current monitor detected")?;
    
    let work_area = current_monitor.work_area();
    let margin = 10;
    
    let new_pos = PhysicalPosition::new(
        work_area.position.x + margin,
        work_area.position.y + margin
    );
    
    window.set_position(Position::Physical(new_pos))
        .map_err(|e| format!("Failed to set window position: {}", e))?;
    
    Ok(())
}

pub async fn move_window_topright(app: tauri::AppHandle) -> Result<(), String> {
    let window = app.get_webview_window("main")
        .ok_or("Failed to get main window")?;
    
    let current_monitor = window.current_monitor()
        .map_err(|e| format!("Failed to get current monitor: {}", e))?
        .ok_or("No current monitor detected")?;
    
    let work_area = current_monitor.work_area();
    let window_size = window.outer_size()
        .map_err(|e| format!("Failed to get window size: {}", e))?;
    let margin = 10;
    
    let new_pos = PhysicalPosition::new(
        work_area.position.x + work_area.size.width as i32 - window_size.width as i32 + margin,
        work_area.position.y + margin
    );
    
    window.set_position(Position::Physical(new_pos))
        .map_err(|e| format!("Failed to set window position: {}", e))?;
    
    Ok(())
}

pub async fn move_window_bottomleft(app: tauri::AppHandle) -> Result<(), String> {
    let window = app.get_webview_window("main")
        .ok_or("Failed to get main window")?;
    
    let current_monitor = window.current_monitor()
        .map_err(|e| format!("Failed to get current monitor: {}", e))?
        .ok_or("No current monitor detected")?;
    
    let work_area = current_monitor.work_area();
    let window_size = window.outer_size()
        .map_err(|e| format!("Failed to get window size: {}", e))?;
    let margin = 10;
    
    let new_pos = PhysicalPosition::new(
        work_area.position.x + margin,
        work_area.position.y + work_area.size.height as i32 - window_size.height as i32 + margin
    );
    
    window.set_position(Position::Physical(new_pos))
        .map_err(|e| format!("Failed to set window position: {}", e))?;
    
    Ok(())
}

pub async fn move_window_bottomright(app: tauri::AppHandle) -> Result<(), String> {
    let window = app.get_webview_window("main")
        .ok_or("Failed to get main window")?;
    
    let current_monitor = window.current_monitor()
        .map_err(|e| format!("Failed to get current monitor: {}", e))?
        .ok_or("No current monitor detected")?;
    
    let work_area = current_monitor.work_area();
    let window_size = window.outer_size()
        .map_err(|e| format!("Failed to get window size: {}", e))?;
    let margin = 10;
    
    let new_pos = PhysicalPosition::new(
        work_area.position.x + work_area.size.width as i32 - window_size.width as i32 + margin,
        work_area.position.y + work_area.size.height as i32 - window_size.height as i32 + margin
    );
    
    window.set_position(Position::Physical(new_pos))
        .map_err(|e| format!("Failed to set window position: {}", e))?;
    
    Ok(())
}

pub async fn switch_to_next_monitor(app: tauri::AppHandle) -> Result<String, String> {
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

pub async fn apply_window_position(app: tauri::AppHandle, _state: State<'_, AppState>, position: &str) -> Result<String, String> {
    match position {
        "center" => {
            move_window_center(app).await?;
            Ok("Window centered".to_string())
        },
        "topleft" => {
            move_window_topleft(app).await?;
            Ok("Window moved to top-left".to_string())
        },
        "topright" => {
            move_window_topright(app).await?;
            Ok("Window moved to top-right".to_string())
        },
        "bottomleft" => {
            move_window_bottomleft(app).await?;
            Ok("Window moved to bottom-left".to_string())
        },
        "bottomright" => {
            move_window_bottomright(app).await?;
            Ok("Window moved to bottom-right".to_string())
        },
        _ => Err(format!("Unknown window position: {}", position))
    }
}