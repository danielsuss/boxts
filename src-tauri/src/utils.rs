use tauri::State;
use crate::{AppState, commands};

pub async fn apply_window_position(app: tauri::AppHandle, state: State<'_, AppState>, position: &str) -> Result<String, String> {
    match position {
        "center" => commands::center_command(app, state).await,
        "topleft" => commands::topleft_command(app, state).await,
        "topright" => commands::topright_command(app, state).await,
        "bottomleft" => commands::bottomleft_command(app, state).await,
        "bottomright" => commands::bottomright_command(app, state).await,
        _ => Err(format!("Unknown window position: {}", position))
    }
}