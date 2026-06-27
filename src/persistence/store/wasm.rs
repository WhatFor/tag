#![cfg(target_arch = "wasm32")]

use crate::persistence::store::{SaveError, SaveStore};

use web_sys::window;

pub struct LocalStorageStore;

impl LocalStorageStore {
    pub fn new() -> Self {
        Self
    }

    fn storage(&self) -> Result<web_sys::Storage, SaveError> {
        window()
            .ok_or_else(|| SaveError::Unavailable("no window".into()))?
            .local_storage()
            .map_err(|e| SaveError::Backend(format!("{e:?}")))?
            .ok_or_else(|| SaveError::Unavailable("localStorage disabled".into()))
    }
}

impl SaveStore for LocalStorageStore {
    fn exists(&self, key: &str) -> Result<bool, SaveError> {
        self.storage()?
            .get_item(key)
            .map(|opt| opt.is_some())
            .map_err(|e| SaveError::Backend(format!("{e:?}")))
    }

    fn read(&self, key: &str) -> Result<Option<String>, super::SaveError> {
        self.storage()?
            .get_item(key)
            .map_err(|e| SaveError::Backend(format!("{e:?}")))
    }

    fn write(&self, key: &str, value: &str) -> Result<(), super::SaveError> {
        self.storage()?
            .set_item(key, value)
            .map_err(|e| SaveError::Backend(format!("{e:?}")))
    }

    fn clear(&self, key: &str) -> Result<(), SaveError> {
        self.storage()?
            .remove_item(key)
            .map_err(|e| SaveError::Backend(format!("{e:?}")))
    }

    fn keys(&self) -> Result<Vec<String>, SaveError> {
        let storage = self.storage()?;
        let len = storage
            .length()
            .map_err(|e| SaveError::Backend(format!("{e:?}")))?;

        let mut keys = Vec::with_capacity(len as usize);
        for i in 0..len {
            if let Some(key) = storage
                .key(i)
                .map_err(|e| SaveError::Backend(format!("{e:?}")))?
            {
                keys.push(key);
            }
        }
        Ok(keys)
    }
}
