use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::OffscreenCanvasRenderingContext2d;

use crate::camera::Camera;
use crate::entity::Entity;
use crate::material::Material;
use crate::model::Model;
use crate::post_processing::{GammaCorrection, ImageFilter, PostProcess};
use crate::renderer;
use crate::vec3::Vec3;

#[wasm_bindgen]
pub struct Scene {
    entities: Vec<Entity>,
    camera: Camera,
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub bounces: u32,
    post_processors: Vec<Rc<dyn PostProcess>>,
}

impl Scene {
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn post_processors(&self) -> &[Rc<dyn PostProcess>] {
        &self.post_processors
    }
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, camera: Camera, samples: u32, bounces: u32) -> Self {
        Self {
            entities: vec![],
            camera,
            width,
            height,
            samples,
            bounces,
            post_processors: vec![],
        }
    }

    pub fn add_filter(&mut self, filter: ImageFilter) {
        self.post_processors.push(Rc::new(filter.into_kernel()));
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn set_gamma_correction(&mut self, gamma: f32) {
        self.post_processors
            .retain(|p| p.as_any().downcast_ref::<GammaCorrection>().is_none());
        self.post_processors.push(Rc::new(GammaCorrection::new(gamma)));
    }

    pub fn render(&self, ctx: &OffscreenCanvasRenderingContext2d, on_sample: js_sys::Function) {
        renderer::render(self, ctx, on_sample);
    }

    pub fn load_model(&mut self, text: &str, position: Vec3, rotation: Vec3, scale: f32, material: Material) {
        let model = Model::parse(text);
        for (a, b, c) in model.triangles() {
            let a = a.rotate_vec(rotation) * scale;
            let b = b.rotate_vec(rotation) * scale;
            let c = c.rotate_vec(rotation) * scale;
            let entity = Entity::new_triangle(position, a, b, c, material);
            self.add_entity(entity);
        }
    }
}
