use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::entity::Entity;
use crate::post_processing::{GammaCorrection, ImageFilter, PostProcess};
use crate::renderer;

#[wasm_bindgen]
pub struct Scene {
    entities: Vec<Entity>,
    pub width: u32,
    pub height: u32,
    pub focal_length: u32,
    pub focal_distance: u32,
    pub appature: f32,
    pub samples: u32,
    pub bounces: u32,
    pub fov: f32,
    post_processors: Vec<Rc<dyn PostProcess>>,
}

impl Scene {
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    pub fn post_processors(&self) -> &[Rc<dyn PostProcess>] {
        &self.post_processors
    }
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        width: u32,
        height: u32,
        focal_length: u32,
        focal_distance: u32,
        appature: f32,
        samples: u32,
        bounces: u32,
        fov: f32,
    ) -> Self {
        Self {
            entities: vec![],
            width,
            height,
            focal_length,
            focal_distance,
            appature,
            samples,
            bounces,
            fov,
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

    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        renderer::render(self, ctx);
    }
}
