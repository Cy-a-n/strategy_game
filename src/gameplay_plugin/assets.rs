use bevy::{
    asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, LoadContext},
    reflect::Reflect,
};
use ron::de::from_bytes;
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Reflect, Asset, Debug, Clone, PartialEq, Eq)]
pub struct TileType {
    combat_width: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct TileTypeLoader;

// TODO! Better error handling.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum TileTypeLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not data of a tile type: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for TileTypeLoader {
    type Asset = TileType;
    type Settings = ();
    type Error = TileTypeLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let tile_type_data = from_bytes::<TileType>(&bytes)?;
        Ok(tile_type_data)
    }

    fn extensions(&self) -> &[&str] {
        &["tile"]
    }
}
