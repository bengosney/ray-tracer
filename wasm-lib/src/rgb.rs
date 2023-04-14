use wasm_bindgen::prelude::*;

use crate::vec3::Vec3;

pub type RGBA_tuple = (u16, u16, u16, u16);

#[wasm_bindgen()]
#[derive(Copy, Clone, PartialEq)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[wasm_bindgen]
impl RGB {
    #[wasm_bindgen(constructor)]
    pub fn new(r: f32, g: f32, b: f32) -> RGB {
        RGB { r: r, g: g, b: b }
    }
}

impl From<Vec3> for RGB {
    fn from(vec: Vec3) -> Self {
        RGB {
            r: vec.x,
            g: vec.y,
            b: vec.z,
        }
    }
}
