use anyhow::{Context, Result, bail};

use keepass::DatabaseKey as KpDatabaseKey;
use keepass::db::{Database as KpDatabase, Entry as KpEntry, Group as KpGroup, NodeRef};
use uuid::Uuid;

use crate::source::DatabaseSource;

/// The state of an in-memory database
pub(crate) enum DatabaseState {
    /// an unlocked database with access to its internal data
    Unlocked {
        database: KpDatabase,
        key: KpDatabaseKey,
    },

    /// a marker that the database has not yet been unlocked
    Locked,
}

/// Internal state for an in-memory database
pub(crate) struct Database {
    pub(crate) state: DatabaseState,
    pub(crate) source: Box<dyn DatabaseSource>,
}

impl Database {
    /// Load a database source, but don't unlock the database yet
    pub(crate) fn load<S: DatabaseSource + 'static>(source: S) -> Result<Self> {
        Ok(Self {
            state: DatabaseState::Locked,
            source: Box::new(source),
        })
    }

    /// Unlock a loaded database
    pub(crate) fn unlock(
        &mut self,
        password: Option<String>,
        keyfile: Option<Vec<u8>>,

        #[cfg(feature = "tauri")] app: tauri::AppHandle,
    ) -> Result<()> {
        let mut key = KpDatabaseKey::new();

        if let Some(p) = password {
            key = key.with_password(&p);
        }

        if let Some(kf) = keyfile {
            key = key.with_keyfile(&mut &kf[..]).context("Reading keyfile")?;
        }

        let mut reader = self.source.open(
            #[cfg(feature = "tauri")]
            app,
        )?;

        let database = KpDatabase::open(&mut reader, key.clone())?;

        self.state = DatabaseState::Unlocked { database, key };

        Ok(())
    }

    /// Lock a database, discarding its key and unlocked data
    pub(crate) fn lock(&mut self) {
        self.state = DatabaseState::Locked
    }

    /// Save the database contents to its source
    pub(crate) fn save(
        &mut self,
        #[cfg(feature = "tauri")] app: tauri::AppHandle,
    ) -> Result<Option<Vec<u8>>> {
        if let DatabaseState::Unlocked { database, key } = &self.state {
            {
                let mut writer = self.source.save(
                    #[cfg(feature = "tauri")]
                    app,
                )?;

                database.save(&mut writer, key.clone())?;
            }
            Ok(self.source.send_saved())
        } else {
            bail!("Database must be open to save")
        }
    }

    /// Convenience method to get an appropriate name for a database
    pub(crate) fn get_name(&self) -> &str {
        match &self.state {
            DatabaseState::Unlocked { database, .. } => database
                .meta
                .database_name
                .as_ref()
                .map(|s| &s[..])
                .unwrap_or_else(|| database.root.get_name()),
            DatabaseState::Locked => self.source.get_name(),
        }
    }

    /// Convenience method to get inside of an unlocked database
    pub(crate) fn get_database(&self) -> Result<&KpDatabase> {
        if let DatabaseState::Unlocked { database, .. } = &self.state {
            Ok(database)
        } else {
            bail!("Cannot access - database is locked")
        }
    }

    /// Convenience to get an icon
    pub(crate) fn get_icon(
        &self,
        uuid: Option<&Uuid>,
        index: Option<usize>,
    ) -> Result<Option<String>> {
        let database = self.get_database()?;
        let icon = crate::icon::get_icon(database, uuid, index);

        Ok(icon)
    }

    /// Find a group by its UUID
    pub(crate) fn find_group(&self, uuid: &Uuid) -> Result<Option<&KpGroup>> {
        let database = self.get_database()?;

        for node in database.root.iter() {
            if let NodeRef::Group(group) = node {
                if &group.uuid == uuid {
                    return Ok(Some(group));
                }
            }
        }

        Ok(None)
    }

    /// Find an entry by its UUID
    pub(crate) fn find_entry(&self, uuid: &Uuid) -> Result<Option<&KpEntry>> {
        let database = self.get_database()?;

        for node in database.root.iter() {
            if let NodeRef::Entry(entry) = node {
                if &entry.uuid == uuid {
                    return Ok(Some(entry));
                }
            }
        }

        Ok(None)
    }
}
