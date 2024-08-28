use bevy::{
    a11y::AccessibilityPlugin,
    app::{PanicHandlerPlugin, Plugin},
    core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin},
    diagnostic::DiagnosticsPlugin,
    hierarchy::HierarchyPlugin,
    input::InputPlugin,
    log::LogPlugin,
    state::{app::AppExtStates, state::States},
    time::TimePlugin,
    transform::TransformPlugin,
    window::WindowPlugin,
};

use crate::gameplay_plugin::GameplayPlugin;

mod gameplay_maps;
mod gameplay_plugin;

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

        app.add_plugins(GameplayPlugin);
        app.init_state::<GameStates>();
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameStates {
    LoadingMainMenu,
    MainMenu,
    LoadingGameplay,
    #[default]
    Gameplay,
}
