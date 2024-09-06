use bevy::{
    app::Plugin,
    prelude::{AppExtStates, StateSet, SubStates},
    reflect::Reflect,
};
use in_main_menu_plugin::InMainMenuPlugin;
use loading_screen_plugin::LoadingScreenPlugin;

use crate::GameStates;

mod in_main_menu_plugin;
mod loading_screen_plugin;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((LoadingScreenPlugin, InMainMenuPlugin));

        app.add_sub_state::<MainMenuStates>();
        app.enable_state_scoped_entities::<MainMenuStates>();
    }
}

#[derive(SubStates, Reflect, Default, Hash, Debug, Clone, PartialEq, Eq)]
#[source(GameStates = GameStates::MainMenu)]
enum MainMenuStates {
    #[default]
    LoadingScreen,
    InMainMenu,
}
