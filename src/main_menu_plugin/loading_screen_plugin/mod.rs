use bevy::{app::Plugin, prelude::OnEnter};
use systems::setup;

use super::MainMenuStates;

mod systems;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(MainMenuStates::LoadingScreen), setup);
    }
}
