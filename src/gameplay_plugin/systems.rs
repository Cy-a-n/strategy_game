use bevy::prelude::{NextState, ResMut};

use super::GameplayStates;

pub fn setup(mut gameplay_states: ResMut<NextState<GameplayStates>>) {
    gameplay_states.as_mut().set(GameplayStates::LoadingScreen);
}
