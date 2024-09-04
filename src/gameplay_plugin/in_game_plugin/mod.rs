use bevy::{
    app::{Plugin, Startup},
    asset::{AssetApp, Handle, ReflectAsset, ReflectHandle},
    prelude::{AppExtStates, OnEnter, ReflectComponent, ReflectResource, States},
    reflect::Reflect,
};

use systems::setup;

use self::camera_plugin::CameraPlugin;

use super::GameplayStates;

mod camera_plugin;
mod systems;

pub(super) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(CameraPlugin);

        app.add_systems(OnEnter(GameplayStates::InGame), setup);
    }
}
