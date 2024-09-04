use bevy::prelude::{NextState, ResMut};

use crate::{main_menu_plugin::MainMenuStates, GameStates};

pub fn setup(
    mut main_menu_states: ResMut<NextState<MainMenuStates>>,
    mut game_states: ResMut<NextState<GameStates>>,
) {
    game_states.as_mut().set(GameStates::Gameplay);
    main_menu_states.as_mut().set(MainMenuStates::None);
}
