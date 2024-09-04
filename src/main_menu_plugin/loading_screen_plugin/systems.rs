use bevy::prelude::{NextState, ResMut};

use crate::main_menu_plugin::MainMenuStates;

pub fn setup(mut main_menu_states: ResMut<NextState<MainMenuStates>>) {
    main_menu_states.as_mut().set(MainMenuStates::InMainMenu);
}
