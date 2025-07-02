use std::path::PathBuf;
use std::process::{Command, Child, Stdio};
use std::fs::OpenOptions;
use tauri::{State, Emitter};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt};

pub fn get_python_paths() -> (PathBuf, PathBuf) {
    if cfg!(debug_assertions) {
        (
            PathBuf::from("../venv/Scripts/python.exe"),
            PathBuf::from("../src-python/server.py")
        )
    } else {
        (
            PathBuf::from("./_up_/var/venv/Scripts/python.exe"),
            PathBuf::from("./_up_/src-python/server.py")
        )
    }
}

pub fn start_server() -> Result<Child, std::io::Error> {
    crate::log::tauri_log("Starting Python server...");
    let (python_exe, server_script) = get_python_paths();
    
    let mut command = Command::new(python_exe);
    command.arg(server_script);
    
    // In production, redirect output to server.log
    if !cfg!(debug_assertions) {
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("server.log")?;
        
        command.stdout(Stdio::from(log_file.try_clone()?));
        command.stderr(Stdio::from(log_file));
    }
    
    command.spawn()
}

pub fn stop_server(state: State<crate::AppState>) {
    let mut server_process = state.server_process.lock().unwrap();
    if let Some(ref mut child) = *server_process {
        crate::log::tauri_log("Stopping Python server...");
        if let Err(e) = child.kill() {
            eprintln!("Failed to kill server process: {}", e);
        } else {
            crate::log::tauri_log("Python server stopped");
        }
        *server_process = None;
    }
}

pub async fn listen_for_ready(app_handle: tauri::AppHandle) {
    tokio::spawn(async move {
        loop {
            match connect_async("ws://127.0.0.1:8000/ws").await {
                Ok((mut ws_stream, _)) => {
                    crate::log::tauri_websocket_log("Connected to WebSocket for ready signals");
                    
                    while let Some(msg) = ws_stream.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                if text == "ready" {
                                    crate::log::tauri_websocket_log("Ready!");
                                    if let Err(e) = app_handle.emit("ready", ()) {
                                        crate::log::tauri_log(&format!("Failed to emit ready event: {}", e));
                                    }
                                }
                            }
                            Ok(Message::Close(_)) => {
                                crate::log::tauri_websocket_log("WebSocket connection closed");
                                break;
                            }
                            Err(e) => {
                                crate::log::tauri_websocket_log(&format!("WebSocket error: {}", e));
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                Err(_) => {}
            }
        }
    });
}