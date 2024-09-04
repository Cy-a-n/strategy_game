use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
    prelude::OnEnter,
    state::condition::in_state,
};

use crate::gameplay_plugin::GameplayStates;

use self::systems::{camera_controller, setup};

mod components;
mod systems;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameplayStates::InGame), setup)
            .add_systems(
                Update,
                camera_controller.run_if(in_state(GameplayStates::InGame)),
            );
    }
}
