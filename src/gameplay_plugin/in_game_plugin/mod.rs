use bevy::{
    app::{Plugin, Update},
    input::ButtonInput,
    prelude::{in_state, IntoSystemConfigs, KeyCode, NextState, OnEnter, Res, ResMut},
};

use systems::setup;

use crate::GameStates;

use self::camera_plugin::CameraPlugin;

use super::GameplayStates;

mod camera_plugin;
mod systems;

pub(super) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(CameraPlugin);

        app.add_systems(OnEnter(GameplayStates::InGame), setup)
            .add_systems(
                Update,
                (|keys: Res<ButtonInput<KeyCode>>,
                  mut game_states: ResMut<NextState<GameStates>>| {
                    if keys.as_ref().pressed(KeyCode::Escape) {
                        game_states.as_mut().set(GameStates::MainMenu)
                    }
                })
                .run_if(in_state(GameplayStates::InGame)),
            );
    }
}
