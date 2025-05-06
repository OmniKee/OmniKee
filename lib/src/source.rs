use std::io::{Cursor, Read, Write};

use anyhow::Result;

/// A way to load a KeePass database
///
/// This is to abstract over whether the database is loaded from a Vec<u8> buffer or a path into
/// the filesystem.
pub trait DatabaseSource: Send {
    /// Get a read handle from the database source
    fn open(&self) -> Result<Box<dyn Read>>;

    /// Get a write handle for the database source
    fn save(&mut self) -> Result<Box<dyn Write + '_>>;

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
    fn open(&self) -> Result<Box<dyn Read>> {
        Ok(Box::new(Cursor::new(self.buffer.clone())))
    }

    fn save(&mut self) -> Result<Box<dyn Write + '_>> {
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
    pub path: std::path::PathBuf,
}

#[cfg(feature = "tauri")]
impl DatabaseSource for FilesystemDatabaseSource {
    fn open(&self) -> Result<Box<dyn Read>> {
        use std::fs::File;
        Ok(Box::new(File::open(&self.path)?))
    }

    fn save(&mut self) -> Result<Box<dyn Write + '_>> {
        use std::fs::File;
        Ok(Box::new(File::create(&self.path)?))
    }

    fn send_saved(&self) -> Option<Vec<u8>> {
        None
    }

    fn get_name(&self) -> &str {
        self.path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    }
}
