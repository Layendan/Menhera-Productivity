use std::fs::File;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn parse_image(image: Vec<u8>) -> Result<(), String> {
    let img: image::DynamicImage = image::load_from_memory(&image).map_err(|e| e.to_string())?;

    let mut file = File::create("image.png").map_err(|e| e.to_string())?;
    img.write_to(&mut file, image::ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    let absolute_path = std::fs::canonicalize("image.png").map_err(|e| e.to_string())?;

    println!("Created file at: {}", absolute_path.display());

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
