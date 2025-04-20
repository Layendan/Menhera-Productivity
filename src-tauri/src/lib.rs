use serde::Serialize;
use std::collections::VecDeque;
use std::sync::Mutex;
use tauri::Manager;
use tch::vision::imagenet;
use tch::CModule;

const HISTORY_SIZE: usize = 15; // number of polls
const DISTRACTED_INTERVALS: [usize; 4] = [15, 30, 45, 60]; // number of polls
const FOCUSED_INTERVALS: [usize; 4] = [15, 30, 45, 60]; // number of polls
const FOCUS_INTERVAL: usize = 2; // number of polls

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
    current_state: State,      // Menhera expression
    history: VecDeque<bool>,   // History of user states
    is_distracted: bool,       // Current user state
    time_in_curr_state: usize, // Time spent in current user state
    model: CModule,
}

impl Menhera {
    fn new() -> Self {
        Self {
            current_state: State::Unknown,                  // Menhera expression
            history: VecDeque::with_capacity(HISTORY_SIZE), // History of user states
            is_distracted: false,                           // Current user state
            time_in_curr_state: 0,                          // Time spent in current user state
            model: CModule::load("resources/model.pt").unwrap(),
        }
    }

    // Updates the current state
    fn update_state(&mut self, new_state: State) {
        self.current_state = new_state;
    }

    // Push state to history queue
    fn update_history(&mut self, user_state: bool) {
        if self.history.len() == HISTORY_SIZE {
            self.history.pop_front();
        }
        self.history.push_back(user_state);
    }

    // Update user's state (is_distracted)
    fn update_is_distracted(&mut self, user_state: bool) {
        self.is_distracted = user_state;
    }

    // Update the timer value
    fn update_timer(&mut self, timer: usize) {
        self.time_in_curr_state = timer;
    }

    // Returns an iterator over the history
    fn get_history(&self) -> impl Iterator<Item = &bool> {
        self.history.iter()
    }

    // Returns user current state
    fn get_is_distracted(&self) -> bool {
        self.is_distracted
    }

    // Returns the timer value
    fn get_timer(&self) -> usize {
        self.time_in_curr_state
    }
}

#[tauri::command]
async fn parse_image(app: tauri::AppHandle, image: Vec<u8>) -> Result<State, String> {
    // #######################################
    // ### Determine next state of Menhera ###
    // #######################################
    // Explanation:
    // There are two states the user can be in (distracted vs. focused)
    // Menhera will determine the current state that the user is in by looking at the majority value of the history window
    // The timer will increase for each poll (5 secs) that the user is in the same state
    // Menhera's state will change depending on how long the user has been in the same state

    // TODO: Get AI classifier result (ResNet?)
    let img = imagenet::load_image_and_resize224_from_memory(&image).map_err(|e| e.to_string())?;
    let mutex_ref = app.state::<Mutex<Menhera>>();
    let mut menhera = mutex_ref.lock().unwrap();
    let input_tensor = img.unsqueeze(0);
    let output = menhera
        .model
        .forward_ts(&[input_tensor])
        .map_err(|e| e.to_string())?
        .softmax(-1, tch::Kind::Float)
        .argmax(1, false)
        .int64_value(&[0]);

    // Convert model output (0 or 1) to boolean
    let ai_result = output == 1;

    // Add AI result to history
    menhera.update_history(ai_result);

    // Determine user's current state based on majority state of the history window
    let history = menhera.get_history();
    // if this is wrong I blame Claude; "majority will be true if there are more true values than false values in the queue"
    let has_majority_true_values = history.filter(|&&b| b).count() > menhera.history.len() / 2;

    // If current user state (history_majority) is different than previous,
    // reset timer and update variable storing user state (is_distracted)
    if !menhera.get_is_distracted() && has_majority_true_values {
        menhera.update_timer(0);
        menhera.update_is_distracted(has_majority_true_values);
    } else if menhera.get_is_distracted() && !has_majority_true_values {
        menhera.update_timer(0);
        menhera.update_is_distracted(has_majority_true_values);
    }

    // Determine Menhera's expression (state) based on timer value
    let timer = menhera.get_timer();
    let distracted = menhera.get_is_distracted();
    let mut next_state = State::Idle;
    if distracted {
        // DISTRACTED_INTERVALS[3] seconds distracted = Distracted4
        if timer >= DISTRACTED_INTERVALS[3] {
            next_state = State::Distracted4;
        }
        // DISTRACTED_INTERVALS[2] seconds distracted = Distracted3
        else if timer >= DISTRACTED_INTERVALS[2] {
            next_state = State::Distracted3;
        }
        // DISTRACTED_INTERVALS[1] seconds distracted = Distracted2
        else if timer >= DISTRACTED_INTERVALS[1] {
            next_state = State::Distracted2;
        }
        // DISTRACTED_INTERVALS[0] seconds distracted = Distracted1
        else if timer >= DISTRACTED_INTERVALS[0] {
            next_state = State::Distracted1;
        }
    } else {
        // Display Focused expressions at certain time intervals
        // At FOCUSED_INTERVALS[0] seconds focused = Focused1
        if timer >= FOCUSED_INTERVALS[0] && timer < FOCUSED_INTERVALS[0] + FOCUS_INTERVAL {
            next_state = State::Focused1;
        }
        // At FOCUSED_INTERVALS[1] seconds focused = Focused2
        else if timer >= FOCUSED_INTERVALS[1] && timer < FOCUSED_INTERVALS[1] + FOCUS_INTERVAL {
            next_state = State::Focused2;
        }
        // At FOCUSED_INTERVALS[2] seconds focused = Focused3
        else if timer >= FOCUSED_INTERVALS[2] && timer < FOCUSED_INTERVALS[2] + FOCUS_INTERVAL {
            next_state = State::Focused3;
        }
        // At FOCUSED_INTERVALS[3] seconds focused = Focused4
        else if timer >= FOCUSED_INTERVALS[3] && timer < FOCUSED_INTERVALS[3] + FOCUS_INTERVAL {
            next_state = State::Focused4;
        }
    }
    // Otherwise default is just idle (above)

    // Increment timer
    let current_timer = menhera.get_timer();
    menhera.update_timer(current_timer + 1);

    // Update Menhera state
    menhera.update_state(next_state);

    println!("State History Queue: {:?}", menhera.history);
    println!("Next State: {}", next_state as u8);

    Ok(next_state)
}

#[tauri::command]
async fn open_menhera(app: tauri::AppHandle) -> Result<(), String> {
    tauri::WebviewWindowBuilder::from_config(
        &app,
        &app.config().app.windows.get(1).unwrap().clone(),
    )
    .unwrap()
    .build()
    .unwrap();

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(Menhera::new()))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_macos_permissions::init())
        .invoke_handler(tauri::generate_handler![parse_image, open_menhera])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
