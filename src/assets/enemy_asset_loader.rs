use crate::prelude::*;
use bevy::prelude::*;

use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Deserialize)]
pub struct EnemyData {
    pub id: String,
    pub display_name: String,
    pub max_health: i32,
    pub stats: Stats,
    pub moves: Vec<EnemyMove>,
    pub loot: Vec<EnemyLoot>,
    pub gold: u32,
}

#[derive(Default, TypePath)]
pub struct EnemyAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum EnemyAssetLoaderError {
    #[error("Could not load enemy: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse enemy RON: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

impl AssetLoader for EnemyAssetLoader {
    type Asset = EnemyData;
    type Settings = ();
    type Error = EnemyAssetLoaderError;

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
        &["enem.ron"]
    }
}
