use bevy::{
    color::Color,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        query::With,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::Vec3,
    render::camera::{Camera, ClearColorConfig, OrthographicProjection},
    state::state_scoped::StateScoped,
    time::Time,
    transform::components::Transform,
};

use crate::gameplay_plugin::GameplayStates;

use super::components::MainCamera;

pub(super) fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::from(Color::BLACK),
                ..Default::default()
            },
            ..Default::default()
        },
        MainCamera,
        StateScoped(GameplayStates::InGame),
    ));
}

pub(super) fn camera_controller(
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    const MOVEMENT_SPEED_PX: f32 = 64.0;
    const ZOOM_SPEED_MUL: f32 = 0.5;
    let (mut transform, mut projection) = camera.single_mut();
    let delta_sec = time.delta_seconds();

    let mut zoom = 0.0;

    if keys.pressed(KeyCode::KeyQ) {
        zoom -= 1.0;
    }
    if keys.pressed(KeyCode::KeyE) {
        zoom += 1.0;
    }

    projection.scale *= 1.0 + zoom * ZOOM_SPEED_MUL * delta_sec;

    let mut movement_direction = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyA) {
        movement_direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        movement_direction.x += 1.0;
    };
    if keys.pressed(KeyCode::KeyS) {
        movement_direction.y -= 1.0;
    };
    if keys.pressed(KeyCode::KeyW) {
        movement_direction.y += 1.0;
    };

    transform.translation += movement_direction * MOVEMENT_SPEED_PX * delta_sec * projection.scale;
}
