use crate::persistence::store::SaveStore;

#[cfg(target_arch = "wasm32")]
use web_sys::window;

pub struct LocalStorageStore;

impl LocalStorageStore {
    pub fn new() -> Self {
        Self
    }
}

impl SaveStore for LocalStorageStore {
    fn read(&self, key: &str) -> Result<Option<String>, super::SaveError> {
        todo!()
    }

    fn write(&self, key: &str, value: &str) -> Result<(), super::SaveError> {
        todo!()
    }

    fn clear(&self, key: &str) -> Result<(), super::SaveError> {
        todo!()
    }
}
