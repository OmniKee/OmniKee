use std::str::FromStr;

use anyhow::{Context, Result};
use keepass::{
    DatabaseKey,
    db::{Group, Node, NodeRef},
};
use serde::{Deserialize, Serialize};

use tsify::Tsify;
use uuid::Uuid;
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

        let database = keepass::Database::open(data, key.clone())?;

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

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DatabaseOverview {
    pub name: String,

    pub root: Folder,
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Folder {
    pub name: String,
    pub uuid: Uuid,
    pub children: Vec<Folder>,
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Entry {
    pub name: Option<String>,
    pub uuid: Uuid,
    pub user_name: Option<String>,
    pub url: Option<String>,
}

impl Into<DatabaseOverview> for &Database {
    fn into(self) -> DatabaseOverview {
        DatabaseOverview {
            name: self.get_name().to_string(),
            root: (&self.database.root).into(),
        }
    }
}

impl Into<Folder> for &Group {
    fn into(self) -> Folder {
        let children: Vec<Folder> = self
            .children
            .iter()
            .filter_map(|node| match node {
                keepass::db::Node::Group(group) => Some(group.into()),
                keepass::db::Node::Entry(..) => None,
            })
            .collect();

        Folder {
            name: self.name.to_string(),
            uuid: self.uuid.clone(),
            children,
        }
    }
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

    pub fn greet(&self, name: &str) -> String {
        format!("Hello, {}! the counter is {}", name, self.counter)
    }

    pub fn increment(&mut self) {
        self.counter += 1;
    }

    pub fn decrement(&mut self) {
        self.counter -= 1;
    }

    /// List databases that are currently loaded in the AppState
    pub fn list_databases(&self) -> Vec<DatabaseOverview> {
        self.databases.iter().map(|db| db.into()).collect()
    }

    /// Load a new database from a buffer
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

    /// List the entries directly contained within a group of a database
    pub fn list_entries(&self, database_idx: usize, group_uuid: String) -> Vec<Entry> {
        let Ok(group_uuid) = Uuid::from_str(&group_uuid) else {
            return Vec::new();
        };

        let Some(db) = self.databases.get(database_idx) else {
            return Vec::new();
        };

        // find the appropriate containing group recursively
        let Some(NodeRef::Group(group)) = db.database.root.iter().find(|node| {
            if let NodeRef::Group(g) = node {
                g.uuid == group_uuid
            } else {
                false
            }
        }) else {
            return Vec::new();
        };

        group
            .children
            .iter()
            .filter_map(|node| match node {
                Node::Entry(entry) => {
                    let name = entry.get_title().map(String::from);
                    let uuid = entry.uuid.clone();
                    let user_name = entry.get_username().map(String::from);
                    let url = entry.get_url().map(String::from);

                    Some(Entry {
                        name,
                        uuid,
                        user_name,
                        url,
                    })
                }
                Node::Group(..) => None,
            })
            .collect()
    }
}
