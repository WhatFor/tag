use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Deserialize)]
pub struct ItemData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub stackable: bool,
    pub icon: String,
}

#[derive(Default, TypePath)]
pub struct ItemAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ItemAssetLoaderError {
    #[error("Could not load item: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse item RON: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

impl AssetLoader for ItemAssetLoader {
    type Asset = ItemData;
    type Settings = ();
    type Error = ItemAssetLoaderError;

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
        &["item.ron"]
    }
}
