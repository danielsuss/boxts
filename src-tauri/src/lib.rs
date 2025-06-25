use reqwest::Client;
use serde::Serialize;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

#[derive(Serialize)]
struct Payload {
    text: String,
}

// Define the Tauri command for posting data to the Flask server
#[tauri::command]
async fn input(text: String) -> Result<String, String> {
    let client = Client::new();
    let url = "http://127.0.0.1:5000/tts";

    let payload = Payload { text };

    // Send the POST request and handle the response
    match client.post(url).json(&payload).send().await {
        Ok(response) => match response.text().await {
            Ok(body) => Ok(body),
            Err(_) => Err("Failed to read response body".to_string()),
        },
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            let ctrl_tilde_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Backquote);
            
            // Register global shortcut with handler
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, shortcut, event| {
                    if shortcut == &ctrl_tilde_shortcut && event.state() == ShortcutState::Pressed {
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build()
            ).expect("Failed to register global shortcut plugin");
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![input])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
