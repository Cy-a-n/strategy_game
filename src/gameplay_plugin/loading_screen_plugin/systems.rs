use bevy::prelude::{NextState, ResMut};

use crate::gameplay_plugin::GameplayStates;

pub fn setup(mut gameplay_states: ResMut<NextState<GameplayStates>>) {
    gameplay_states.as_mut().set(GameplayStates::InGame);
}
