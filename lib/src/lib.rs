mod icon;

mod database;
mod exchange;
mod source;

use std::str::FromStr;

#[cfg(feature = "tauri")]
use std::path::Path;

use anyhow::Result;
use keepass::db::{Node, Value as KpValue};

use uuid::Uuid;
use wasm_bindgen::prelude::*;

use crate::database::Database;

pub use crate::exchange::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct AppState {
    databases: Vec<Database>,
}

#[cfg_attr(not(feature = "tauri"), wasm_bindgen)]
impl AppState {
    pub fn new() -> Self {
        #[cfg(not(feature = "tauri"))]
        {
            console_error_panic_hook::set_once()
        }

        Default::default()
    }

    /// List databases that are currently loaded in the AppState
    pub fn list_databases(&self) -> Vec<DatabaseOverview> {
        self.databases.iter().map(|db| db.into()).collect()
    }

    /// Load and unlock a demo database
    #[cfg(not(feature = "tauri"))]
    pub fn load_demo(&mut self) -> Result<DatabaseOverview, String> {
        let data = include_bytes!("demo.kdbx");

        let mut db = Database::load(crate::source::BufferDatabaseSource {
            name: "demo.kdbx".to_string(),
            buffer: data.to_vec(),
        })
        .map_err(|e| format!("{}", e))?;

        db.unlock(Some("demopass".to_string()), None)
            .map_err(|e| format!("{}", e))?;

        let res: DatabaseOverview = (&db).into();
        self.databases.push(db);

        Ok(res)
    }

    /// Load and unlock a demo database
    #[cfg(feature = "tauri")]
    pub fn load_demo(&mut self, app: tauri::AppHandle) -> Result<DatabaseOverview, String> {
        let data = include_bytes!("demo.kdbx");

        let mut db = Database::load(crate::source::BufferDatabaseSource {
            name: "demo.kdbx".to_string(),
            buffer: data.to_vec(),
        })
        .map_err(|e| format!("{}", e))?;

        db.unlock(Some("demopass".to_string()), None, app)
            .map_err(|e| format!("{}", e))?;

        let res: DatabaseOverview = (&db).into();
        self.databases.push(db);

        Ok(res)
    }

    /// Load a new database from a buffer
    pub fn load_database_buffer(
        &mut self,
        name: String,
        data: &[u8],
    ) -> Result<DatabaseOverview, String> {
        let db = Database::load(crate::source::BufferDatabaseSource {
            name,
            buffer: data.to_vec(),
        })
        .map_err(|e| format!("{}", e))?;

        let res: DatabaseOverview = (&db).into();
        self.databases.push(db);

        Ok(res)
    }

    /// Load a new database from a filesystem path
    #[cfg(feature = "tauri")]
    pub fn load_database_path(&mut self, path: &Path) -> Result<DatabaseOverview, String> {
        let db = Database::load(crate::source::FilesystemDatabaseSource {
            path: tauri_plugin_fs::FilePath::Path(path.to_path_buf()),
        })
        .map_err(|e| format!("{}", e))?;

        let res: DatabaseOverview = (&db).into();
        self.databases.push(db);

        Ok(res)
    }

    /// Unlock a loaded database
    #[cfg(not(feature = "tauri"))]
    pub fn unlock_database(
        &mut self,
        database_idx: usize,
        password: Option<String>,
        keyfile: Option<Vec<u8>>,
    ) -> Result<DatabaseOverview, String> {
        let Some(db) = self.databases.get_mut(database_idx) else {
            return Err("No database by that index".to_string());
        };

        db.unlock(password, keyfile).map_err(|e| format!("{}", e))?;

        Ok((&*db).into())
    }

    /// Unlock a loaded database
    #[cfg(feature = "tauri")]
    pub fn unlock_database(
        &mut self,
        database_idx: usize,
        password: Option<String>,
        keyfile: Option<Vec<u8>>,
        app: tauri::AppHandle,
    ) -> Result<DatabaseOverview, String> {
        let Some(db) = self.databases.get_mut(database_idx) else {
            return Err("No database by that index".to_string());
        };

        db.unlock(password, keyfile, app)
            .map_err(|e| format!("{}", e))?;

        Ok((&*db).into())
    }

    /// Lock a loaded database
    pub fn lock_database(&mut self, database_idx: usize) -> Result<DatabaseOverview, String> {
        let Some(db) = self.databases.get_mut(database_idx) else {
            return Err("No database by that index".to_string());
        };

        db.lock();
        Ok((&*db).into())
    }

    /// Save a database to the same path it was loaded from
    #[cfg(not(feature = "tauri"))]
    pub fn save_database(&mut self, database_idx: usize) -> Result<Option<Vec<u8>>, String> {
        let Some(db) = self.databases.get_mut(database_idx) else {
            return Err("No database by that index".to_string());
        };

        db.save().map_err(|e| format!("{}", e))
    }

    /// Save a database to the same path it was loaded from
    #[cfg(feature = "tauri")]
    pub fn save_database(
        &mut self,
        database_idx: usize,
        app: tauri::AppHandle,
    ) -> Result<Option<Vec<u8>>, String> {
        let Some(db) = self.databases.get_mut(database_idx) else {
            return Err("No database by that index".to_string());
        };

        db.save(app).map_err(|e| format!("{}", e))
    }

    /// Save a database to a specified destination
    #[cfg(feature = "tauri")]
    pub fn save_database_as(
        &mut self,
        database_idx: usize,
        path: tauri_plugin_fs::FilePath,
        app: tauri::AppHandle,
    ) -> Result<Option<Vec<u8>>, String> {
        let Some(db) = self.databases.get_mut(database_idx) else {
            return Err("No database by that index".to_string());
        };

        db.source = Box::new(crate::source::FilesystemDatabaseSource { path });

        db.save(app).map_err(|e| format!("{}", e))
    }

    /// Close a database
    pub fn close_database(&mut self, database_idx: usize) -> Result<(), String> {
        if database_idx >= self.databases.len() {
            return Err("No database by that index".to_string());
        }

        self.databases.remove(database_idx);

        Ok(())
    }

    /// List the entries directly contained within a group of a database
    pub fn list_entries(
        &self,
        database_idx: usize,
        group_uuid: String,
    ) -> Result<Vec<Entry>, String> {
        let group_uuid = Uuid::from_str(&group_uuid).map_err(|e| format!("{}", e))?;

        let database = self
            .databases
            .get(database_idx)
            .ok_or("Cannot get database by that index".to_string())?;

        let group = database
            .group(&group_uuid)
            .map_err(|e| format!("{}", e))?
            .ok_or("Group not found by UUID".to_string())?;

        let out = group
            .children
            .iter()
            .filter_map(|node| match node {
                Node::Entry(entry) => {
                    let name = entry.get_title().map(String::from);
                    let uuid = entry.uuid.clone();
                    let user_name = entry.get_username().map(String::from);
                    let url = entry.get_url().map(String::from);

                    let fields = entry
                        .fields
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.into()))
                        .collect();

                    let icon = database
                        .get_icon(entry.custom_icon_uuid.as_ref(), entry.icon_id)
                        .unwrap_or_default();

                    Some(Entry {
                        name,
                        uuid,
                        user_name,
                        url,
                        fields,
                        icon,
                    })
                }
                Node::Group(..) => None,
            })
            .collect();

        Ok(out)
    }

    /// Reveal a protected value within an entry, e.g. a password
    pub fn reveal_protected(
        &self,
        database_idx: usize,
        entry_uuid: &str,
        field_name: &str,
    ) -> Result<String, String> {
        let entry_uuid = Uuid::from_str(&entry_uuid).map_err(|e| format!("{}", e))?;

        let database = self
            .databases
            .get(database_idx)
            .ok_or("Cannot get database by that index".to_string())?;

        let entry = database
            .entry(&entry_uuid)
            .map_err(|e| format!("{}", e))?
            .ok_or("No entry by that UUID".to_string())?;

        let value = entry
            .fields
            .get(field_name)
            .ok_or("Cannot find a field with that name".to_string())?;

        if let KpValue::Protected(v) = value {
            String::from_utf8(v.unsecure().to_vec()).map_err(|e| format!("{}", e))
        } else {
            Err("The field is not protected".to_string())
        }
    }

    /// Set the name of a group
    pub fn set_group_name(
        &mut self,
        database_idx: usize,
        group_uuid: &str,
        name: String,
    ) -> Result<(), String> {
        let group_uuid = Uuid::from_str(&group_uuid).map_err(|e| format!("{}", e))?;

        let database = self
            .databases
            .get_mut(database_idx)
            .ok_or("Cannot get database by that index".to_string())?;

        let group = database
            .group_mut(&group_uuid)
            .map_err(|e| format!("{}", e))?
            .ok_or("No entry by that UUID".to_string())?;

        group.name = name;

        Ok(())
    }

    /// Set the value of a field within an entry
    pub fn set_field(
        &mut self,
        database_idx: usize,
        entry_uuid: &str,
        field_name: String,
        value: ValueSet,
    ) -> Result<(), String> {
        let entry_uuid = Uuid::from_str(&entry_uuid).map_err(|e| format!("{}", e))?;

        let database = self
            .databases
            .get_mut(database_idx)
            .ok_or("Cannot get database by that index".to_string())?;

        let entry = database
            .entry_mut(&entry_uuid)
            .map_err(|e| format!("{}", e))?
            .ok_or("No entry by that UUID".to_string())?;

        entry.fields.insert(field_name, value.into());

        Ok(())
    }

    /// Get the current time-based one-time password (TOTP) for an entry
    pub fn get_otp(
        &self,
        database_idx: usize,
        entry_uuid: &str,
        time: u64,
    ) -> Result<OTPResponse, String> {
        let entry_uuid = Uuid::from_str(&entry_uuid).map_err(|e| format!("{}", e))?;

        let database = self
            .databases
            .get(database_idx)
            .ok_or("Cannot get database by that index".to_string())?;

        let entry = database
            .entry(&entry_uuid)
            .map_err(|e| format!("{}", e))?
            .ok_or("No entry by that UUID".to_string())?;

        let value = entry.get_otp().map_err(|e| e.to_string())?.value_at(time);

        Ok(OTPResponse {
            code: value.code,
            valid_for: value.valid_for,
            period: value.period,
        })
    }
}
