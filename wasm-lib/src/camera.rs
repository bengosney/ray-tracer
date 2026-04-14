use wasm_bindgen::prelude::*;

use crate::vec3::Vec3;

#[wasm_bindgen]
pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec3,
    pub focal_length: u32,
    pub focal_distance: u32,
    pub aperture: f32,
}

#[wasm_bindgen]
impl Camera {
    #[wasm_bindgen(constructor)]
    pub fn new(position: Vec3, rotation: Vec3, focal_length: u32, focal_distance: u32, aperture: f32) -> Self {
        Self {
            position,
            rotation,
            focal_length,
            focal_distance,
            aperture,
        }
    }
}
