use std::{fs::File, io::Read};

use crate::gameplay_plugin::components::{AxialCoordinates, Neighboring};

use super::super::super::{
    resources::TilesByCoordinates,
    save_file::{ConnectedTiles, SaveFile, Tile, TileConnection},
};
use bevy::{
    asset::AssetServer,
    core::Name,
    prelude::{Commands, Image, Res, Transform},
    sprite::SpriteBundle,
    utils::HashMap,
};
use itertools::izip;
use ron::de::from_bytes;

use super::super::super::assets::TileTypeAsset;

pub fn load_from_file(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO!$
    let path = "save_files/scenarios/test_map";

    let mut bytes = vec![];
    File::open(format!("assets/{path}/game_state.ron"))
        .unwrap()
        .read_to_end(&mut bytes)
        .unwrap();
    let SaveFile {
        tiles,
        tile_connections,
    } = match from_bytes::<SaveFile>(&bytes) {
        Ok(value) => value,
        Err(err) => panic!("{err}"),
    };

    // Every component gets its own vec for now.
    let tile_ids = tiles
        .iter()
        .map(|_| commands.spawn(()).id())
        .collect::<Vec<_>>();
    let mut tile_names = Vec::with_capacity(tiles.len());
    let mut tile_types = Vec::with_capacity(tiles.len());
    let mut textures = Vec::with_capacity(tiles.len());
    let mut cubic_coordinates = Vec::with_capacity(tiles.len());
    let mut transforms = Vec::with_capacity(tiles.len());
    let mut neighboring_tiles = Vec::with_capacity(tiles.len());

    let tile_connection_ids = tile_connections
        .iter()
        .map(|_| commands.spawn(()).id())
        .collect::<Vec<_>>();
    let mut connected_tiles = Vec::with_capacity(tile_connections.len());

    for Tile {
        tile_type,
        cubic_coordinates: current_cubic_coordinates,
    } in tiles
    {
        tile_names.push(Name::new(tile_type.to_string()));
        tile_types
            .push(asset_server.load::<TileTypeAsset>(format!(
                "{path}/tile_types/{tile_type}/tile_type_data.ron"
            )));
        textures
            .push(asset_server.load::<Image>(format!("{path}/tile_types/{tile_type}/texture.png")));

        transforms.push(Transform::from_translation(
            current_cubic_coordinates.into(),
        ));
        cubic_coordinates.push(current_cubic_coordinates);

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
            cubic_coordinates.get(*tile_0).unwrap(),
            cubic_coordinates.get(*tile_1).unwrap(),
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
    for (tile_id, coordinates) in tile_ids.iter().zip(cubic_coordinates.iter()) {
        tiles_by_coordinates.0.insert(*coordinates, *tile_id);
    }

    // Check if save file is corrupted due to missing connections.
    for (cubic_coordinates, neighboring_tiles) in
        cubic_coordinates.iter().zip(neighboring_tiles.iter())
    {
        if neighboring_tiles.right.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&cubic_coordinates.next_right())
        {
            panic!("");
        }
        if neighboring_tiles.lower_right.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&cubic_coordinates.next_lower_right())
        {
            panic!("");
        }
        if neighboring_tiles.lower_left.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&cubic_coordinates.next_lower_left())
        {
            panic!("");
        }
        if neighboring_tiles.left.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&cubic_coordinates.next_left())
        {
            panic!("");
        }
        if neighboring_tiles.upper_left.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&cubic_coordinates.next_upper_left())
        {
            panic!("");
        }
        if neighboring_tiles.upper_right.is_none()
            && tiles_by_coordinates
                .0
                .contains_key(&cubic_coordinates.next_upper_right())
        {
            panic!("");
        }
    }

    for (tile_id, tile_name, tile_type, texture, cubic_coordinates, transform, neighboring_tiles) in izip!(
        tile_ids,
        tile_names,
        tile_types,
        textures,
        cubic_coordinates,
        transforms,
        neighboring_tiles
    ) {
        commands.entity(tile_id).insert((
            tile_type,
            tile_name,
            SpriteBundle {
                texture,
                transform,
                ..Default::default()
            },
            cubic_coordinates,
            neighboring_tiles,
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
        ));
    }

    commands.insert_resource(tiles_by_coordinates);
}
