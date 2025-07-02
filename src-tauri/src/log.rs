const TAURI_STRING: &str = "\x1b[96mTAU\x1b[38;5;208mRI\x1b[37m:\x1b[0m    ";
const TAURI_WS_STRING: &str = "\x1b[96mTAU\x1b[38;5;208mRI\x1b[0m \x1b[91mW\x1b[37m:\x1b[0m  ";

pub fn tauri_log(message: &str) {
    println!("{}{}", TAURI_STRING, message);
}

pub fn tauri_websocket_log(message: &str) {
    println!("{}{}", TAURI_WS_STRING, message);
}