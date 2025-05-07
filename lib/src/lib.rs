mod icon;

use std::{collections::HashMap, str::FromStr, time::Duration};

use anyhow::{Context, Result};
use keepass::{
    DatabaseKey,
    db::{Database as KpDatabase, Group as KpGroup, Node, NodeRef, Value as KpValue},
};
use serde::{Deserialize, Serialize};

use tsify::Tsify;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct AppState {
    databases: Vec<Database>,
}

#[derive(Clone)]
struct Database {
    database: keepass::Database,
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

        Ok(Self { database })
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

    pub root: Group,
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Group {
    pub name: String,
    pub uuid: Uuid,
    pub children: Vec<Group>,
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Entry {
    pub name: Option<String>,
    pub uuid: Uuid,
    pub user_name: Option<String>,
    pub url: Option<String>,

    pub fields: HashMap<String, Value>,

    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Value {
    Bytes(Vec<u8>),
    Unprotected(String),
    Protected,
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct OTPResponse {
    pub code: String,
    pub valid_for: Duration,
    pub period: Duration,
}

impl Into<DatabaseOverview> for &Database {
    fn into(self) -> DatabaseOverview {
        DatabaseOverview {
            name: self.get_name().to_string(),
            root: (&self.database.root, &self.database).into(),
        }
    }
}

impl Into<Group> for (&KpGroup, &KpDatabase) {
    fn into(self) -> Group {
        let children: Vec<Group> = self
            .0
            .children
            .iter()
            .filter_map(|node| match node {
                keepass::db::Node::Group(group) => Some((group, self.1).into()),
                keepass::db::Node::Entry(..) => None,
            })
            .collect();

        let icon = icon::get_icon(&self.1, self.0.custom_icon_uuid.as_ref(), self.0.icon_id);

        Group {
            name: self.0.name.to_string(),
            uuid: self.0.uuid.clone(),
            children,
            icon,
        }
    }
}

impl Into<Value> for &KpValue {
    fn into(self) -> Value {
        match self {
            KpValue::Bytes(items) => Value::Bytes(items.to_owned()),
            KpValue::Unprotected(s) => Value::Unprotected(s.to_owned()),
            KpValue::Protected(..) => Value::Protected,
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

    /// Close a database
    pub fn close_database(&mut self, database_idx: usize) -> Result<(), String> {
        if database_idx >= self.databases.len() {
            return Err("No database by that index".to_string());
        }

        self.databases.remove(database_idx);

        Ok(())
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

                    let fields = entry
                        .fields
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.into()))
                        .collect();

                    let icon = icon::get_icon(
                        &db.database,
                        entry.custom_icon_uuid.as_ref(),
                        entry.icon_id,
                    );

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
            .collect()
    }

    /// Reveal a protected value within an entry, e.g. a password
    pub fn reveal_protected(
        &self,
        database_idx: usize,
        entry_uuid: &str,
        field_name: &str,
    ) -> Option<String> {
        let Ok(entry_uuid) = Uuid::from_str(&entry_uuid) else {
            return None;
        };

        let db = self.databases.get(database_idx)?;

        // find the appropriate containing group recursively
        let Some(NodeRef::Entry(entry)) = db.database.root.iter().find(|node| {
            if let NodeRef::Entry(e) = node {
                e.uuid == entry_uuid
            } else {
                false
            }
        }) else {
            return None;
        };

        let value = entry.fields.get(field_name)?;

        if let KpValue::Protected(v) = value {
            String::from_utf8(v.unsecure().to_vec()).ok()
        } else {
            None
        }
    }

    /// Get the current time-based one-time password (TOTP) for an entry
    pub fn get_otp(
        &self,
        database_idx: usize,
        entry_uuid: &str,
        time: u64,
    ) -> Result<OTPResponse, String> {
        let Ok(entry_uuid) = Uuid::from_str(&entry_uuid) else {
            return Err("Invalid entry UUID".into());
        };

        let db = self
            .databases
            .get(database_idx)
            .ok_or("No database with that index".to_string())?;

        // find the appropriate containing group recursively
        let Some(NodeRef::Entry(entry)) = db.database.root.iter().find(|node| {
            if let NodeRef::Entry(e) = node {
                e.uuid == entry_uuid
            } else {
                false
            }
        }) else {
            return Err("No otp field".into());
        };

        let value = entry.get_otp().map_err(|e| e.to_string())?.value_at(time);

        Ok(OTPResponse {
            code: value.code,
            valid_for: value.valid_for,
            period: value.period,
        })
    }
}
