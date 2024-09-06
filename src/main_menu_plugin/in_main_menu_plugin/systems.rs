use bevy::prelude::{Commands, NextState, ResMut};

use crate::{resources::SaveFilePath, GameStates};

pub fn setup(mut commands: Commands, mut game_states: ResMut<NextState<GameStates>>) {
    commands.insert_resource(SaveFilePath::new("save_files/scenarios/test_map".into()));
    game_states.as_mut().set(GameStates::Gameplay);
}
