use crate::prelude::*;
use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use serde::Deserialize;
use thiserror::Error;

pub type AreaId = String;
pub type CharacterId = String;

#[derive(TypePath, Deserialize)]
pub struct AreaNarration {
    pub lines: Vec<String>,
}

#[derive(TypePath, Deserialize)]
pub struct AreaDialogue {
    pub character_id: Option<CharacterId>,
    pub lines: Vec<String>,
}

#[derive(Asset, TypePath, Deserialize)]
pub struct AreaData {
    pub id: AreaId,
    pub name: String,
    pub narration: AreaNarration,
    pub dialogue: AreaDialogue,
    pub exits: Vec<AreaExit>,
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
