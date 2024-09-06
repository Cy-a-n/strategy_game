use bevy::prelude::{Reflect, Resource};

#[derive(Resource, Reflect, Debug, Clone, PartialEq, Eq)]
pub struct SaveFilePath {
    relative_to_assets: String,
}

impl SaveFilePath {
    pub fn new(relative_to_assets: String) -> Self {
        Self { relative_to_assets }
    }

    pub fn relative_to_assets(&self) -> &str {
        &self.relative_to_assets
    }
}
