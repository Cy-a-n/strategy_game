use bevy::{app::App, DefaultPlugins};
use strategy_game::MyGamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MyGamePlugin)
        .run();
}
