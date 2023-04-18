use wasm_bindgen::prelude::*;

use crate::{
    intersection::Intersection,
    rgb::RGB,
    vec3::{Ray, Vec3},
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub trait Intersectable {
    fn intersection(self, origin: Vec3, direction: Vec3) -> Option<Intersection>;
}

#[wasm_bindgen()]
#[derive(Copy, Clone, PartialEq)]
pub struct Entity {
    position: Vec3,
    radius: f32,
    material: Material,
}

#[wasm_bindgen()]
#[derive(Copy, Clone, PartialEq)]
pub struct Shape {}

#[wasm_bindgen()]
#[derive(Copy, Clone, PartialEq)]
pub struct Material {
    pub emission: RGB,
    pub reflectivity: RGB,
    pub roughness: f32,
}

impl Entity {
    pub fn intersection(self, ray: Ray) -> Option<Intersection> {
        let sphere_ray = self.position - ray.origin;
        let dist_sphere_ray = sphere_ray.mag();
        let dist_to_closest_point_on_ray = sphere_ray.dot(ray.direction);
        let dist_from_closest_point_to_sphere =
            (dist_sphere_ray.powi(2) - dist_to_closest_point_on_ray.powi(2)).sqrt();

        let dist_to_intersection = dist_to_closest_point_on_ray
            - (self.radius.powi(2) - dist_from_closest_point_to_sphere.powi(2))
                .abs()
                .sqrt();

        let point = ray.origin + (ray.direction * dist_to_intersection);
        //let roughness = Vec3::rng() * self.material.roughness;
        let normal = (point - self.position).normalize(); // + roughness;

        if dist_to_closest_point_on_ray > 0.0 && dist_from_closest_point_to_sphere < self.radius {
            return Some(Intersection {
                dist: dist_to_intersection,
                point: point,
                normal: normal,
                entity: Some(self),
            });
        }

        None
    }

    pub fn material(self) -> Material {
        self.material
    }
}

#[wasm_bindgen]
impl Entity {
    #[wasm_bindgen(constructor)]
    pub fn new(
        position: Vec3,
        emission: RGB,
        reflectivity: RGB,
        roughness: f32,
        radius: f32,
    ) -> Self {
        Self {
            position,
            radius,
            material: Material {
                emission,
                reflectivity,
                roughness,
            },
        }
    }
}
