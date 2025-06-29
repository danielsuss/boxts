use std::path::PathBuf;
use std::process::{Command, Child, Stdio};
use std::fs::OpenOptions;
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