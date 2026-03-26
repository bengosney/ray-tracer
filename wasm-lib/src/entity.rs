use wasm_bindgen::prelude::*;

use crate::{intersection::Intersection, material::Material, ray::Ray, rgb::Rgb, vec3::Vec3};

#[wasm_bindgen()]
#[derive(Copy, Clone, PartialEq)]
pub struct Entity {
    position: Vec3,
    radius: f32,
    material: Material,
}

impl Entity {
    pub fn intersection(self, ray: Ray) -> Option<Intersection> {
        let sphere_ray = self.position - ray.origin;
        let dist_sphere_ray_sq = sphere_ray.mag_squared();
        let dist_to_closest_point_on_ray = sphere_ray.dot(ray.direction);
        let dist_from_closest_sq = dist_sphere_ray_sq - dist_to_closest_point_on_ray.powi(2);
        let radius_sq = self.radius * self.radius;

        if dist_to_closest_point_on_ray > 0.0 && dist_from_closest_sq < radius_sq {
            let dist_to_intersection = dist_to_closest_point_on_ray - (radius_sq - dist_from_closest_sq).abs().sqrt();

            let point = ray.origin + (ray.direction * dist_to_intersection);
            let normal = (point - self.position).normalize();

            return Some(Intersection {
                dist: dist_to_intersection,
                point,
                normal,
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
    pub fn new(position: Vec3, emission: Rgb, albedo: Rgb, metallic: f32, roughness: f32, radius: f32) -> Self {
        Self {
            position,
            radius,
            material: Material {
                emission,
                albedo,
                metallic,
                roughness,
            },
        }
    }
}
