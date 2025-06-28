use std::path::PathBuf;
use std::process::Command;
use std::io::Write;
use std::fs::OpenOptions;

#[derive(Debug)]
pub enum SetupError {
    IoError(std::io::Error),
    CommandFailed(String),
    PathNotFound(String),
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupError::IoError(err) => write!(f, "IO Error: {}", err),
            SetupError::CommandFailed(msg) => write!(f, "Command Failed: {}", msg),
            SetupError::PathNotFound(path) => write!(f, "Path Not Found: {}", path),
        }
    }
}

impl From<std::io::Error> for SetupError {
    fn from(error: std::io::Error) -> Self {
        SetupError::IoError(error)
    }
}

fn create_log_file() -> Result<std::fs::File, std::io::Error> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open("setup.log")
}

fn log_message(message: &str) {
    if let Ok(mut file) = create_log_file() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let _ = writeln!(file, "[{}] {}", timestamp, message);
    }
    crate::log::tauri_log(message);
}

pub fn is_setup_complete() -> bool {
    if cfg!(debug_assertions) {
        // In development, assume developers handle their own Python setup
        true
    } else {
        let venv_python = PathBuf::from("./_up_/var/venv/Scripts/python.exe");
        venv_python.exists() && check_ffmpeg_available()
    }
}

fn check_ffmpeg_available() -> bool {
    match Command::new("ffmpeg")
        .arg("-version")
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                log_message("FFmpeg found on system");
                true
            } else {
                log_message("WARNING: FFmpeg not found. Audio processing may not work properly.");
                log_message("Please install FFmpeg: https://ffmpeg.org/download.html");
                false
            }
        },
        Err(_) => {
            log_message("WARNING: FFmpeg not found. Audio processing may not work properly.");
            log_message("Please install FFmpeg: https://ffmpeg.org/download.html");
            false
        }
    }
}

pub async fn run_setup() -> Result<(), SetupError> {
    if cfg!(debug_assertions) {
        // In development, developers handle their own Python environment
        log_message("Development mode: Python setup is handled by developer");
        Ok(())
    } else {
        run_production_setup().await
    }
}


async fn run_production_setup() -> Result<(), SetupError> {
    log_message("Running production setup...");
    
    let python_exe = PathBuf::from("./_up_/python-resources/runtime/python/embed/python.exe");
    let get_pip = PathBuf::from("./_up_/python-resources/runtime/lib/get-pip.py");
    let requirements = PathBuf::from("./_up_/src-python/requirements.txt");
    let venv_dir = PathBuf::from("./_up_/var/venv");
    
    // Verify required files exist
    if !python_exe.exists() {
        let msg = format!("Python executable not found: {:?}", python_exe);
        log_message(&msg);
        return Err(SetupError::PathNotFound(msg));
    }
    if !get_pip.exists() {
        let msg = format!("get-pip.py not found: {:?}", get_pip);
        log_message(&msg);
        return Err(SetupError::PathNotFound(msg));
    }
    if !requirements.exists() {
        let msg = format!("requirements.txt not found: {:?}", requirements);
        log_message(&msg);
        return Err(SetupError::PathNotFound(msg));
    }
    
    log_message("All required files found");
    
    // Create var directory if it doesn't exist
    if let Some(parent) = venv_dir.parent() {
        std::fs::create_dir_all(parent)?;
        log_message("Created var directory");
    }
    
    // Step 1: Install pip to the embedded Python directory
    log_message("Installing pip...");
    let python_dir = python_exe.parent().unwrap();
    let status = Command::new(&python_exe)
        .arg(&get_pip)
        .arg("--target")
        .arg(python_dir)
        .status()?;
        
    if !status.success() {
        log_message("ERROR: Failed to install pip");
        return Err(SetupError::CommandFailed("Failed to install pip".to_string()));
    }
    log_message("Pip installed successfully");
    
    // Step 2: Install virtualenv to the embedded Python directory
    log_message("Installing virtualenv...");
    let status = Command::new(&python_exe)
        .arg("-m")
        .arg("pip")
        .arg("install")
        .arg("--target")
        .arg(python_dir)
        .arg("virtualenv")
        .status()?;
        
    if !status.success() {
        log_message("ERROR: Failed to install virtualenv");
        return Err(SetupError::CommandFailed("Failed to install virtualenv".to_string()));
    }
    log_message("Virtualenv installed successfully");
    
    // Step 3: Create virtual environment using virtualenv
    log_message("Creating virtual environment...");
    let status = Command::new(&python_exe)
        .arg("-m")
        .arg("virtualenv")
        .arg(&venv_dir)
        .status()?;
        
    if !status.success() {
        log_message("ERROR: Failed to create virtual environment");
        return Err(SetupError::CommandFailed("Failed to create venv".to_string()));
    }
    log_message("Virtual environment created successfully");
    
    // Step 4: Install requirements
    log_message("Installing requirements...");
    let venv_pip = venv_dir.join("Scripts/pip.exe");
    let status = Command::new(&venv_pip)
        .arg("install")
        .arg("--no-cache-dir")
        .arg("-r")
        .arg(&requirements)
        .status()?;
        
    if !status.success() {
        log_message("ERROR: Failed to install requirements");
        return Err(SetupError::CommandFailed("Failed to install requirements".to_string()));
    }
    
    log_message("Requirements installed successfully");
    log_message("Production setup complete!");
    Ok(())
}