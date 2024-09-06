use bevy::{
    a11y::AccessibilityPlugin,
    app::{PanicHandlerPlugin, Plugin},
    core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin},
    diagnostic::DiagnosticsPlugin,
    hierarchy::HierarchyPlugin,
    input::InputPlugin,
    log::LogPlugin,
    prelude::ReflectResource,
    state::{app::AppExtStates, state::States},
    time::TimePlugin,
    transform::TransformPlugin,
    window::WindowPlugin,
};
use main_menu_plugin::MainMenuPlugin;
use resources::SaveFilePath;

use crate::gameplay_plugin::GameplayPlugin;

mod gameplay_plugin;
mod main_menu_plugin;

mod cleanup;

mod resources;

pub struct MyGamePlugin;

impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // Assert DefaultPlugins added.
        debug_assert!(app.is_plugin_added::<LogPlugin>());
        debug_assert!(app.is_plugin_added::<TaskPoolPlugin>());
        debug_assert!(app.is_plugin_added::<TypeRegistrationPlugin>());
        debug_assert!(app.is_plugin_added::<FrameCountPlugin>());
        debug_assert!(app.is_plugin_added::<HierarchyPlugin>());
        debug_assert!(app.is_plugin_added::<DiagnosticsPlugin>());
        debug_assert!(app.is_plugin_added::<AccessibilityPlugin>());
        debug_assert!(app.is_plugin_added::<PanicHandlerPlugin>());
        debug_assert!(app.is_plugin_added::<InputPlugin>());
        debug_assert!(app.is_plugin_added::<WindowPlugin>());
        debug_assert!(app.is_plugin_added::<TransformPlugin>());
        debug_assert!(app.is_plugin_added::<TimePlugin>());

        app.add_plugins((MainMenuPlugin, GameplayPlugin));

        app.register_type::<SaveFilePath>()
            .register_type_data::<SaveFilePath, ReflectResource>();

        app.init_state::<GameStates>();
        app.enable_state_scoped_entities::<GameStates>();
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameStates {
    #[default]
    MainMenu,
    Gameplay,
}
