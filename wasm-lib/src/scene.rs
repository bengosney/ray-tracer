use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};


use crate::{entity::Entity, intersection::Intersection, vec3::Vec3};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
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

    fn intersection(origin: Vec3, direction: Vec3, entities: Vec<Entity>) -> Intersection {
        let mut closest_intersection: Intersection = Intersection::empty();

        for entity in entities {
            let intersection = entity.intersection(origin, direction);
            if intersection.dist < closest_intersection.dist {
                closest_intersection = intersection;
            }
        }

        return closest_intersection;
    }

    fn trace(origin: Vec3, direction: Vec3, entities: Vec<Entity>, steps: u32) -> Vec3 {
        let intersect = Self::intersection(origin, direction, entities.clone());

        if intersect.collided && steps > 0 {
            let reflected_direction = direction.reflect(intersect.normal);
            let entity = intersect.entity.unwrap();

            let filtered_entities: Vec<Entity> = entities.into_iter()
                .filter(|e| e != &entity)
                .collect();

            let bounce = Self::trace(
                intersect.point,
                reflected_direction,
                filtered_entities,
                steps - 1,
            );

            return Vec3::from(entity.emission) + (bounce * Vec3::from(entity.reflectivity));
        }

        return Vec3::new(0.0, 0.0, 0.0);
    }

    fn samples_to_pixel_map(samples: &Vec<Vec<Vec<Vec3>>>) -> Vec<u8> {
        let mut pixels: Vec<u8> = vec![];

        for row in samples {
            for sample_group in row {
                let sample = Vec3::avg(&sample_group);

                pixels.push(sample.x as u8);
                pixels.push(sample.y as u8);
                pixels.push(sample.z as u8);
                pixels.push(255);
            }
        }

        return pixels;
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        let half_width: i32 = (self.width / 2) as i32;
        let half_height: i32 = (self.height / 2) as i32;

        let origin: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut samples: Vec<Vec<Vec<Vec3>>> =
            vec![vec![vec![]; self.width as usize]; self.height as usize];

        for pass in 0..self.samples {
            log(&format!("Pass {}", pass));
            for i in 0..self.width as i32 {
                for j in 0..self.height as i32 {
                    let x: i32 = i - half_width;
                    let y: i32 = j - half_height;
                    let direction = (Vec3 {
                        x: x as f32,
                        y: y as f32,
                        z: self.focal_length as f32,
                    })
                    .normalize();

                    let res = Self::trace(origin, direction, self.entities.clone(), self.bounces);

                    samples[j as usize][i as usize].push(res);
                }
            }
            let mut data = Scene::samples_to_pixel_map(&samples);
            let data_result = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), self.width, self.height);
            let data = data_result.unwrap();
            ctx.put_image_data(&data, 0.0, 0.0);
        }
    }
}
