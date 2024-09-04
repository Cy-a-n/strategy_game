use bevy::{
    asset::Handle,
    ecs::entity::Entity,
    math::{Vec2, Vec3},
    prelude::Component,
    reflect::Reflect,
};
use derive_more::derive::{Add, AddAssign, Neg, Not, Sub, SubAssign};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::assets::TileTypeAsset;

pub type TileConnectionEntity = Entity;
pub type TileEntity = Entity;

#[derive(Reflect, Component, Default, Debug, Clone, PartialEq, Eq)]
pub struct TileType {
    tile_type_data: Handle<TileTypeAsset>,
}

#[derive(Reflect, Component, Default, Debug, Clone, PartialEq, Eq)]
pub struct NeighboringTiles {
    pub right: Option<TileConnectionEntity>,
    pub upper_right: Option<TileConnectionEntity>,
    pub lower_right: Option<TileConnectionEntity>,
    pub left: Option<TileConnectionEntity>,
    pub lower_left: Option<TileConnectionEntity>,
    pub upper_left: Option<TileConnectionEntity>,
}

#[derive(Reflect, Component, Debug, Clone, PartialEq, Eq)]
pub struct ConnectedTiles(pub TileEntity, pub TileEntity);

// TODO! Mention this is ref https://www.redblobgames.com/grids/hexagons/
#[derive(
    Not,
    Neg,
    Add,
    Sub,
    AddAssign,
    SubAssign,
    Serialize,
    Deserialize,
    Hash,
    Copy,
    Component,
    Reflect,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct AxialCoordinates {
    q: i32,
    r: i32,
}
impl AxialCoordinates {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    pub fn q(&self) -> i32 {
        self.q
    }

    pub fn r(&self) -> i32 {
        self.r
    }

    pub const RIGHT: Self = Self { q: 1, r: 0 };
    pub const LOWER_RIGHT: Self = Self { q: 0, r: 1 };
    pub const LOWER_LEFT: Self = Self { q: -1, r: 1 };
    pub const LEFT: Self = Self { q: -1, r: 0 };
    pub const UPPER_LEFT: Self = Self { q: 0, r: -1 };
    pub const UPPER_RIGHT: Self = Self { q: 1, r: -1 };

    pub fn next_right(&self) -> Self {
        *self + Self::RIGHT
    }

    pub fn next_lower_right(&self) -> Self {
        *self + Self::LOWER_RIGHT
    }

    pub fn next_lower_left(&self) -> Self {
        *self + Self::LOWER_LEFT
    }

    pub fn next_left(&self) -> Self {
        *self + Self::LEFT
    }

    pub fn next_upper_left(&self) -> Self {
        *self + Self::UPPER_LEFT
    }

    pub fn next_upper_right(&self) -> Self {
        *self + Self::UPPER_RIGHT
    }

    pub fn neighboring(&self, other: &AxialCoordinates) -> Option<Neighboring> {
        match *other - *self {
            Self::RIGHT => Some(Neighboring::Right),
            Self::LOWER_RIGHT => Some(Neighboring::LowerRight),
            Self::LOWER_LEFT => Some(Neighboring::LowerLeft),
            Self::LEFT => Some(Neighboring::Left),
            Self::UPPER_LEFT => Some(Neighboring::UpperLeft),
            Self::UPPER_RIGHT => Some(Neighboring::UpperRight),
            _ => None,
        }
    }
}

impl From<Vec2> for AxialCoordinates {
    // TODO! Note: rounds down.
    fn from(Vec2 { x, y }: Vec2) -> Self {
        let r = (y / -21.0) as i32;
        let q = ((x - r as f32 * 16.0) / 32.0) as i32;
        AxialCoordinates { q, r }
    }
}

impl From<Neighboring> for AxialCoordinates {
    fn from(value: Neighboring) -> Self {
        match value {
            Neighboring::Right => Self::RIGHT,
            Neighboring::LowerRight => Self::LOWER_RIGHT,
            Neighboring::LowerLeft => Self::LOWER_LEFT,
            Neighboring::Left => Self::LEFT,
            Neighboring::UpperLeft => Self::UPPER_LEFT,
            Neighboring::UpperRight => Self::UPPER_RIGHT,
        }
    }
}

impl From<AxialCoordinates> for Vec2 {
    fn from(val: AxialCoordinates) -> Self {
        Vec2 {
            x: val.q as f32 * 32.0 + val.r as f32 * 16.0,
            y: val.q as f32 * 0.0 + val.r as f32 * -21.0,
        }
    }
}

impl From<AxialCoordinates> for Vec3 {
    fn from(val: AxialCoordinates) -> Self {
        Vec2::from(val).extend(0.0)
    }
}

#[derive(Copy, Error, Debug, Clone, PartialEq, Eq)]
#[error("The provided tiles do not neighbor each other.")]
pub struct NotNeighboring;

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Neighboring {
    Right,
    LowerRight,
    LowerLeft,
    Left,
    UpperLeft,
    UpperRight,
}
