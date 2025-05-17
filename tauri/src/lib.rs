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
fn load_demo(app: AppHandle, state: State<'_>) -> Result<DatabaseOverview, String> {
    let mut state = state.lock().unwrap();
    state.load_demo(app)
}

#[tauri::command]
async fn load_database(app: AppHandle, state: State<'_>) -> Result<DatabaseOverview, String> {
    let app = app.clone();

    let path = tauri::async_runtime::spawn_blocking(move || {
        app.dialog()
            .file()
            .add_filter("KeePass Databases", &["kdbx"])
            .blocking_pick_file()
            .and_then(|p| p.into_path().ok())
    })
    .await
    .map_err(|e| e.to_string())?;

    if let Some(path) = path {
        let mut state = state.lock().unwrap();
        return state.load_database_path(&path);
    }

    Err("Loading aborted".into())
}

#[tauri::command]
fn unlock_database(
    app: AppHandle,
    state: State<'_>,
    database_idx: usize,
    password: Option<String>,
    keyfile: Option<Vec<u8>>,
) -> Result<DatabaseOverview, String> {
    let mut state = state.lock().unwrap();
    state.unlock_database(database_idx, password, keyfile, app)
}

#[tauri::command]
fn lock_database(state: State<'_>, database_idx: usize) -> Result<DatabaseOverview, String> {
    let mut state = state.lock().unwrap();
    state.lock_database(database_idx)
}

#[tauri::command]
fn save_database(
    app: AppHandle,
    state: State<'_>,
    database_idx: usize,
) -> Result<Option<Vec<u8>>, String> {
    let mut state = state.lock().unwrap();
    state.save_database(database_idx, app)
}

#[tauri::command]
async fn save_database_as(
    app: AppHandle,
    state: State<'_>,
    database_idx: usize,
) -> Result<(), String> {
    let app_thread = app.clone();

    let path = tauri::async_runtime::spawn_blocking(move || {
        app_thread
            .dialog()
            .file()
            .add_filter("KeePass Databases", &["kdbx"])
            .blocking_save_file()
    })
    .await
    .map_err(|e| e.to_string())?;

    if let Some(path) = path {
        let mut state = state.lock().unwrap();
        state.save_database_as(database_idx, path, app)?;

        return Ok(());
    }

    Err("Loading aborted".into())
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
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            list_databases,
            load_demo,
            load_database,
            unlock_database,
            lock_database,
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
