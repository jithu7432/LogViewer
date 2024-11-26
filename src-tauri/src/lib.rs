use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use serde::Serialize;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[derive(Debug, Serialize)]
struct LogElement {
    line_no: usize,
    datetime: String,
    log_level: String,
    message: String,
}
#[tauri::command]
fn get_logs() -> Vec<LogElement> {
    let re = Regex::new(r"([0-9].*?:\d+)\s\[(.*?)\]\s(.*)").unwrap();
    let s = r"C:\Users\JJ\Downloads\t.log";

    let mut my_strings = Vec::new();

    let file = File::open(s).expect("Failed to open file");
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            if let Some(cap) = re.captures(line.as_str()) {
                let group1 = cap.get(1).unwrap().as_str();
                let group2 = cap.get(2).unwrap().as_str();
                let group3 = cap.get(3).unwrap().as_str();
                my_strings.push(LogElement {
                    line_no: i + 1,
                    datetime: group1.to_string(),
                    log_level: group2.to_string(),
                    message: group3.to_string(),
                })
            }
        }
    }
    my_strings
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_logs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
