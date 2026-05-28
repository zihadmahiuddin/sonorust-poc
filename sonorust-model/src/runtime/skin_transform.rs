use bevy::{ecs::resource::Resource, math::Mat4};

#[derive(Debug, Default, Resource)]
pub struct RuntimeSkinTransform(pub Mat4);

impl RuntimeSkinTransform {
    pub const ID: u16 = 1003;
}
