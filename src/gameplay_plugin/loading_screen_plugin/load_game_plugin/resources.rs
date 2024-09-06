use bevy::{asset::UntypedHandle, prelude::Resource};

#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct LoadFromFileSuccessful {
    pub assets_to_load: Vec<UntypedHandle>,
}
