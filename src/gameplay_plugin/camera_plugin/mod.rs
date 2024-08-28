use bevy::{
    app::{Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
    state::condition::in_state,
};

use crate::GameStates;

use self::systems::{camera_controller, setup};

mod components;
mod systems;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup.run_if(in_state(GameStates::Gameplay)))
            .add_systems(
                Update,
                camera_controller.run_if(in_state(GameStates::Gameplay)),
            );
    }
}
