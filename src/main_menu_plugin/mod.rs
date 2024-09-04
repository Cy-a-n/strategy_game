use bevy::{
    app::Plugin,
    prelude::{AppExtStates, OnEnter, States},
    reflect::Reflect,
};
use in_main_menu_plugin::InMainMenuPlugin;
use loading_screen_plugin::LoadingScreenPlugin;
use systems::setup;

use crate::GameStates;

mod in_main_menu_plugin;
mod loading_screen_plugin;
mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(LoadingScreenPlugin);
        app.add_plugins(InMainMenuPlugin);
        app.init_state::<MainMenuStates>();
        app.add_systems(OnEnter(GameStates::MainMenu), setup);
    }
}

#[derive(States, Reflect, Default, Hash, Debug, Clone, PartialEq, Eq)]
enum MainMenuStates {
    #[default]
    None,
    LoadingScreen,
    InMainMenu,
}
