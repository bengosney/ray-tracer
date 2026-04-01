use wasm_bindgen::prelude::*;

use crate::rgb::Rgb;

#[wasm_bindgen()]
#[derive(Copy, Clone, PartialEq)]
pub struct Material {
    pub emission: Rgb,
    pub albedo: Rgb,
    pub metallic: f32,
    pub roughness: f32,
    pub transmission: f32,
    pub ior: f32,
}

#[wasm_bindgen()]
impl Material {
    #[wasm_bindgen(constructor)]
    pub fn new(emission: Rgb, albedo: Rgb, metallic: f32, roughness: f32, transmission: f32, ior: f32) -> Material {
        Material {
            emission,
            albedo,
            metallic,
            roughness,
            transmission,
            ior,
        }
    }
}
