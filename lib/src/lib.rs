use anyhow::{Context, Result};
use keepass::DatabaseKey;
use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct AppState {
    counter: i32,

    databases: Vec<Database>,
}

#[derive(Clone)]
struct Database {
    database: keepass::Database,
    key: keepass::DatabaseKey,
}

impl Database {
    fn load(
        data: &mut dyn std::io::Read,
        password: Option<String>,
        keyfile: Option<Vec<u8>>,
    ) -> Result<Self> {
        let mut key = DatabaseKey::new();

        if let Some(p) = password {
            key = key.with_password(&p);
        }

        if let Some(kf) = keyfile {
            key = key.with_keyfile(&mut &kf[..]).context("Reading keyfile")?;
        }

        let database = keepass::Database::open(data, key.clone()).context("Opening database")?;

        Ok(Self { database, key })
    }

    fn save(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();

        self.database
            .save(&mut buffer, self.key.clone())
            .context("Writing database")?;

        Ok(buffer)
    }

    fn get_name(&self) -> &str {
        self.database
            .meta
            .database_name
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or_else(|| self.database.root.get_name())
    }
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseOverview {
    pub name: String,
}

impl Into<DatabaseOverview> for &Database {
    fn into(self) -> DatabaseOverview {
        DatabaseOverview {
            name: self.get_name().to_string(),
        }
    }
}

#[cfg_attr(not(feature = "tauri"), wasm_bindgen)]
impl AppState {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn greet(&self, name: &str) -> String {
        format!("Hello, {}! the counter is {}", name, self.counter)
    }

    pub fn increment(&mut self) {
        self.counter += 1;
    }

    pub fn decrement(&mut self) {
        self.counter -= 1;
    }

    #[cfg_attr(not(feature = "tauri"), wasm_bindgen(js_name = "list_databases"))]
    pub fn list_databases_wasm(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(
            &self
                .databases
                .iter()
                .map(|db| db.into())
                .collect::<Vec<DatabaseOverview>>(),
        )
    }

    #[cfg(feature = "tauri")]
    pub fn list_databases(&self) -> Vec<DatabaseOverview> {
        self.databases.iter().map(|db| db.into()).collect()
    }

    #[cfg_attr(not(feature = "tauri"), wasm_bindgen(js_name = "load_database"))]
    pub fn load_database_wasm(
        &mut self,
        mut data: &[u8],
        password: Option<String>,
        keyfile: Option<Vec<u8>>,
    ) -> std::result::Result<JsValue, serde_wasm_bindgen::Error> {
        let db = Database::load(&mut data, password, keyfile)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let res: DatabaseOverview = (&db).into();
        self.databases.push(db);

        serde_wasm_bindgen::to_value(&res)
    }

    #[cfg(feature = "tauri")]
    pub fn load_database(
        &mut self,
        mut data: &[u8],
        password: Option<String>,
        keyfile: Option<Vec<u8>>,
    ) -> Result<DatabaseOverview, String> {
        let db = Database::load(&mut data, password, keyfile).map_err(|e| format!("{}", e))?;

        let res: DatabaseOverview = (&db).into();
        self.databases.push(db);

        Ok(res)
    }
}
