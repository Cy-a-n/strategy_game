use bevy::{
    app::{Plugin, Update},
    prelude::{in_state, IntoSystemConfigs, OnEnter},
};
use systems::{setup, temporary_main_menu};

use super::MainMenuStates;

mod systems;

pub struct InMainMenuPlugin;

impl Plugin for InMainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(MainMenuStates::InMainMenu), setup)
            .add_systems(
                Update,
                temporary_main_menu.run_if(in_state(MainMenuStates::InMainMenu)),
            );
    }
}
