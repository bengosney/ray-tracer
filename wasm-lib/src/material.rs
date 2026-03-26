use wasm_bindgen::prelude::*;

use crate::rgb::Rgb;

#[wasm_bindgen()]
#[derive(Copy, Clone, PartialEq)]
pub struct Material {
    pub emission: Rgb,
    pub albedo: Rgb,
    pub metallic: f32,
    pub roughness: f32,
}
