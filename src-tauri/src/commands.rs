use tauri::State;
use crate::{AppState, config, utils};

pub async fn exit_command(app: tauri::AppHandle) -> Result<String, String> {
    app.cleanup_before_exit();
    app.exit(0);
    Ok("Application exited".to_string())
}

pub async fn nextmonitor_command(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    // Switch to next monitor and get the new monitor ID
    let (switch_result, new_monitor_id) = utils::switch_to_next_monitor(app.clone()).await?;

    // Save the new monitor ID to config
    let _ = config::set_monitor_id(&state, new_monitor_id);

    // Re-apply current position on new monitor
    let current_position_str = config::get_window_position(&state);
    utils::apply_window_position(app, state, &current_position_str).await?;
    
    Ok(switch_result)
}

pub async fn center_command(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    let result = utils::apply_window_position(app, state.clone(), "center").await?;
    let _ = config::set_window_position(&state, "center");
    Ok(result)
}

pub async fn topleft_command(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    let result = utils::apply_window_position(app, state.clone(), "topleft").await?;
    let _ = config::set_window_position(&state, "topleft");
    Ok(result)
}

pub async fn topright_command(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    let result = utils::apply_window_position(app, state.clone(), "topright").await?;
    let _ = config::set_window_position(&state, "topright");
    Ok(result)
}

pub async fn bottomleft_command(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    let result = utils::apply_window_position(app, state.clone(), "bottomleft").await?;
    let _ = config::set_window_position(&state, "bottomleft");
    Ok(result)
}

pub async fn bottomright_command(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    let result = utils::apply_window_position(app, state.clone(), "bottomright").await?;
    let _ = config::set_window_position(&state, "bottomright");
    Ok(result)
}

pub async fn resetconfig_command(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    config::reset_config(app, state).await.map_err(|e| format!("Failed to reset config: {}", e))?;
    
    Ok("Config reset to defaults".to_string())
}

pub async fn test_command(argument: Option<String>) -> Result<String, String> {
    match argument {
        Some(arg) => {
            println!("Selected item: {}", arg);
            Ok(format!("Selected item: {}", arg))
        },
        None => Err("No item selected for /test command".to_string()),
    }
}