use std::{fmt::Display, fs::File, io::Read, vec};

use crate::{
    gameplay_plugin::{
        components::{TileConnectionEntity, TileEntity, TileType},
        resources::{HexagonalMap, Tile},
        save_file::{SaveFile, SaveFileTile, SaveFileTileConnection, SaveFileTileData},
        GameplayStates,
    },
    resources::SaveFilePath,
    GameStates,
};

use super::resources::LoadFromFileSuccessful;
use bevy::{
    asset::{AssetServer, LoadState},
    core::Name,
    log::error,
    math::Vec2,
    prelude::{Commands, NextState, Res, ResMut, StateScoped, Transform},
    sprite::SpriteBundle,
};
use ron::de::from_bytes;

/// Utility function to log an error and set the game state to MainMenu.
fn handle_error(game_states: &mut ResMut<NextState<GameStates>>, path: &str, err: impl Display) {
    error!("Failed to load save file at {path}: {err}");
    game_states.set(GameStates::MainMenu);
}

pub fn load_from_file(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_states: ResMut<NextState<GameStates>>,
    save_file_path: Res<SaveFilePath>,
) {
    // Read the file and parse it.
    let path = save_file_path.as_ref().relative_to_assets();
    let game_state_path = format!("assets/{path}/game_state.ron");
    let mut bytes = vec![];
    let mut file = match File::open(&game_state_path) {
        Ok(file) => file,
        Err(err) => {
            handle_error(&mut game_states, &game_state_path, err);
            return;
        }
    };
    if let Err(err) = file.read_to_end(&mut bytes) {
        handle_error(&mut game_states, &game_state_path, err);
        return;
    };
    let SaveFile { tiles } = match from_bytes::<SaveFile>(&bytes) {
        Ok(value) => value,
        Err(err) => {
            handle_error(&mut game_states, &game_state_path, err);
            return;
        }
    };

    // This will be converted into the final hexagonal map resource later on.
    let mut tiles_and_connection_entities = Vec::with_capacity(tiles.capacity());
    // Converting this into a hex map is useful for two reasons:
    // 1. It will check if the length of the vec corresponds to a valid amount of tiles (1, 7, 19, 37, etc. tiles).
    // It will allow us to get the correct coordinates for each tile corresponding to its index.
    let temporary_hexagonal_map = match HexagonalMap::from_vec(tiles) {
        Ok(val) => val,
        Err(err) => {
            handle_error(
                &mut game_states,
                &game_state_path,
                format!("The length of `tiles` was incorrect: {err}"),
            );
            return;
        }
    };
    let mut assets_to_load = vec![];
    for (
        SaveFileTile {
            tile_data,
            tile_connection_right,
            tile_connection_lower_right,
            tile_connection_lower_left,
        },
        coordinates,
    ) in temporary_hexagonal_map.iter_with_coordinates()
    {
        // Instance the tile entity.
        let SaveFileTileData { tile_type } = tile_data;
        let tile_type_asset =
            asset_server.load(format!("{path}/tile_types/{tile_type}/tile_type.ron"));
        assets_to_load.push(tile_type_asset.clone().untyped());
        let texture_asset = asset_server.load(format!("{path}/tile_types/{tile_type}/texture.png"));
        assets_to_load.push(texture_asset.clone().untyped());
        let tile_entity = commands
            .spawn((
                SpriteBundle {
                    texture: texture_asset,
                    transform: Transform::from_translation(Vec2::from(coordinates).extend(0.0)),
                    ..Default::default()
                },
                TileType::new(tile_type_asset),
                coordinates,
                Name::new(tile_type),
                StateScoped(GameStates::Gameplay),
            ))
            .id();

        fn instance_tile_connection(
            tile_connection: SaveFileTileConnection,
            commands: &mut Commands<'_, '_>,
        ) -> bevy::prelude::Entity {
            let SaveFileTileConnection { temp_data } = tile_connection;
            commands
                .spawn((
                    Name::new(format!("tile_connection_{}", temp_data.0)),
                    temp_data,
                    StateScoped(GameStates::Gameplay),
                ))
                .id()
        }

        let tile_connection_right_entity =
            instance_tile_connection(tile_connection_right, &mut commands);
        let tile_connection_lower_right_entity =
            instance_tile_connection(tile_connection_lower_right, &mut commands);
        let tile_connection_lower_left_entity =
            instance_tile_connection(tile_connection_lower_left, &mut commands);

        tiles_and_connection_entities.push(Tile::new(
            tile_entity,
            tile_connection_right_entity,
            tile_connection_lower_right_entity,
            tile_connection_lower_left_entity,
        ));
    }

    commands.insert_resource(LoadFromFileSuccessful { assets_to_load });
    commands.insert_resource(HexagonalMap::from_vec(tiles_and_connection_entities).expect("BUG: Could not convert from `tiles_and_connection_entities` to a `HexagonalMap` despite the fact it was to convert from `tiles` to such a map."));
}

pub fn check_if_loaded(
    load_from_file_successful: Res<LoadFromFileSuccessful>,
    asset_server: Res<AssetServer>,
    mut game_states: ResMut<NextState<GameStates>>,
    mut gameplay_states: ResMut<NextState<GameplayStates>>,
) {
    let mut loaded = true;
    for handle in &load_from_file_successful.assets_to_load {
        match asset_server.load_state(handle.id()) {
            LoadState::Failed(err) => {
                error!("Failed to load save file: {err}");
                game_states.as_mut().set(GameStates::MainMenu);
                return;
            }
            LoadState::Loaded => continue,
            _ => loaded = false,
        };
    }
    if loaded {
        gameplay_states.as_mut().set(GameplayStates::InGame);
    }
}
