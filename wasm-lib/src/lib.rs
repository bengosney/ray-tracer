mod aabb;
mod entity;
mod intersection;
mod material;
mod plane;
mod post_processing;
mod ray;
mod renderer;
mod rgb;
mod scene;
mod sphere;
mod tracer;
mod vec2;
mod vec3;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
