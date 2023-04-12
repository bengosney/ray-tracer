use wasm_bindgen::prelude::*;

use crate::{rgb::RGB, vec3::Vec3};

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq)]
pub enum Shape {
    Sphere,
    Cube,
}

#[wasm_bindgen()]
#[derive(Copy, Clone, PartialEq)]
pub struct Object {
    pub shape: Shape,
    pub position: Vec3,
    pub emission: RGB,
    pub reflectivity: RGB,
    pub roughness: f32,
}
