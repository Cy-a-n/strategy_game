use bevy::{
    app::{Plugin, Update},
    prelude::{in_state, resource_exists, IntoSystemConfigs, OnEnter},
};
use resources::LoadFromFileSuccessful;
use systems::{check_if_loaded, load_from_file};

use crate::{cleanup::Cleanup, gameplay_plugin::GameplayStates};

mod resources;
mod systems;

pub struct LoadGamePlugin;

impl Plugin for LoadGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameplayStates::LoadingScreen), load_from_file)
            .add_systems(
                Update,
                check_if_loaded
                    .run_if(in_state(GameplayStates::LoadingScreen))
                    .run_if(resource_exists::<LoadFromFileSuccessful>),
            )
            .cleanup_resource::<LoadFromFileSuccessful>(GameplayStates::LoadingScreen);
    }
}
