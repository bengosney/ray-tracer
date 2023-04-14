mod entity;
mod intersection;
mod rgb;
mod scene;
mod vec2;
mod vec3;

use rgb::RGB;
use scene::Scene;
use vec3::Vec3;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn bob(by: f32) -> Vec3 {
    let a: Vec3 = Vec3 {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };
    let b: Vec3 = Vec3 {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };

    let colour: RGB = RGB {
        r: 1.0,
        g: 2.0,
        b: 3.0,
    };

    let c: Vec3 = (a * b) * by;

    log(&format!("c = <{}, {}, {}>", c.x, c.y, c.z));

    Vec3::from(colour) * 4.0
}

#[wasm_bindgen]
pub fn render(_scene: Scene) {
    log(&format!("might work at some point:"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
