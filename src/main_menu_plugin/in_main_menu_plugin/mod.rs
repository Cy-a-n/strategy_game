use bevy::{app::Plugin, prelude::OnEnter};
use systems::setup;

use super::MainMenuStates;

mod systems;

pub struct InMainMenuPlugin;

impl Plugin for InMainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(MainMenuStates::InMainMenu), setup);
    }
}
