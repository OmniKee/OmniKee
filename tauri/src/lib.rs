use std::sync::Mutex;

use omnikee_lib::{AppState, DatabaseOverview, Entry, OTPResponse};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

type State<'a> = tauri::State<'a, Mutex<AppState>>;

#[tauri::command]
fn list_databases(state: State<'_>) -> Vec<DatabaseOverview> {
    let state = state.lock().unwrap();
    state.list_databases()
}

#[tauri::command]
fn load_demo(state: State<'_>) -> Result<DatabaseOverview, String> {
    let mut state = state.lock().unwrap();
    state.load_demo()
}

#[tauri::command]
fn load_database(app: AppHandle, state: State<'_>) -> Result<DatabaseOverview, String> {
    let mut state = state.lock().unwrap();
    let path = app
        .dialog()
        .file()
        .add_filter("KeePass Databases", &["kdbx"])
        .blocking_pick_file()
        .and_then(|p| p.into_path().ok());

    if let Some(path) = path {
        return state.load_database_path(&path);
    }

    Err("Loading aborted".into())
}

#[tauri::command]
fn save_database(state: State<'_>, database_idx: usize) -> Result<Option<Vec<u8>>, String> {
    let mut state = state.lock().unwrap();
    state.save_database(database_idx)
}

#[tauri::command]
fn save_database_as(app: AppHandle, state: State<'_>, database_idx: usize) -> Result<(), String> {
    let mut state = state.lock().unwrap();

    let path = app
        .dialog()
        .file()
        .add_filter("KeePass Databases", &["kdbx"])
        .blocking_save_file()
        .and_then(|p| p.into_path().ok());

    if let Some(path) = path {
        state.save_database_as(database_idx, &path)?;
    }

    Ok(())
}

#[tauri::command]
fn close_database(state: State<'_>, database_idx: usize) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    state.close_database(database_idx)
}

#[tauri::command]
fn list_entries(
    state: State<'_>,
    database_idx: usize,
    group_uuid: String,
) -> Result<Vec<Entry>, String> {
    let state = state.lock().unwrap();
    state.list_entries(database_idx, group_uuid)
}

#[tauri::command]
fn reveal_protected(
    state: State<'_>,
    database_idx: usize,
    entry_uuid: String,
    field_name: String,
) -> Result<String, String> {
    let state = state.lock().unwrap();
    state.reveal_protected(database_idx, &entry_uuid, &field_name)
}

#[tauri::command]
fn get_otp(
    state: State<'_>,
    database_idx: usize,
    entry_uuid: String,
    time: u64,
) -> Result<OTPResponse, String> {
    let state = state.lock().unwrap();
    state.get_otp(database_idx, &entry_uuid, time)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state: AppState = Default::default();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_databases,
            load_demo,
            load_database,
            save_database,
            save_database_as,
            close_database,
            list_entries,
            reveal_protected,
            get_otp,
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
