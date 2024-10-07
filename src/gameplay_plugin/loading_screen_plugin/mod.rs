use bevy::{app::Plugin, prelude::OnEnter};
use load_game_plugin::LoadGamePlugin;
// use load_game_plugin::LoadGamePlugin;
use systems::setup;

use super::GameplayStates;

mod load_game_plugin;

mod systems;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(LoadGamePlugin);

        app.add_systems(OnEnter(GameplayStates::LoadingScreen), setup);
    }
}
