use wasm_bindgen::{prelude::*, convert::FromWasmAbi};

use crate::{rgb::RGB, vec3::Vec3};

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq)]
pub enum Shape {
    Sphere,
    Cube,
}

#[wasm_bindgen()]
#[derive(Copy, Clone, PartialEq)]
pub struct Entity {
    pub shape: Shape,
    pub position: Vec3,
    pub emission: RGB,
    pub reflectivity: RGB,
    pub roughness: f32,
}

#[wasm_bindgen]
impl Entity {
    #[wasm_bindgen(constructor)]
    pub fn new(shape: Shape, position: Vec3, emission: RGB, reflectivity: RGB, roughness: f32) -> Self {
        Self { shape, position, emission, reflectivity, roughness }
    }
}

#[wasm_bindgen]
pub struct Scene {
    entities: Vec<Entity>,
}

#[wasm_bindgen]
impl Scene {
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}
