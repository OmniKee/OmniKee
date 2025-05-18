//! External data access types
//!
//! These types are return types for the frontend application

use std::{collections::HashMap, time::Duration};

use keepass::db::{Database as KpDatabase, Group as KpGroup, Value as KpValue};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use crate::database::{Database, DatabaseState};

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "state")]
pub enum DatabaseOverview {
    Unlocked {
        file_name: String,
        name: String,
        root: Group,
    },
    Locked {
        file_name: String,
        name: String,
    },
}

impl Into<DatabaseOverview> for &Database {
    fn into(self) -> DatabaseOverview {
        match &self.state {
            DatabaseState::Locked => DatabaseOverview::Locked {
                file_name: self.source.get_name().to_string(),
                name: self.get_name().to_string(),
            },
            DatabaseState::Unlocked { database, .. } => DatabaseOverview::Unlocked {
                file_name: self.source.get_name().to_string(),
                name: self.get_name().to_string(),
                root: (&database.root, database).into(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Group {
    pub name: String,
    pub uuid: Uuid,
    pub children: Vec<Group>,
    pub icon: Option<String>,
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

        let icon = crate::icon::get_icon(&self.1, self.0.custom_icon_uuid.as_ref(), self.0.icon_id);

        Group {
            name: self.0.name.to_string(),
            uuid: self.0.uuid.clone(),
            children,
            icon,
        }
    }
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

impl Into<Value> for &KpValue {
    fn into(self) -> Value {
        match self {
            KpValue::Bytes(items) => Value::Bytes(items.to_owned()),
            KpValue::Unprotected(s) => Value::Unprotected(s.to_owned()),
            KpValue::Protected(..) => Value::Protected,
        }
    }
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum ValueSet {
    Bytes(Vec<u8>),
    Unprotected(String),
    Protected(String),
}

impl Into<KpValue> for ValueSet {
    fn into(self) -> KpValue {
        match self {
            ValueSet::Bytes(buffer) => KpValue::Bytes(buffer),
            ValueSet::Unprotected(value) => KpValue::Unprotected(value),
            ValueSet::Protected(value) => KpValue::Protected(value.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct OTPResponse {
    pub code: String,
    pub valid_for: Duration,
    pub period: Duration,
}
