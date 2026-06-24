use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Deserialize)]
pub struct CharacterData {
    pub id: String,
    pub display_name: String,
    pub speech_sfx: Option<String>,
    pub font: Option<String>,
    pub font_colour: Option<Color>,
}

#[derive(Default, TypePath)]
pub struct CharacterAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CharacterAssetLoaderError {
    #[error("Could not load character: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse character RON: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

impl AssetLoader for CharacterAssetLoader {
    type Asset = CharacterData;
    type Settings = ();
    type Error = CharacterAssetLoaderError;

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
        &["char.ron"]
    }
}
