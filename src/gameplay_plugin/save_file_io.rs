use std::{fs::File, hash::Hash, io::Read, path::Path};

use bevy::{
    asset::AssetServer,
    core::Name,
    ecs::world::World,
    prelude::{Commands, Image, Res, Transform},
    sprite::SpriteBundle,
    utils::hashbrown::HashMap,
};
use itertools::izip;
use ron::de::from_bytes;
use serde::{Deserialize, Serialize};

use super::tiles::{
    self, CubicCoordinates, NeighboringTiles, TileType, TileTypeAsset, TilesByCoordinates,
};

type TileIdx = usize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct Tile {
    tile_type: String,
    // TODO! Mention this is ref https://www.redblobgames.com/grids/hexagons/
    cubic_coordinates: (i32, i32, i32),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct ConnectedTiles(TileIdx, TileIdx);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct TileConnection {
    connected_tiles: ConnectedTiles,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SaveFile {
    tiles: Vec<Tile>,
    tile_connections: Vec<TileConnection>,
}

impl SaveFile {
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
            cubic_coordinates: coordinates,
        } in &tiles
        {
            tile_names.push(Name::new(tile_type.to_string()));
            tile_types.push(asset_server.load::<TileTypeAsset>(format!(
                "{path}/tile_types/{tile_type}/tile_type_data.ron"
            )));
            textures.push(
                asset_server.load::<Image>(format!("{path}/tile_types/{tile_type}/texture.png")),
            );

            assert_eq!(coordinates.0 + coordinates.1 + coordinates.2, 0);
            cubic_coordinates.push(tiles::CubicCoordinates(
                coordinates.0,
                coordinates.1,
                coordinates.2,
            ));

            transforms.push(Transform::from_xyz(
                coordinates.0 as f32 * 32.0 + coordinates.1 as f32 * 16.0,
                coordinates.0 as f32 * 0.0 + coordinates.1 as f32 * -21.0,
                0.0,
            ));

            // To be filled later.
            neighboring_tiles.push(NeighboringTiles::default());
        }

        for (
            i,
            TileConnection {
                connected_tiles: ConnectedTiles(tile_0, tile_1),
            },
        ) in tile_connections.iter().enumerate()
        {
            let CubicCoordinates(q_0, r_0, s_0) = *cubic_coordinates.get(*tile_0).unwrap();
            let CubicCoordinates(q_1, r_1, s_1) = *cubic_coordinates.get(*tile_1).unwrap();
            match (q_1 - q_0, r_1 - r_0, s_1 - s_0) {
                (1, 0, -1) => {
                    neighboring_tiles[*tile_0].right = Some(tile_connection_ids[i]);
                    neighboring_tiles[*tile_1].left = Some(tile_connection_ids[i]);
                }
                (0, 1, -1) => {
                    neighboring_tiles[*tile_0].lower_right = Some(tile_connection_ids[i]);
                    neighboring_tiles[*tile_1].upper_left = Some(tile_connection_ids[i]);
                }
                (-1, 1, 0) => {
                    neighboring_tiles[*tile_0].lower_left = Some(tile_connection_ids[i]);
                    neighboring_tiles[*tile_1].upper_right = Some(tile_connection_ids[i]);
                }
                (-1, 0, 1) => {
                    neighboring_tiles[*tile_0].left = Some(tile_connection_ids[i]);
                    neighboring_tiles[*tile_1].right = Some(tile_connection_ids[i]);
                }
                (0, -1, 1) => {
                    neighboring_tiles[*tile_0].upper_left = Some(tile_connection_ids[i]);
                    neighboring_tiles[*tile_1].lower_right = Some(tile_connection_ids[i]);
                }
                (1, -1, 0) => {
                    neighboring_tiles[*tile_0].upper_right = Some(tile_connection_ids[i]);
                    neighboring_tiles[*tile_1].lower_left = Some(tile_connection_ids[i]);
                }
                _ => panic!(""),
            }
            connected_tiles.push(tiles::ConnectedTiles(tile_ids[*tile_0], tile_ids[*tile_1]));
        }
        println!("{neighboring_tiles:#?}");

        let mut tiles_by_coordinates = TilesByCoordinates(HashMap::new());
        for (tile_id, coordinates) in tile_ids.iter().zip(cubic_coordinates.iter()) {
            tiles_by_coordinates.0.insert(coordinates.clone(), *tile_id);
        }

        // Check if save file is corrupted due to missing connections.
        for (CubicCoordinates(q, r, s), neighboring_tiles) in
            cubic_coordinates.iter().zip(neighboring_tiles.iter())
        {
            if neighboring_tiles.right.is_none()
                && tiles_by_coordinates
                    .0
                    .contains_key(&CubicCoordinates(q + 1, r + 0, s - 1))
            {
                panic!("");
            }
            if neighboring_tiles.lower_right.is_none()
                && tiles_by_coordinates
                    .0
                    .contains_key(&CubicCoordinates(q + 0, r + 1, s - 1))
            {
                panic!("");
            }
            if neighboring_tiles.lower_left.is_none()
                && tiles_by_coordinates
                    .0
                    .contains_key(&CubicCoordinates(q - 1, r + 1, s + 0))
            {
                panic!("");
            }
            if neighboring_tiles.left.is_none()
                && tiles_by_coordinates
                    .0
                    .contains_key(&CubicCoordinates(q - 1, r + 0, s + 1))
            {
                panic!("");
            }
            if neighboring_tiles.upper_left.is_none()
                && tiles_by_coordinates
                    .0
                    .contains_key(&CubicCoordinates(q + 0, r - 1, s + 1))
            {
                panic!("");
            }
            if neighboring_tiles.upper_right.is_none()
                && tiles_by_coordinates
                    .0
                    .contains_key(&CubicCoordinates(q + 1, r - 1, s + 0))
            {
                panic!("");
            }
        }

        for (
            tile_id,
            tile_name,
            tile_type,
            texture,
            cubic_coordinates,
            transform,
            neighboring_tiles,
        ) in izip!(
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
}
