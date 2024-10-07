use assets::{TileTypeLoader, UnitTypeLoader};
use bevy::{
    app::Plugin,
    asset::{AssetApp, Handle, ReflectAsset, ReflectHandle},
    prelude::{AppExtStates, OnEnter, ReflectResource, StateSet, SubStates},
    reflect::Reflect,
};
use components::{AxialCoordinates, ConnectedTiles, TileType};
use in_game_plugin::InGamePlugin;
use loading_screen_plugin::LoadingScreenPlugin;
use resources::{HexagonalMap, Tile};
use systems::setup;

use crate::{cleanup::Cleanup, GameStates};

mod in_game_plugin;
mod loading_screen_plugin;

mod assets;
mod components;
mod resources;
mod save_file;
mod systems;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // Plugins
        app.add_plugins((LoadingScreenPlugin, InGamePlugin));

        // Components
        app.register_type::<ConnectedTiles>()
            .register_type::<TileType>()
            .register_type::<AxialCoordinates>();

        // Resources
        app.register_type::<HexagonalMap<Tile>>()
            .register_type_data::<HexagonalMap<Tile>, ReflectResource>()
            .cleanup_resource::<HexagonalMap<Tile>>(GameStates::Gameplay);

        // Assets, asset loaders and their handles.
        app.init_asset::<assets::TileType>()
            .init_asset_loader::<TileTypeLoader>()
            .register_type::<assets::TileType>()
            .register_type_data::<assets::TileType, ReflectAsset>()
            .register_type::<Handle<assets::TileType>>()
            .register_type_data::<Handle<assets::TileType>, ReflectHandle>();
        app.init_asset::<assets::UnitType>()
            .init_asset_loader::<UnitTypeLoader>()
            .register_type::<assets::UnitType>()
            .register_type_data::<assets::UnitType, ReflectAsset>()
            .register_type::<Handle<assets::UnitType>>()
            .register_type_data::<Handle<assets::UnitType>, ReflectHandle>();

        // Other systems.
        app.add_systems(OnEnter(GameStates::Gameplay), setup);

        // States.
        app.add_sub_state::<GameplayStates>();
        app.enable_state_scoped_entities::<GameplayStates>();
    }
}

#[derive(SubStates, Reflect, Hash, Default, Debug, Clone, PartialEq, Eq)]
#[source(GameStates = GameStates::Gameplay {..})]
enum GameplayStates {
    #[default]
    LoadingScreen,
    InGame,
}
