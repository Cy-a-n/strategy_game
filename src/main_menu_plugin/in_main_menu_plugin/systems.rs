use std::fs;

use bevy::{
    color::Color,
    prelude::{Camera, Camera2dBundle, Commands, NextState, StateScoped},
};
use bevy::prelude::*;
use bevy_egui::egui::{ScrollArea, SidePanel};
use bevy_egui::EguiContext;
use bevy_window::PrimaryWindow;

use crate::{main_menu_plugin::MainMenuStates, resources::SaveFilePath, GameStates};

pub fn setup(mut commands: Commands) {
    // A temporary mockup of a main menu. Will be completely reworked later on.
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: Color::BLACK.into(),
                ..Default::default()
            },
            ..Default::default()
        },
        StateScoped(MainMenuStates::InMainMenu),
    ));
}

pub fn temporary_main_menu(world: &mut World) {
    let ctx = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
        .unwrap();
    SidePanel::left("temporary_main_menu")
        .resizable(true)
        .show(ctx.clone().get_mut(), |ui| {
            ScrollArea::both().show(ui, |ui| {
                for entry in fs::read_dir("./assets/save_files/scenarios").unwrap() {
                    let path = entry.unwrap().path();
                    if path.is_dir() {
                        let dir_name = path.file_name().unwrap().to_string_lossy();
                        if ui.button(dir_name.clone()).clicked() {
                            world.insert_resource(SaveFilePath::new(format!(
                                "save_files/scenarios/{dir_name}"
                            )));
                            world
                                .get_resource_mut::<NextState<GameStates>>()
                                .unwrap()
                                .as_mut()
                                .set(GameStates::Gameplay);
                            return;
                        };
                    }
                }
                ui.allocate_space(ui.available_size());
            });
        });
}
