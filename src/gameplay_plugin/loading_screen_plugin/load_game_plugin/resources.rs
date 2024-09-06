use bevy::{asset::UntypedHandle, prelude::Resource};

// Cannot be reflected because `UntypedHandle` doesn't implement it.
#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct LoadFromFileSuccessful {
    pub assets_to_load: Vec<UntypedHandle>,
}
