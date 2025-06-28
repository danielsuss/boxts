const TAURI_STRING: &str = "\x1b[96mTAU\x1b[38;5;208mRI\x1b[37m:\x1b[0m    ";

pub fn tauri_log(message: &str) {
    println!("{}{}", TAURI_STRING, message);
}