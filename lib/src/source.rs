use std::io::{Cursor, Read, Write};

use anyhow::Result;

/// A way to load a KeePass database
///
/// This is to abstract over whether the database is loaded from a Vec<u8> buffer or a path into
/// the filesystem.
pub trait DatabaseSource: Send {
    /// Get a read handle from the database source
    fn open(&self, #[cfg(feature = "tauri")] app: tauri::AppHandle) -> Result<Box<dyn Read>>;

    /// Get a write handle for the database source
    fn save(
        &mut self,
        #[cfg(feature = "tauri")] app: tauri::AppHandle,
    ) -> Result<Box<dyn Write + '_>>;

    /// If applicable, send the internal data as a buffer
    fn send_saved(&self) -> Option<Vec<u8>>;

    /// Get a name describing the source (like a file name)
    fn get_name(&self) -> &str;
}

/// A database loaded from an in-memory buffer (for web deploys)
pub struct BufferDatabaseSource {
    pub name: String,
    pub buffer: Vec<u8>,
}

impl DatabaseSource for BufferDatabaseSource {
    fn open(&self, #[cfg(feature = "tauri")] _: tauri::AppHandle) -> Result<Box<dyn Read>> {
        Ok(Box::new(Cursor::new(self.buffer.clone())))
    }

    fn save(
        &mut self,
        #[cfg(feature = "tauri")] _: tauri::AppHandle,
    ) -> Result<Box<dyn Write + '_>> {
        self.buffer.clear();
        Ok(Box::new(&mut self.buffer))
    }

    fn send_saved(&self) -> Option<Vec<u8>> {
        Some(self.buffer.clone())
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

/// A database loaded from the filesystem (for Tauri deploys)
#[cfg(feature = "tauri")]
pub struct FilesystemDatabaseSource {
    pub path: tauri_plugin_fs::FilePath,
}

#[cfg(feature = "tauri")]
impl DatabaseSource for FilesystemDatabaseSource {
    fn open(&self, app: tauri::AppHandle) -> Result<Box<dyn Read>> {
        use tauri_plugin_fs::{FsExt, OpenOptions};

        let options = OpenOptions::new().read(true).clone();

        Ok(Box::new(app.fs().open(self.path.clone(), options)?))
    }

    fn save(&mut self, app: tauri::AppHandle) -> Result<Box<dyn Write>> {
        use tauri_plugin_fs::{FsExt, OpenOptions};

        let options = OpenOptions::new().write(true).create(true).clone();

        Ok(Box::new(app.fs().open(self.path.clone(), options)?))
    }

    fn send_saved(&self) -> Option<Vec<u8>> {
        None
    }

    fn get_name(&self) -> &str {
        match &self.path {
            tauri_plugin_fs::FilePath::Url(url) => url
                .path_segments()
                .and_then(|segments| segments.last())
                .unwrap_or_default(),
            tauri_plugin_fs::FilePath::Path(path_buf) => path_buf
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default(),
        }
    }
}
