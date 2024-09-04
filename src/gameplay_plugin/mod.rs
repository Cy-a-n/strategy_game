use bevy::{
    app::{Plugin, Startup},
    asset::{AssetApp, Handle, ReflectHandle},
};
use save_file_io::SaveFile;
use tiles::{
    ConnectedTiles, CubicCoordinates, NeighboringTiles, TileType, TileTypeAsset, TileTypeLoader,
    TilesByCoordinates,
};

use self::camera_plugin::CameraPlugin;

mod camera_plugin;
mod save_file_io;
mod tiles;

pub(super) struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(CameraPlugin)
            .init_asset::<TileTypeAsset>()
            .init_asset_loader::<TileTypeLoader>()
            .register_type::<NeighboringTiles>()
            .register_type::<ConnectedTiles>()
            .register_type::<TileType>()
            .register_type::<CubicCoordinates>()
            .register_type::<TilesByCoordinates>()
            .register_type::<TileTypeAsset>()
            .register_type_data::<Handle<TileTypeAsset>, ReflectHandle>()
            .add_systems(Startup, SaveFile::load_from_file);
    }
}
