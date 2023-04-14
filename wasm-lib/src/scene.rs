use wasm_bindgen::prelude::*;

use crate::{
    rgb::RGB,
    vec3::Vec3,
};

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
    pub fn new(
        shape: Shape,
        position: Vec3,
        emission: RGB,
        reflectivity: RGB,
        roughness: f32,
    ) -> Self {
        Self {
            shape,
            position,
            emission,
            reflectivity,
            roughness,
        }
    }
}

#[wasm_bindgen]
pub struct Scene {
    entities: Vec<Entity>,
    pub width: u32,
    pub height: u32,
    pub focal_length: u32,
    pub samples: u32,
    pub bounces: u32,
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, focal_length: u32, samples: u32, bounces: u32) -> Self {
        Self {
            entities: vec![],
            width,
            height,
            focal_length,
            samples,
            bounces,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn render(&self) -> String {
        let halfWidth: u32 = self.width / 2;
        let halfHeight: u32 = self.height / 2;

        let origin: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut samples: Vec<Vec<Vec<Vec3>>> = vec![vec![vec![]; self.height as usize]; self.width as usize];

        for _ in 0..self.samples {
            for i in 0..self.width {
                for j in 0..self.height {
                    let x = i - halfWidth;
                    let y = j - halfHeight;
                    let direction = (Vec3 {
                        x: x as f32,
                        y: y as f32,
                        z: self.focal_length as f32,
                    })
                    .normalize();

                    samples[x as usize][y as usize].push(direction);
                    //let colour = avg(samples[x][y]);

                    //drawPixel({ x: i, y: j }, vec3ToRGB(colour));
                }
            }
        }

        "Rendering".to_string()
    }
}
