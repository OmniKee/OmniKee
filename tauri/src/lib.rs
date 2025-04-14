use std::sync::Mutex;

use omnikee_lib::AppState;
use tauri::Manager;

type State<'a> = tauri::State<'a, Mutex<AppState>>;

#[tauri::command]
fn greet(name: &str, state: State<'_>) -> String {
    let state = state.lock().unwrap();
    state.greet(name)
}

#[tauri::command]
fn increment(state: State<'_>) {
    let mut state = state.lock().unwrap();
    state.increment();
}

#[tauri::command]
fn decrement(state: State<'_>) {
    let mut state = state.lock().unwrap();
    state.decrement();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state: AppState = Default::default();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, increment, decrement])
        .setup(|app| {
            app.manage(Mutex::new(state));

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
