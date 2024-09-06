use bevy::{
    color::Color,
    prelude::{Camera, Camera2dBundle, Commands, NextState, ResMut, StateScoped},
};

use crate::main_menu_plugin::MainMenuStates;

pub fn setup(mut commands: Commands, mut main_menu_states: ResMut<NextState<MainMenuStates>>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: Color::WHITE.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        StateScoped(MainMenuStates::LoadingScreen),
    ));

    main_menu_states.as_mut().set(MainMenuStates::InMainMenu);
}
