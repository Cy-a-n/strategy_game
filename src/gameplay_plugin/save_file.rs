use serde::{Deserialize, Serialize};

use super::components::AxialCoordinates;

type TileIdx = usize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    pub tile_type: String,
    // TODO! Mention this is ref https://www.redblobgames.com/grids/hexagons/
    pub cubic_coordinates: AxialCoordinates,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ConnectedTiles(pub TileIdx, pub TileIdx);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TileConnection {
    pub connected_tiles: ConnectedTiles,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SaveFile {
    pub tiles: Vec<Tile>,
    pub tile_connections: Vec<TileConnection>,
}
