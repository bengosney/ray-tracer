use wasm_bindgen::prelude::*;

use crate::{intersection::Intersection, rgb::RGB, vec3::Vec3};

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
    pub radius: f32,
}

impl Entity {
    pub fn intersection(self, origin: Vec3, direction: Vec3) -> Option<Intersection> {
        match self.shape {
            Shape::Sphere => self.sphere_intersection(origin, direction),
            Shape::Cube => todo!("Cube intersection"),
        }
    }

    fn sphere_intersection(self, origin: Vec3, direction: Vec3) -> Option<Intersection> {
        let sphere_ray = self.position - origin;
        let dist_sphere_ray = sphere_ray.mag();
        let dist_to_closest_point_on_ray = sphere_ray.dot(direction);
        let dist_from_closest_point_to_sphere =
            (dist_sphere_ray.powi(2) - dist_to_closest_point_on_ray.powi(2)).sqrt();

        let dist_to_intersection = dist_to_closest_point_on_ray
            - (self.radius.powi(2) - dist_from_closest_point_to_sphere.powi(2))
                .abs()
                .sqrt();
        let point = origin + (direction * dist_to_intersection);
        let roughness = Vec3::rng() * self.roughness;
        let normal = (point - self.position).normalize() + roughness;

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
        radius: f32,
    ) -> Self {
        Self {
            shape,
            position,
            emission,
            reflectivity,
            roughness,
            radius,
        }
    }
}
