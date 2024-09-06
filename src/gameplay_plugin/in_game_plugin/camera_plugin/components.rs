use bevy::{ecs::component::Component, reflect::Reflect};

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
pub(super) struct MainCamera;
