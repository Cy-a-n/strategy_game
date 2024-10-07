use bevy::{asset::Handle, ecs::entity::Entity, math::Vec2, prelude::Component, reflect::Reflect};
use serde::{Deserialize, Serialize};

use super::assets;

pub type TileConnectionEntity = Entity;
pub type TileEntity = Entity;

#[derive(Reflect, Component, Default, Debug, Clone, PartialEq, Eq)]
pub struct TileType {
    tile_type_data: Handle<assets::TileType>,
}

impl TileType {
    pub fn new(tile_type_data: Handle<assets::TileType>) -> Self {
        Self { tile_type_data }
    }

    pub fn tile_type_data(&self) -> &Handle<assets::TileType> {
        &self.tile_type_data
    }
}

// TODO!
#[derive(Reflect, Component, Debug, Clone, PartialEq, Eq)]
pub struct ConnectedTiles(pub TileEntity, pub TileEntity);

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    lost_health: u8,
}

#[derive(Copy, Component, Reflect, Debug, Clone, PartialEq, Eq)]
pub struct AxialCoordinates {
    q: i32,
    r: i32,
}

#[derive(Reflect, Component, Debug, Clone, PartialEq, Eq)]
pub struct CubicCoordinates {
    q: i32,
    r: i32,
    s: i32,
}

impl CubicCoordinates {
    pub fn new(q: i32, r: i32, s: i32) -> Self {
        Self { q, r, s }
    }

    pub fn q(&self) -> i32 {
        self.q
    }

    pub fn r(&self) -> i32 {
        self.r
    }

    pub fn s(&self) -> i32 {
        self.s
    }
}

impl AxialCoordinates {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    pub fn distance_to_origin(&self) -> u32 {
        let CubicCoordinates { q, r, s } = (*self).into();
        (q.unsigned_abs() + r.unsigned_abs() + s.unsigned_abs()) / 2
    }

    pub fn q(&self) -> i32 {
        self.q
    }

    pub fn r(&self) -> i32 {
        self.r
    }
}

impl From<AxialCoordinates> for CubicCoordinates {
    fn from(value: AxialCoordinates) -> Self {
        let AxialCoordinates { q, r } = value;
        CubicCoordinates { q, r, s: -q - r }
    }
}

// TODO!
const Q_VECTOR: Vec2 = Vec2 { x: 32.0, y: 0.0 };
const R_VECTOR: Vec2 = Vec2 { x: 16.0, y: -21.0 };

// TODO!
impl From<AxialCoordinates> for Vec2 {
    fn from(value: AxialCoordinates) -> Self {
        Vec2 {
            x: value.q as f32 * Q_VECTOR.x + value.r as f32 * R_VECTOR.x,
            y: value.q as f32 * Q_VECTOR.y + value.r as f32 * R_VECTOR.y,
        }
    }
}

// TODO!
#[derive(Serialize, Deserialize, Component, Reflect, Debug, Clone, PartialEq, Eq)]
pub struct TempConnectionComponent(pub usize);
