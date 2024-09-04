use bevy::{app::App, DefaultPlugins};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use strategy_game::MyGamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MyGamePlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
