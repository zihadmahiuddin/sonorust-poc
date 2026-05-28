use bevy::math::DMat4;
use sonorust_model::runtime::particle_transform::RuntimeParticleTransform;

use crate::blocks::MemoryRegion;

impl MemoryRegion for RuntimeParticleTransform {
    fn size(&self) -> usize {
        16
    }

    fn read(&self, index: usize) -> Option<f64> {
        if index < 16 {
            let cols = self.0.to_cols_array();
            Some(cols[index])
        } else {
            panic!("Matrix index out of bounds: {index}");
        }
    }

    fn write(&mut self, index: usize, value: f64) {
        if index < 16 {
            let mut cols = self.0.to_cols_array();
            cols[index] = value;
            self.0 = DMat4::from_cols_array(&cols);
        } else {
            panic!("Matrix index out of bounds: {index}");
        }
    }
}
