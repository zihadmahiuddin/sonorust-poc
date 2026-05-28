use bevy::{ecs::resource::Resource, math::DMat4};

#[derive(Debug, Default, Resource)]
pub struct RuntimeParticleTransform(pub DMat4);

impl RuntimeParticleTransform {
    pub const ID: u16 = 1004;
}
