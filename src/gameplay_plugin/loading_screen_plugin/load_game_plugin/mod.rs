use bevy::{app::Plugin, prelude::OnEnter};
use systems::load_from_file;

use crate::gameplay_plugin::GameplayStates;

mod systems;

pub struct LoadGamePlugin;

impl Plugin for LoadGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameplayStates::LoadingScreen), load_from_file);
    }
}
