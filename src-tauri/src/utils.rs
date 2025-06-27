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

pub async fn switch_to_next_monitor(app: tauri::AppHandle) -> Result<(String, u32), String> {
    let window = app.get_webview_window("main")
        .ok_or("Failed to get main window")?;
    
    let current_monitor = window.current_monitor()
        .map_err(|e| format!("Failed to get current monitor: {}", e))?
        .ok_or("No current monitor detected")?;
    
    let all_monitors = app.available_monitors()
        .map_err(|e| format!("Failed to get available monitors: {}", e))?;
    
    if all_monitors.len() <= 1 {
        return Ok(("Only one monitor available".to_string(), 0));
    }
    
    let current_idx = all_monitors.iter().position(|m| 
        m.name() == current_monitor.name()
    ).ok_or("Current monitor not found in available monitors")?;
    
    let next_idx = (current_idx + 1) % all_monitors.len();
    let next_monitor_id = next_idx as u32;
    
    // Use switch_to_monitor to handle the actual switching
    let (switch_result, new_monitor_id) = switch_to_monitor(app, next_monitor_id).await?;
    
    Ok((switch_result, new_monitor_id))
}


pub async fn switch_to_monitor(app: tauri::AppHandle, monitor_id: u32) -> Result<(String, u32), String> {
    let all_monitors = app.available_monitors()
        .map_err(|e| format!("Failed to get available monitors: {}", e))?;
    
    let target_monitor = if let Some(monitor) = all_monitors.get(monitor_id as usize) {
        monitor.clone()
    } else {
        all_monitors.get(0)
            .ok_or("No monitors available")?
            .clone()
    };

    let target_monitor_idx = all_monitors.iter().position(|m| 
        m.name() == target_monitor.name()
    ).unwrap();

    let target_monitor_id = target_monitor_idx as u32;
    
    let window = app.get_webview_window("main")
        .ok_or("Failed to get main window")?;

    let work_area = target_monitor.work_area();
    let center_pos = PhysicalPosition::new(
        work_area.position.x + work_area.size.width as i32 / 2,
        work_area.position.y + work_area.size.height as i32 / 2
    );
    
    window.set_position(Position::Physical(center_pos))
        .map_err(|e| format!("Failed to move to target monitor: {}", e))?;
    
    Ok((format!("Switched to monitor: {}", 
        target_monitor.name().map_or("Unknown", |v| v)), target_monitor_id))
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