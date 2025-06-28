use std::path::PathBuf;
use std::process::{Command, Child};
use tauri::State;

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
    
    Command::new(python_exe)
        .arg(server_script)
        .spawn()
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