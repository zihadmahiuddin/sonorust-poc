use bevy::math::Mat4;
use sonorust_model::runtime::skin_transform::RuntimeSkinTransform;

use crate::blocks::MemoryRegion;

impl MemoryRegion for RuntimeSkinTransform {
    fn size(&self) -> usize {
        16
    }

    fn read(&self, index: usize) -> Option<f64> {
        if index >= 16 {
            panic!("Matrix index out of bounds: {index}");
        }

        let row = index / 4;
        let col = index % 4;
        let glam_col_major_index = col * 4 + row;

        let cols_array = self.0.to_cols_array();
        Some(cols_array[glam_col_major_index] as f64)
    }

    fn write(&mut self, index: usize, value: f64) {
        if index >= 16 {
            panic!("Matrix index out of bounds: {index}");
        }

        let row = index / 4;
        let col = index % 4;
        let glam_col_major_index = col * 4 + row;
        let mut cols_array = self.0.to_cols_array();
        cols_array[glam_col_major_index] = value as f32;
        self.0 = Mat4::from_cols_array(&cols_array);
    }
}
