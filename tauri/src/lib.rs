use std::sync::Mutex;

use omnikee_lib::{AppState, DatabaseOverview, Entry};
use tauri::Manager;

type State<'a> = tauri::State<'a, Mutex<AppState>>;

#[tauri::command]
fn list_databases(state: State<'_>) -> Vec<DatabaseOverview> {
    let state = state.lock().unwrap();
    state.list_databases()
}

#[tauri::command]
fn load_database(
    state: State<'_>,
    data: Vec<u8>,
    password: Option<String>,
    keyfile: Option<Vec<u8>>,
) -> Result<DatabaseOverview, String> {
    let mut state = state.lock().unwrap();
    state.load_database(&data[..], password, keyfile)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state: AppState = Default::default();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_databases,
            load_database,
        ])
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
