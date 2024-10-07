use std::any;

use bevy::{
    app::App,
    prelude::{Commands, OnExit, Resource, States},
};

pub trait Cleanup {
    fn cleanup_resource<Res: Resource>(&mut self, on_exit: impl States) -> &mut Self;
}

impl Cleanup for App {
    fn cleanup_resource<Res: Resource>(&mut self, on_exit: impl States) -> &mut Self {
        self.add_systems(OnExit(on_exit), |mut commands: Commands| {
            commands.remove_resource::<Res>();
        });
        self
    }
}
