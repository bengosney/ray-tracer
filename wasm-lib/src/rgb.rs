use wasm_bindgen::prelude::*;

use crate::vec3::Vec3;

#[wasm_bindgen()]
#[derive(Copy, Clone, PartialEq)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[wasm_bindgen]
impl Rgb {
    #[wasm_bindgen(constructor)]
    pub fn new(r: f32, g: f32, b: f32) -> Rgb {
        Rgb { r, g, b }
    }
}

impl From<Vec3> for Rgb {
    fn from(vec: Vec3) -> Self {
        Rgb {
            r: vec.x,
            g: vec.y,
            b: vec.z,
        }
    }
}
