#![cfg(not(target_arch = "wasm32"))]

use std::fs;
use std::path::PathBuf;

use crate::persistence::store::{SaveError, SaveStore};

const APP_DIR: &str = "tag";
const FILE_EXT: &str = "ron";

pub struct FileSystemStorageStore {
    base: PathBuf,
}

impl FileSystemStorageStore {
    pub fn new() -> Self {
        let base = dirs::data_dir()
            .unwrap_or_else(std::env::temp_dir)
            .join(APP_DIR);

        Self { base }
    }

    fn path_for(&self, key: &str) -> PathBuf {
        self.base.join(format!("{key}.{FILE_EXT}"))
    }
}

impl SaveStore for FileSystemStorageStore {
    fn exists(&self, key: &str) -> Result<bool, SaveError> {
        let path = self.path_for(key);

        fs::exists(&path).map_err(|e| SaveError::Backend(format!("write {path:?}: {e}")))
    }

    fn read(&self, key: &str) -> Result<Option<String>, SaveError> {
        let path = self.path_for(key);
        match fs::read_to_string(&path) {
            Ok(s) => Ok(Some(s)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(SaveError::Backend(format!("read {path:?}: {e}"))),
        }
    }

    fn write(&self, key: &str, value: &str) -> Result<(), SaveError> {
        fs::create_dir_all(&self.base)
            .map_err(|e| SaveError::Backend(format!("mkdir {:?}: {e}", self.base)))?;

        let path = self.path_for(key);

        fs::write(&path, value).map_err(|e| SaveError::Backend(format!("write {path:?}: {e}")))
    }

    fn clear(&self, key: &str) -> Result<(), SaveError> {
        let path = self.path_for(key);

        match std::fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(SaveError::Backend(format!("remove {path:?}: {e}"))),
        }
    }

    fn keys(&self) -> Result<Vec<String>, SaveError> {
        let entries = match fs::read_dir(&self.base) {
            Ok(entries) => entries,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(SaveError::Backend(format!("read_dir {:?}: {e}", self.base))),
        };

        let mut keys = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| SaveError::Backend(format!("read_dir entry: {e}")))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some(FILE_EXT) {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    keys.push(stem.to_string());
                }
            }
        }
        Ok(keys)
    }
}
