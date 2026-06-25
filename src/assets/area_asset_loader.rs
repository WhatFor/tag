use crate::prelude::*;
use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Deserialize)]
pub struct AreaData {
    pub id: String,
    pub name: String,
    pub content: AreaContent,
    pub exits: Vec<AreaExit>,
    pub is_checkpoint: bool,
}

#[derive(Default, TypePath)]
pub struct AreaAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AreaAssetLoaderError {
    #[error("Could not load area: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse area RON: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

impl AssetLoader for AreaAssetLoader {
    type Asset = AreaData;
    type Settings = ();
    type Error = AreaAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(ron::de::from_bytes(&bytes)?)
    }

    fn extensions(&self) -> &[&str] {
        &["area.ron"]
    }
}
