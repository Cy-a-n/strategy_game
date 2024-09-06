use assets::TileTypeLoader;
use bevy::{
    app::Plugin,
    asset::{AssetApp, Handle, ReflectAsset, ReflectHandle},
    prelude::{AppExtStates, OnEnter, ReflectComponent, ReflectResource, StateSet, SubStates},
    reflect::Reflect,
};
use components::{AxialCoordinates, ConnectedTiles, NeighboringTiles, TileType};
use in_game_plugin::InGamePlugin;
use loading_screen_plugin::LoadingScreenPlugin;
use resources::TilesByCoordinates;
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
        app.add_plugins((LoadingScreenPlugin, InGamePlugin));

        app.register_type::<NeighboringTiles>()
            .register_type::<ConnectedTiles>()
            .register_type::<TileType>()
            .register_type::<AxialCoordinates>()
            .register_type::<TilesByCoordinates>()
            .register_type::<assets::TileType>()
            .register_type::<Handle<assets::TileType>>()
            .register_type_data::<NeighboringTiles, ReflectComponent>()
            .register_type_data::<ConnectedTiles, ReflectComponent>()
            .register_type_data::<TileType, ReflectComponent>()
            .register_type_data::<AxialCoordinates, ReflectComponent>()
            .register_type_data::<TilesByCoordinates, ReflectResource>()
            .register_type_data::<assets::TileType, ReflectAsset>()
            .register_type_data::<Handle<assets::TileType>, ReflectHandle>();

        app.init_asset::<assets::TileType>()
            .init_asset_loader::<TileTypeLoader>();

        app.add_systems(OnEnter(GameStates::Gameplay), setup)
            .cleanup_resource::<TilesByCoordinates>(GameStates::Gameplay);

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
