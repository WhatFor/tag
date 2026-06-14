use thiserror::Error;

pub mod wasm;

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("Storage unavailable: {0}")]
    Unavailable(String),
    #[error("Backend error: {0}")]
    Backend(String),
}

pub trait SaveStore: Send + Sync + 'static {
    fn read(&self, key: &str) -> Result<Option<String>, SaveError>;
    fn write(&self, key: &str, value: &str) -> Result<(), SaveError>;
    fn clear(&self, key: &str) -> Result<(), SaveError>;
}
