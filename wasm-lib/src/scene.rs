use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use crate::rgb::RGB;
use crate::vec3::Ray;
use crate::{entity::Entity, intersection::Intersection, vec3::Vec3};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub struct Scene {
    entities: Vec<Entity>,
    pub width: u32,
    pub height: u32,
    pub focal_length: u32,
    pub samples: u32,
    pub bounces: u32,
    pub background: Vec3,
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new(
        width: u32,
        height: u32,
        focal_length: u32,
        samples: u32,
        bounces: u32,
        background: RGB,
    ) -> Self {
        Self {
            entities: vec![],
            width,
            height,
            focal_length,
            samples,
            bounces,
            background: background.into(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    fn intersection(ray: Ray, entities: Vec<Entity>) -> Option<Intersection> {
        let intersection = entities
            .iter()
            .fold(Intersection::empty(), |previous, entity| {
                match entity.intersection(ray.clone()) {
                    Some(intersection) => Intersection::closest(intersection, previous),
                    None => previous,
                }
            });

        match intersection {
            Intersection { entity: None, .. } => None,
            intersection => Some(intersection),
        }
    }

    fn trace(ray: Ray, entities: Vec<Entity>, background: Vec3, steps: u32) -> Vec3 {
        match Self::intersection(ray, entities.clone()) {
            Some(intersection) if steps > 0 => {
                let reflected_direction: Vec3 = ray.direction.reflect(intersection.normal);
                let entity: Entity = intersection.entity.unwrap();

                let bounce = Self::trace(
                    Ray {
                        origin: intersection.point,
                        direction: reflected_direction,
                    },
                    entities,
                    background,
                    steps - 1,
                );

                let material = entity.material();
                Vec3::from(material.emission) + (bounce * Vec3::from(material.reflectivity))
            }
            _ => background,
        }
    }

    fn samples_to_pixel_map(samples: &Vec<Vec<Vec<Vec3>>>) -> Vec<u8> {
        samples.iter().fold(Vec::new(), |acc: Vec<u8>, row| {
            [
                acc,
                row.iter()
                    .map(|sample_group| Vec3::avg(&sample_group))
                    .map(|sample| vec![sample.x as u8, sample.y as u8, sample.z as u8, 255])
                    .fold(Vec::new(), |acc: Vec<u8>, sample| [acc, sample].concat()),
            ]
            .concat()
        })
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        let half_width: i32 = (self.width / 2) as i32;
        let half_height: i32 = (self.height / 2) as i32;

        let width = self.width;
        let height = self.height;
        let focal_length = self.focal_length;
        let bounces = self.bounces;
        let entities = self.entities.clone();
        let sample_count = self.samples;
        let background = self.background;

        let local_context = ctx.clone();

        let origin: Vec3 = Vec3::zero();
        let mut samples: Vec<Vec<Vec<Vec3>>> =
            vec![vec![vec![]; self.width as usize]; self.height as usize];

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let mut i = 0;
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if i > sample_count {
                log("done.");
                let _ = f.borrow_mut().take();
                return;
            }
            log(&format!("Sample {}", i));
            for i in 0..width as i32 {
                for j in 0..height as i32 {
                    let x: i32 = i - half_width;
                    let y: i32 = j - half_height;
                    let direction = (Vec3 {
                        x: x as f32,
                        y: y as f32,
                        z: focal_length as f32,
                    })
                    .normalize();

                    let res = Self::trace(
                        Ray { origin, direction },
                        entities.clone(),
                        background,
                        bounces,
                    );
                    samples[j as usize][i as usize].push(res);
                }
            }

            let mut data = Scene::samples_to_pixel_map(&samples);
            let image_data =
                ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)
                    .unwrap();
            local_context.put_image_data(&image_data, 0.0, 0.0).ok();

            i += 1;
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}
