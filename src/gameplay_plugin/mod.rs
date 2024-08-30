use core::slice;

use bevy::{
    app::{Plugin, Startup},
    asset::{Asset, AssetServer, Assets, Handle},
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
        world::World,
    },
    math::{
        primitives::{Rectangle, Segment2d},
        Quat, Vec2, Vec3,
    },
    render::{
        mesh::{Capsule2dMeshBuilder, Mesh},
        render_resource::Texture,
        texture::Image,
        view::{InheritedVisibility, ViewVisibility, Visibility},
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle, Sprite, SpriteBundle},
    transform::components::{GlobalTransform, Transform},
};

use self::camera_plugin::CameraPlugin;

mod camera_plugin;

pub(super) struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(CameraPlugin)
            .add_systems(Startup, spawn_map_randomly);
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(super) struct NeighboringTiles {
    right: Option<Entity>,
    upper_right: Option<Entity>,
    lower_right: Option<Entity>,
    left: Option<Entity>,
    lower_left: Option<Entity>,
    upper_left: Option<Entity>,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct TileConnection(Entity, Entity);

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(super) struct NeighboringTilesSaveFile {
    right: Option<usize>,
    upper_right: Option<usize>,
    lower_right: Option<usize>,
    left: Option<usize>,
    lower_left: Option<usize>,
    upper_left: Option<usize>,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct TileConnectionSaveFile(usize, usize);

fn spawn_map_randomly(mut commands: Commands, asset_server: Res<AssetServer>) {
    let saved_tiles = Box::new([
        (
            "forest_tile.png",
            NeighboringTilesSaveFile {
                lower_right: Some(0),
                lower_left: Some(1),
                ..Default::default()
            },
        ),
        (
            "forest_tile.png",
            NeighboringTilesSaveFile {
                upper_left: Some(0),
                lower_left: Some(2),
                ..Default::default()
            },
        ),
        (
            "forest_tile.png",
            NeighboringTilesSaveFile {
                upper_right: Some(1),
                lower_right: Some(3),
                ..Default::default()
            },
        ),
        (
            "forest_tile.png",
            NeighboringTilesSaveFile {
                upper_right: Some(2),
                upper_left: Some(3),
                ..Default::default()
            },
        ),
    ]);

    let saved_tile_connections = Box::new([
        TileConnectionSaveFile(0, 1),
        TileConnectionSaveFile(0, 2),
        TileConnectionSaveFile(1, 3),
        TileConnectionSaveFile(2, 3),
    ]);

    let mut translations: Box<[Option<Vec2>]> = vec![None; saved_tiles.len()].into_boxed_slice();
    let mut current_translation = Vec2::new(0.0, 0.0);
    let mut tiles_todo = vec![0];

    loop {
        let current_tile = match tiles_todo.pop() {
            Some(current_tile) => todo!(),
            None => todo!(),
        };
    }
}
