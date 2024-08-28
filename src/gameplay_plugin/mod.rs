use core::slice;

use bevy::{
    app::{Plugin, Startup},
    asset::{Asset, AssetServer, Assets},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Res, ResMut},
        world::World,
    },
    math::{Vec2, Vec3},
    render::{
        render_resource::Texture,
        texture::Image,
        view::{InheritedVisibility, ViewVisibility, Visibility},
    },
    sprite::{Sprite, SpriteBundle},
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
pub(super) struct NeighbouringTiles {
    upper: Option<Entity>,
    upper_right: Option<Entity>,
    lower_right: Option<Entity>,
    lower: Option<Entity>,
    lower_left: Option<Entity>,
    upper_left: Option<Entity>,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct TileConnection(Entity, Entity);

fn spawn_map_randomly(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tiles = Box::new([
        ("forest_tile.png", (0, 0)),
        ("water_tile.png", (0, 1)),
        ("house_tile.png", (1, 0)),
        ("road_tile.png", (1, -1)),
        ("water_tile.png", (0, -1)),
        ("forest_tile.png", (-1, 0)),
        ("house_tile.png", (-1, 1)),
    ]);
    let tile_connections = Box::new([
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (0, 5),
        (0, 6),
        (1, 2),
        (1, 6),
        (2, 3),
        (3, 4),
        (4, 5),
        (5, 6),
    ]);

    let tiles_entities = tiles
        .into_iter()
        .map(|(image_path, axial_coordinates)| {
            (
                commands
                    .spawn(SpriteBundle {
                        transform: Transform::from_translation(axial_coordinates_to_translation(
                            axial_coordinates.0,
                            axial_coordinates.1,
                        )),
                        texture: asset_server.load(image_path),
                        ..Default::default()
                    })
                    .id(),
                NeighbouringTiles::default(),
            )
        })
        .collect::<Box<[(Entity, NeighbouringTiles)]>>();

    for ((entity_0, neighbouring_tiles_0), (entity_1, neighbouring_tiles_1)) in tile_connections
        .into_iter()
        .map(|(entity_0, entity_1)| (&tiles_entities[entity_0], &tiles_entities[entity_1]))
    {
        assert_ne!(entity_0, entity_1);
        commands.spawn(TileConnection(*entity_0, *entity_1));
        // neighbouring_tiles_0.
    }
}

fn axial_coordinates_to_translation(q: i32, r: i32) -> Vec3 {
    let q = q as f32;
    let r = r as f32;
    Vec3 {
        x: q * 32.0 + r * 16.0,
        y: q * 0.0 + r * 21.0,
        z: 0.0,
    }
}
