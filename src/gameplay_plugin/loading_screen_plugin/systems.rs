use bevy::{
    color::Color,
    prelude::{Camera, Camera2dBundle, Commands, StateScoped},
};

use crate::gameplay_plugin::GameplayStates;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                ..Default::default()
            },
            ..Default::default()
        },
        StateScoped(GameplayStates::LoadingScreen),
    ));
}
