use std::{fs::File, io::Write};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn parse_image(image: Vec<u8>) -> Result<(), String> {
    let img = image::load_from_memory(&image).map_err(|e| e.to_string())?;

    let mut file = File::create("image.txt").map_err(|e| e.to_string())?;
    file.write_all(img.as_bytes()).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, parse_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
