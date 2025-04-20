use rand::Rng;
use serde::Serialize;
use std::collections::VecDeque;
use std::sync::Mutex;
use tauri::Manager;

const HISTORY_SIZE: usize = 60; // For 1 minute of history

#[derive(Debug, Serialize, Clone, Copy)]
#[allow(dead_code)]
enum State {
    Unknown,
    Idle,
    Distracted1,
    Distracted2,
    Distracted3,
    Distracted4,
    Focused1,
    Focused2,
    Focused3,
    Focused4,
}

struct Menhera {
    current_state: State,
    history: VecDeque<State>,
}

impl Menhera {
    fn new() -> Self {
        Self {
            current_state: State::Unknown,
            history: VecDeque::with_capacity(HISTORY_SIZE),
        }
    }

    // Updates the current state and pushes it to the history queue
    fn update_state(&mut self, new_state: State) {
        self.current_state = new_state;
        if self.history.len() == HISTORY_SIZE {
            self.history.pop_front();
        }
        self.history.push_back(new_state);
    }

    // Returns the current state
    fn get_state(&self) -> &State {
        &self.current_state // Return a reference
    }

    // Returns an iterator over the history
    fn history(&self) -> impl Iterator<Item = &State> {
        self.history.iter()
    }
}

#[tauri::command]
async fn parse_image(app: tauri::AppHandle, image: Vec<u8>) -> Result<State, String> {
    println!("Parsing image...");
    // This part still needs to be synchronous since image processing is CPU-bound
    // let img: image::DynamicImage = image::load_from_memory(&image).map_err(|e| e.to_string())?;

    println!("Image parsed");

    // // Use async file operations
    // let path = "../image.png";

    // // Create a buffer in memory first
    // let mut buffer = std::io::Cursor::new(Vec::new());
    // img.write_to(&mut buffer, image::ImageFormat::Png)
    //     .map_err(|e| e.to_string())?;

    // // Write buffer to file asynchronously
    // tokio::fs::write(path, buffer.into_inner())
    //     .await
    //     .map_err(|e| e.to_string())?;

    // // Get absolute path (also async)
    // let absolute_path = tokio::fs::canonicalize(path)
    //     .await
    //     .map_err(|e| e.to_string())?;

    // println!("Created file at: {}", absolute_path.display());

    // Simulate image processing by returning a random state

    let mut rng = rand::rng();
    let states = [
        State::Idle,
        State::Distracted1,
        State::Distracted2,
        State::Distracted3,
        State::Distracted4,
        State::Focused1,
        State::Focused2,
        State::Focused3,
        State::Focused4,
    ];
    let random_state = states[rng.random_range(0..states.len())];
    println!("Returning random state: {:?}", random_state);

    app.state::<Mutex<Menhera>>()
        .lock()
        .unwrap()
        .update_state(random_state);

    Ok(random_state)
}

#[tauri::command]
async fn open_menhera(app: tauri::AppHandle) -> Result<(), String> {
    let webview_window = tauri::WebviewWindowBuilder::from_config(
        &app,
        &app.config().app.windows.get(1).unwrap().clone(),
    )
    .unwrap()
    .build()
    .unwrap();

    webview_window.open_devtools();

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(Menhera::new()))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![parse_image, open_menhera])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
