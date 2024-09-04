use bevy::{
    asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, Handle, LoadContext, ReflectAsset},
    ecs::entity::Entity,
    prelude::{Component, ReflectResource, Resource},
    reflect::{impl_reflect, Reflect, TypePath},
    utils::HashMap,
};
use ron::de::from_bytes;
use serde::Deserialize;
use thiserror::Error;

pub type TileConnectionEntity = Entity;
pub type TileEntity = Entity;

#[derive(Deserialize, Reflect, Asset, Debug, Clone, PartialEq, Eq)]
#[reflect(Asset)]
pub struct TileTypeAsset {
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
    type Asset = TileTypeAsset;
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
        let tile_type_data = from_bytes::<TileTypeAsset>(&bytes)?;
        Ok(tile_type_data)
    }

    fn extensions(&self) -> &[&str] {
        &["tile"]
    }
}

#[derive(Reflect, Component, Default, Debug, Clone, PartialEq, Eq)]
pub struct TileType {
    tile_type_data: Handle<TileTypeAsset>,
}

#[derive(Reflect, Component, Default, Debug, Clone, PartialEq, Eq)]
pub struct NeighboringTiles {
    pub right: Option<TileConnectionEntity>,
    pub upper_right: Option<TileConnectionEntity>,
    pub lower_right: Option<TileConnectionEntity>,
    pub left: Option<TileConnectionEntity>,
    pub lower_left: Option<TileConnectionEntity>,
    pub upper_left: Option<TileConnectionEntity>,
}

// TODO! Mention this is ref https://www.redblobgames.com/grids/hexagons/
#[derive(Hash, Component, Reflect, Debug, Clone, PartialEq, Eq)]
pub struct CubicCoordinates(pub i32, pub i32, pub i32);

#[derive(Reflect, Component, Debug, Clone, PartialEq, Eq)]
pub struct ConnectedTiles(pub TileEntity, pub TileEntity);

#[derive(Resource, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Resource)]
pub struct TilesByCoordinates(pub HashMap<CubicCoordinates, TileEntity>);
