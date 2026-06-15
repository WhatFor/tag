use thiserror::Error;

pub mod fs;
pub mod wasm;

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("Storage unavailable: {0}")]
    #[allow(dead_code)]
    Unavailable(String),
    #[error("Backend error: {0}")]
    Backend(String),
}

pub trait SaveStore: Send + Sync + 'static {
    fn exists(&self, key: &str) -> Result<bool, SaveError>;
    fn read(&self, key: &str) -> Result<Option<String>, SaveError>;
    fn write(&self, key: &str, value: &str) -> Result<(), SaveError>;
    fn clear(&self, key: &str) -> Result<(), SaveError>;
}
