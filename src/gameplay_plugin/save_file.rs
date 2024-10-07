use serde::{Deserialize, Serialize};

use super::components::TempConnectionComponent;

pub type TileTypePath = String;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SaveFileTileData {
    pub tile_type: TileTypePath,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SaveFileTileConnection {
    pub temp_data: TempConnectionComponent,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SaveFileTile {
    pub tile_data: SaveFileTileData,
    pub tile_connection_right: SaveFileTileConnection,
    pub tile_connection_lower_right: SaveFileTileConnection,
    pub tile_connection_lower_left: SaveFileTileConnection,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SaveFile {
    pub tiles: Vec<SaveFileTile>,
}
