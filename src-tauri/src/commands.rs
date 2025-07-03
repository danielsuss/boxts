use tauri::{Manager, State};
use tauri_plugin_dialog::DialogExt;
use crate::{AppState, config, utils, server_utils, bridge};

pub async fn exit_command(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    // Stop server before exiting
    server_utils::stop_server(state);
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

pub async fn outputdevice_command(argument: Option<String>, state: State<'_, crate::AppState>) -> Result<String, String> {
    match argument {
        Some(device_name) => {
            let _ = config::set_output_device(&state, &device_name);
            crate::log::tauri_log(&format!("Selected output device: {}", device_name));
            Ok(format!("Output device set to: {}", device_name))
        },
        None => Err("No output device selected".to_string()),
    }
}

pub async fn volume_command(argument: Option<String>, state: State<'_, crate::AppState>) -> Result<String, String> {
    match argument {
        Some(volume_str) => {
            match volume_str.parse::<f32>() {
                Ok(volume) => {
                    let _ = config::set_volume(&state, volume);
                    crate::log::tauri_log(&format!("Selected volume: {}", volume));
                    
                    // Send volume update request to Python server
                    match bridge::send_volume_request().await {
                        Ok(_response) => Ok(format!("Volume set to: {}", volume)),
                        Err(e) => Err(format!("Failed to update volume: {}", e)),
                    }
                },
                Err(_) => Err("Invalid volume value".to_string()),
            }
        },
        None => Err("No volume selected".to_string()),
    }
}

pub async fn clonevoice_command(app: tauri::AppHandle, state: State<'_, crate::AppState>) -> Result<String, String> {
    {
        let mut dialog_active = state.dialog_active.lock().unwrap();
        *dialog_active = true;
    }
    
    let file_path = app
        .dialog()
        .file()
        .add_filter("Audio Files", &["mp3", "wav", "flac", "ogg", "m4a", "aac"])
        .blocking_pick_file();
    
    {
        let mut dialog_active = state.dialog_active.lock().unwrap();
        *dialog_active = false;
    }
    
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }

    match file_path {
        Some(path) => {
            let path_str = path.to_string();
            crate::log::tauri_log(&format!("Selected voice file for cloning: {:?}", path));
            
            // Send clone voice request to Python server
            match bridge::send_clonevoice_request(path_str).await {
                Ok(response) => Ok(format!("Voice cloning started: {}", response)),
                Err(e) => Err(format!("Failed to start voice cloning: {}", e)),
            }
        },
        None => {
            server_utils::emit_ready(app.clone()).await;
            Ok("File selection cancelled".to_string())
        },
    }
}

pub async fn restartserver_command(state: State<'_, crate::AppState>) -> Result<String, String> {
    crate::log::tauri_log("Restarting Python server...");
    
    // Stop the current server
    server_utils::stop_server(state.clone());
    
    // Start a new server
    match server_utils::start_server() {
        Ok(child) => {
            let mut server_process = state.server_process.lock().unwrap();
            *server_process = Some(child);
            Ok("Server restarted successfully".to_string())
        },
        Err(e) => Err(format!("Failed to restart server: {}", e)),
    }
}

pub async fn start_command(argument: Option<String>, state: State<'_, crate::AppState>) -> Result<String, String> {
    match argument {
        Some(voice_name) => {
            // Save the selected voice to config
            let _ = config::set_voice(&state, &voice_name);
            crate::log::tauri_log(&format!("Selected voice: {}", voice_name));
            
            // Send start request to Python server
            match bridge::send_start_request(voice_name.clone()).await {
                Ok(_response) => Ok(format!("TTS started with voice: {}", voice_name)),
                Err(e) => Err(format!("Failed to start TTS: {}", e)),
            }
        },
        None => Err("No voice selected".to_string()),
    }
}

pub async fn listdevices_command() -> Result<String, String> {
    crate::log::tauri_log("Listing audio devices...");
    
    // Send list devices request to Python server
    match bridge::send_listdevices_request().await {
        Ok(_response) => {
            crate::log::tauri_log("Audio devices listed successfully");
            Ok("Audio devices listed in server log".to_string())
        },
        Err(e) => Err(format!("Failed to list devices: {}", e)),
    }
}

pub async fn stop_command() -> Result<String, String> {
    crate::log::tauri_log("Stopping TTS and cleaning up resources...");
    
    // Send stop request to Python server
    match bridge::send_stop_request().await {
        Ok(_response) => {
            crate::log::tauri_log("TTS stopped and resources cleaned up successfully");
            Ok("TTS stopped and resources cleaned up".to_string())
        },
        Err(e) => Err(format!("Failed to stop TTS: {}", e)),
    }
}

pub async fn changevoice_command(argument: Option<String>, state: State<'_, crate::AppState>) -> Result<String, String> {
    match argument {
        Some(voice_name) => {
            // Save the selected voice to config
            let _ = config::set_voice(&state, &voice_name);
            crate::log::tauri_log(&format!("Selected voice: {}", voice_name));
            
            // Send changevoice request to Python server
            match bridge::send_changevoice_request(voice_name.clone()).await {
                Ok(_response) => Ok(format!("Voice changed to: {}", voice_name)),
                Err(e) => Err(format!("Failed to change voice: {}", e)),
            }
        },
        None => Err("No voice selected".to_string()),
    }
}

pub async fn ready_command() -> Result<String, String> {
    crate::log::tauri_log("Sending manual ready signal...");
    
    // Send ready request to Python server
    match bridge::send_ready_request().await {
        Ok(_response) => {
            Ok("Ready signal sent".to_string())
        },
        Err(e) => Err(format!("Failed to send ready signal: {}", e)),
    }
}

pub async fn lostfocus_command(argument: Option<String>, state: State<'_, crate::AppState>) -> Result<String, String> {
    match argument {
        Some(behaviour) => {
            match behaviour.as_str() {
                "show" | "hide" => {
                    let _ = config::set_lostfocus_behaviour(&state, &behaviour);
                    crate::log::tauri_log(&format!("Lost focus behaviour set to: {}", behaviour));
                    Ok(format!("Lost focus behaviour set to: {}", behaviour))
                },
                _ => Err("Invalid behaviour. Use 'show' or 'hide'.".to_string()),
            }
        },
        None => Err("No behaviour selected".to_string()),
    }
}

