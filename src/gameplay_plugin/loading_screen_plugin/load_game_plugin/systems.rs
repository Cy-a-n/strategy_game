use std::{fs::File, io::Read};

use crate::{
    gameplay_plugin::{
        components::{AxialCoordinates, Neighboring},
        GameplayStates,
    },
    resources::SaveFilePath,
    GameStates,
};

use super::{
    super::super::{
        resources::TilesByCoordinates,
        save_file::{ConnectedTiles, SaveFile, Tile, TileConnection},
    },
    resources::LoadFromFileSuccessful,
};
use bevy::{
    asset::{AssetServer, LoadState},
    core::Name,
    log::error,
    prelude::{Commands, Image, NextState, Res, ResMut, StateScoped, Transform},
    sprite::SpriteBundle,
    utils::HashMap,
};
use itertools::{chain, izip};
use ron::de::from_bytes;

use super::super::super::assets::TileTypeAsset;

pub fn load_from_file(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_states: ResMut<NextState<GameStates>>,
    save_file_path: Res<SaveFilePath>,
) {
    let path = save_file_path.as_ref().relative_to_assets();
    let game_state_path = format!("assets/{path}/game_state.ron");
    let mut bytes = vec![];
    let mut file = match File::open(game_state_path.clone()) {
        Ok(file) => file,
        Err(_) => {
            error!("Failed to load save file because \"{game_state_path}\" does not exist.");
            game_states.as_mut().set(GameStates::MainMenu);
            return;
        }
    };
    if let Err(err) = file.read_to_end(&mut bytes) {
        error!("Failed to load save file at \"{game_state_path}\": {err}");
        game_states.as_mut().set(GameStates::MainMenu);
        return;
    };
    let SaveFile {
        tiles,
        tile_connections,
    } = match from_bytes::<SaveFile>(&bytes) {
        Ok(value) => value,
        Err(err) => {
            error!("Failed to load save file at \"{game_state_path}\": {err}");
            game_states.as_mut().set(GameStates::MainMenu);
            return;
        }
    };

    // Every component gets its own vec for now.
    let tile_ids = tiles
        .iter()
        .map(|_| commands.spawn(()).id())
        .collect::<Vec<_>>();
    let mut tile_names = Vec::with_capacity(tiles.len());
    let mut tile_types = Vec::with_capacity(tiles.len());
    let mut textures = Vec::with_capacity(tiles.len());
    let mut axial_coordinates = Vec::with_capacity(tiles.len());
    let mut neighboring_tiles = Vec::with_capacity(tiles.len());

    let tile_connection_ids = tile_connections
        .iter()
        .map(|_| commands.spawn(()).id())
        .collect::<Vec<_>>();
    let mut connected_tiles = Vec::with_capacity(tile_connections.len());

    for Tile {
        tile_type,
        axial_coordinates: current_axial_coordinates,
    } in tiles
    {
        tile_names.push(Name::new(tile_type.to_string()));
        tile_types
            .push(asset_server.load::<TileTypeAsset>(format!(
                "{path}/tile_types/{tile_type}/tile_type_data.ron"
            )));
        textures
            .push(asset_server.load::<Image>(format!("{path}/tile_types/{tile_type}/texture.png")));

        axial_coordinates.push(current_axial_coordinates);

        // To be filled later.
        neighboring_tiles.push(super::super::super::components::NeighboringTiles::default());
    }

    for (
        i,
        TileConnection {
            connected_tiles: ConnectedTiles(tile_0, tile_1),
        },
    ) in tile_connections.iter().enumerate()
    {
        match AxialCoordinates::neighboring(
            axial_coordinates.get(*tile_0).unwrap(),
            axial_coordinates.get(*tile_1).unwrap(),
        ) {
            Some(neighboring) => match neighboring {
                Neighboring::Right => {
                    neighboring_tiles[*tile_0].right = Some(tile_connection_ids[i]);
                    neighboring_tiles[*tile_1].left = Some(tile_connection_ids[i]);
                }
                Neighboring::LowerRight => {
                    neighboring_tiles[*tile_0].lower_right = Some(tile_connection_ids[i]);
                    neighboring_tiles[*tile_1].upper_left = Some(tile_connection_ids[i]);
                }
                Neighboring::LowerLeft => {
                    neighboring_tiles[*tile_0].lower_left = Some(tile_connection_ids[i]);
                    neighboring_tiles[*tile_1].upper_right = Some(tile_connection_ids[i]);
                }
                Neighboring::Left => {
                    neighboring_tiles[*tile_0].left = Some(tile_connection_ids[i]);
                    neighboring_tiles[*tile_1].right = Some(tile_connection_ids[i]);
                }
                Neighboring::UpperLeft => {
                    neighboring_tiles[*tile_0].upper_left = Some(tile_connection_ids[i]);
                    neighboring_tiles[*tile_1].lower_right = Some(tile_connection_ids[i]);
                }
                Neighboring::UpperRight => {
                    neighboring_tiles[*tile_0].upper_right = Some(tile_connection_ids[i]);
                    neighboring_tiles[*tile_1].lower_left = Some(tile_connection_ids[i]);
                }
            },
            None => todo!(),
        }
        connected_tiles.push(super::super::super::components::ConnectedTiles(
            tile_ids[*tile_0],
            tile_ids[*tile_1],
        ));
    }

    let mut tiles_by_coordinates = TilesByCoordinates(HashMap::new());
    for (tile_id, coordinates) in tile_ids.iter().zip(axial_coordinates.iter()) {
        tiles_by_coordinates.0.insert(*coordinates, *tile_id);
    }

    // Check if save file is corrupted due to missing connections.
    for (axial_coordinates, neighboring_tiles) in
        axial_coordinates.iter().zip(neighboring_tiles.iter())
    {
        if neighboring_tiles.right.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&axial_coordinates.next_right())
        {
            error!(
                "Failed to load save file because the tile at the axial coordinates {axial_coordinates} had a neighboring tile to its right at coordinates {} but no corresponding `TileConnection` at \"{game_state_path}\".",
                axial_coordinates.next_right()
            );
            game_states.as_mut().set(GameStates::MainMenu);
            return;
        }
        if neighboring_tiles.lower_right.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&axial_coordinates.next_lower_right())
        {
            error!(
                "Failed to load save file because the tile at the axial coordinates {axial_coordinates} had a neighboring tile to its lower right at coordinates {} but no corresponding `TileConnection` at \"{game_state_path}\".",
                axial_coordinates.next_lower_right()
            );
            game_states.as_mut().set(GameStates::MainMenu);
            return;
        }
        if neighboring_tiles.lower_left.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&axial_coordinates.next_lower_left())
        {
            error!(
                "Failed to load save file because the tile at the axial coordinates {axial_coordinates} had a neighboring tile to its lower left at coordinates {} but no corresponding `TileConnection` at \"{game_state_path}\".",
                axial_coordinates.next_lower_left()
            );
            game_states.as_mut().set(GameStates::MainMenu);
            return;
        }
        if neighboring_tiles.left.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&axial_coordinates.next_left())
        {
            error!(
                "Failed to load save file because the tile at the axial coordinates {axial_coordinates} had a neighboring tile to its left at coordinates {} but no corresponding `TileConnection` at \"{game_state_path}\".",
                axial_coordinates.next_left()
            );
            game_states.as_mut().set(GameStates::MainMenu);
            return;
        }
        if neighboring_tiles.upper_left.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&axial_coordinates.next_upper_left())
        {
            error!(
                "Failed to load save file because the tile at the axial coordinates {axial_coordinates} had a neighboring tile to its upper left at coordinates {} but no corresponding `TileConnection` at \"{game_state_path}\".",
                axial_coordinates.next_upper_left()
            );
            game_states.as_mut().set(GameStates::MainMenu);
            return;
        }
        if neighboring_tiles.upper_right.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&axial_coordinates.next_upper_right())
        {
            error!(
                "Failed to load save file because the tile at the axial coordinates {axial_coordinates} had a neighboring tile to its upper right at coordinates {} but no corresponding `TileConnection` at \"{game_state_path}\".",
                axial_coordinates.next_upper_right()
            );
            game_states.as_mut().set(GameStates::MainMenu);
            return;
        }
    }

    commands.insert_resource(LoadFromFileSuccessful {
        assets_to_load: chain!(
            tile_types.iter().map(|handle| handle.clone().untyped()),
            textures.iter().map(|handle| handle.clone().untyped())
        )
        .collect(),
    });

    for (tile_id, tile_name, tile_type, texture, axial_coordinates, neighboring_tiles) in izip!(
        tile_ids,
        tile_names,
        tile_types,
        textures,
        axial_coordinates,
        neighboring_tiles
    ) {
        commands.entity(tile_id).insert((
            tile_type,
            tile_name,
            SpriteBundle {
                texture,
                transform: Transform::from_translation(axial_coordinates.into()),
                ..Default::default()
            },
            axial_coordinates,
            neighboring_tiles,
            StateScoped(GameStates::Gameplay),
        ));
    }

    for (tile_connection_id, connected_tiles) in izip!(tile_connection_ids, connected_tiles) {
        commands.entity(tile_connection_id).insert((
            Name::new(format!(
                "tile_connection_{}_{}",
                connected_tiles.0.index(),
                connected_tiles.1.index()
            )),
            connected_tiles,
            StateScoped(GameStates::Gameplay),
        ));
    }

    commands.insert_resource(tiles_by_coordinates);
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
