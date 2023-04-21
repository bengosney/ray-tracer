mod entity;
mod intersection;
mod rgb;
mod scene;
mod vec2;
mod vec3;
mod camera;
mod convolutions;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
