use std::{fmt::Display, fs::File, io::Read};

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
    let SaveFile {
        tiles,
        tile_connections,
    } = match from_bytes::<SaveFile>(&bytes) {
        Ok(value) => value,
        Err(err) => {
            handle_error(&mut game_states, &game_state_path, err);
            return;
        }
    };

    // Every component gets its own vec for now.
    let tile_ids = tiles
        .iter()
        .map(|_| commands.spawn(StateScoped(GameStates::Gameplay)).id())
        .collect::<Vec<_>>();
    let mut tile_names = Vec::with_capacity(tiles.len());
    let mut tile_types = Vec::with_capacity(tiles.len());
    let mut textures = Vec::with_capacity(tiles.len());
    let mut axial_coordinates = Vec::with_capacity(tiles.len());
    let mut neighboring_tiles = Vec::with_capacity(tiles.len());

    let tile_connection_ids = tile_connections
        .iter()
        .map(|_| commands.spawn(StateScoped(GameStates::Gameplay)).id())
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
            match axial_coordinates.get(*tile_0) {
                Some(axial_coordinates) => axial_coordinates,
                None => {
                    handle_error(&mut game_states, &game_state_path, format!("The tile connection which connects the tiles at index {tile_0} and {tile_1} did contain the out-of-bounds tile index {tile_0}."));
                    return;
                }
            },
            match axial_coordinates.get(*tile_1) {
                Some(axial_coordinates) => axial_coordinates,
                None => {
                    handle_error(&mut game_states, &game_state_path, format!("The tile connection which connects the tiles at index {tile_0} and {tile_1} did contain the out-of-bounds tile index {tile_1}."));
                    return;
                }
            },
        ) {
            Some(neighboring) => match neighboring {
                Neighboring::Right => {
                    let tile = &mut neighboring_tiles[*tile_0].right;
                    if tile.is_some() {
                        handle_error(
                            &mut game_states,
                            &game_state_path,
                            format!(
                                "Duplicate connection detected: Multiple tile connections point to the 'right' side of the tile at index {tile_0}. One of these connections also links to the tile at index {tile_1}."
                            )
                        );
                        return;
                    }
                    *tile = Some(tile_connection_ids[i]);

                    let tile = &mut neighboring_tiles[*tile_1].left;
                    if tile.is_some() {
                        handle_error(
                            &mut game_states,
                            &game_state_path,
                            format!(
                                "Duplicate connection detected: Multiple tile connections point to the 'left' side of the tile at index {tile_1}. One of these connections also links to the tile at index {tile_0}."
                            )
                        );
                        return;
                    }
                    *tile = Some(tile_connection_ids[i]);
                }

                Neighboring::LowerRight => {
                    let tile = &mut neighboring_tiles[*tile_0].lower_right;
                    if tile.is_some() {
                        handle_error(
                            &mut game_states,
                            &game_state_path,
                            format!(
                                "Duplicate connection detected: Multiple tile connections point to the 'lower right' side of the tile at index {tile_0}. One of these connections also links to the tile at index {tile_1}."
                            )
                        );
                        return;
                    }
                    *tile = Some(tile_connection_ids[i]);

                    let tile = &mut neighboring_tiles[*tile_1].upper_left;
                    if tile.is_some() {
                        handle_error(
                            &mut game_states,
                            &game_state_path,
                            format!(
                                "Duplicate connection detected: Multiple tile connections point to the 'upper left' side of the tile at index {tile_1}. One of these connections also links to the tile at index {tile_0}."
                            )
                        );
                        return;
                    }
                    *tile = Some(tile_connection_ids[i]);
                }

                Neighboring::LowerLeft => {
                    let tile = &mut neighboring_tiles[*tile_0].lower_left;
                    if tile.is_some() {
                        handle_error(
                            &mut game_states,
                            &game_state_path,
                            format!(
                                "Duplicate connection detected: Multiple tile connections point to the 'lower left' side of the tile at index {tile_0}. One of these connections also links to the tile at index {tile_1}."
                            )
                        );
                        return;
                    }
                    *tile = Some(tile_connection_ids[i]);

                    let tile = &mut neighboring_tiles[*tile_1].upper_right;
                    if tile.is_some() {
                        handle_error(
                            &mut game_states,
                            &game_state_path,
                            format!(
                                "Duplicate connection detected: Multiple tile connections point to the 'upper right' side of the tile at index {tile_1}. One of these connections also links to the tile at index {tile_0}."
                            )
                        );
                        return;
                    }
                    *tile = Some(tile_connection_ids[i]);
                }

                Neighboring::Left => {
                    let tile = &mut neighboring_tiles[*tile_0].left;
                    if tile.is_some() {
                        handle_error(
                            &mut game_states,
                            &game_state_path,
                            format!(
                                "Duplicate connection detected: Multiple tile connections point to the 'left' side of the tile at index {tile_0}. One of these connections also links to the tile at index {tile_1}."
                            )
                        );
                        return;
                    }
                    *tile = Some(tile_connection_ids[i]);

                    let tile = &mut neighboring_tiles[*tile_1].right;
                    if tile.is_some() {
                        handle_error(
                            &mut game_states,
                            &game_state_path,
                            format!(
                                "Duplicate connection detected: Multiple tile connections point to the 'right' side of the tile at index {tile_1}. One of these connections also links to the tile at index {tile_0}."
                            )
                        );
                        return;
                    }
                    *tile = Some(tile_connection_ids[i]);
                }

                Neighboring::UpperLeft => {
                    let tile = &mut neighboring_tiles[*tile_0].upper_left;
                    if tile.is_some() {
                        handle_error(
                            &mut game_states,
                            &game_state_path,
                            format!(
                                "Duplicate connection detected: Multiple tile connections point to the 'upper left' side of the tile at index {tile_0}. One of these connections also links to the tile at index {tile_1}."
                            )
                        );
                        return;
                    }
                    *tile = Some(tile_connection_ids[i]);

                    let tile = &mut neighboring_tiles[*tile_1].lower_right;
                    if tile.is_some() {
                        handle_error(
                            &mut game_states,
                            &game_state_path,
                            format!(
                                "Duplicate connection detected: Multiple tile connections point to the 'lower right' side of the tile at index {tile_1}. One of these connections also links to the tile at index {tile_0}."
                            )
                        );
                        return;
                    }
                    *tile = Some(tile_connection_ids[i]);
                }

                Neighboring::UpperRight => {
                    let tile = &mut neighboring_tiles[*tile_0].upper_right;
                    if tile.is_some() {
                        handle_error(
                            &mut game_states,
                            &game_state_path,
                            format!(
                                "Duplicate connection detected: Multiple tile connections point to the 'upper right' side of the tile at index {tile_0}. One of these connections also links to the tile at index {tile_1}."
                            )
                        );
                        return;
                    }
                    *tile = Some(tile_connection_ids[i]);

                    let tile = &mut neighboring_tiles[*tile_1].lower_left;
                    if tile.is_some() {
                        handle_error(
                            &mut game_states,
                            &game_state_path,
                            format!(
                                "Duplicate connection detected: Multiple tile connections point to the 'lower left' side of the tile at index {tile_1}. One of these connections also links to the tile at index {tile_0}."
                            )
                        );
                        return;
                    }
                    *tile = Some(tile_connection_ids[i]);
                }
            },
            None => {
                handle_error(&mut game_states, &game_state_path, format!("The tile connection which connects the tiles at index {tile_0} and {tile_1} connects tiles that are not neighboring."));
                return;
            }
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
            handle_error(&mut game_states, &game_state_path, format!("The tile at the axial coordinates {axial_coordinates} had a neighboring tile to its right at coordinates {} but no corresponding `TileConnection`.", axial_coordinates.next_right()));
            return;
        }
        if neighboring_tiles.lower_right.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&axial_coordinates.next_lower_right())
        {
            handle_error(&mut game_states, &game_state_path, format!("The tile at the axial coordinates {axial_coordinates} had a neighboring tile to its lower right at coordinates {} but no corresponding `TileConnection`.", axial_coordinates.next_right()));
            return;
        }
        if neighboring_tiles.lower_left.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&axial_coordinates.next_lower_left())
        {
            handle_error(&mut game_states, &game_state_path, format!("The tile at the axial coordinates {axial_coordinates} had a neighboring tile to its lower left at coordinates {} but no corresponding `TileConnection`.", axial_coordinates.next_right()));
            return;
        }
        if neighboring_tiles.left.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&axial_coordinates.next_left())
        {
            handle_error(&mut game_states, &game_state_path, format!("The tile at the axial coordinates {axial_coordinates} had a neighboring tile to its left at coordinates {} but no corresponding `TileConnection`.", axial_coordinates.next_right()));
            return;
        }
        if neighboring_tiles.upper_left.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&axial_coordinates.next_upper_left())
        {
            handle_error(&mut game_states, &game_state_path, format!("The tile at the axial coordinates {axial_coordinates} had a neighboring tile to its upper left at coordinates {} but no corresponding `TileConnection`.", axial_coordinates.next_right()));
            return;
        }
        if neighboring_tiles.upper_right.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&axial_coordinates.next_upper_right())
        {
            handle_error(&mut game_states, &game_state_path, format!("The tile at the axial coordinates {axial_coordinates} had a neighboring tile to its upper right at coordinates {} but no corresponding `TileConnection`.", axial_coordinates.next_right()));
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

    commands.insert_or_spawn_batch(
        tile_ids.into_iter().zip(
            izip!(
                tile_names,
                tile_types,
                textures,
                axial_coordinates,
                neighboring_tiles
            )
            .map(
                |(tile_name, tile_type, texture, axial_coordinates, neighboring_tiles)| {
                    (
                        tile_type,
                        tile_name,
                        SpriteBundle {
                            texture,
                            transform: Transform::from_translation(axial_coordinates.into()),
                            ..Default::default()
                        },
                        axial_coordinates,
                        neighboring_tiles,
                    )
                },
            ),
        ),
    );

    commands.insert_or_spawn_batch(tile_connection_ids.into_iter().zip(
        izip!(connected_tiles).map(|connected_tiles| {
            (
                Name::new(format!(
                    "tile_connection_{}_{}",
                    connected_tiles.0.index(),
                    connected_tiles.1.index()
                )),
                connected_tiles,
            )
        }),
    ));

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
