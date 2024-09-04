use bevy::{ecs::entity::Entity, prelude::Resource, reflect::Reflect, utils::HashMap};

use super::components::AxialCoordinates;

pub type TileEntity = Entity;

#[derive(Resource, Reflect, Debug, Clone, PartialEq, Eq)]
pub struct TilesByCoordinates(pub HashMap<AxialCoordinates, TileEntity>);
