use wasm_bindgen::prelude::*;

use crate::{intersection::Intersection, material::Material, ray::Ray, rgb::Rgb, vec3::Vec3};

#[derive(Copy, Clone, PartialEq)]
pub enum Shape {
    Sphere { radius: f32 },
    Plane { normal: Vec3 },
}

#[wasm_bindgen()]
#[derive(Copy, Clone, PartialEq)]
pub struct Entity {
    position: Vec3,
    shape: Shape,
    material: Material,
}

impl Entity {
    pub fn intersection(self, ray: Ray) -> Option<Intersection> {
        match self.shape {
            Shape::Sphere { radius } => self.intersect_sphere(ray, radius),
            Shape::Plane { normal } => self.intersect_plane(ray, normal),
        }
    }

    fn intersect_sphere(self, ray: Ray, radius: f32) -> Option<Intersection> {
        let sphere_ray = self.position - ray.origin;
        let dist_sphere_ray_sq = sphere_ray.mag_squared();
        let dist_to_closest_point_on_ray = sphere_ray.dot(ray.direction);
        let dist_from_closest_sq = dist_sphere_ray_sq - dist_to_closest_point_on_ray.powi(2);
        let radius_sq = radius * radius;

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

    fn intersect_plane(self, ray: Ray, normal: Vec3) -> Option<Intersection> {
        let denom = ray.direction.dot(normal);
        if denom.abs() < 0.0001 {
            return None;
        }

        let t = (self.position - ray.origin).dot(normal) / denom;
        if t < 0.001 {
            return None;
        }

        let point = ray.origin + ray.direction * t;

        Some(Intersection {
            dist: t,
            point,
            normal: if denom > 0.0 { normal * -1.0 } else { normal },
            entity: Some(self),
        })
    }

    pub fn material(self) -> Material {
        self.material
    }
}

#[wasm_bindgen]
impl Entity {
    //#[wasm_bindgen(constructor)]
    pub fn new_sphere(position: Vec3, emission: Rgb, albedo: Rgb, metallic: f32, roughness: f32, radius: f32) -> Self {
        Self {
            position,
            shape: Shape::Sphere { radius },
            material: Material {
                emission,
                albedo,
                metallic,
                roughness,
            },
        }
    }

    pub fn new_plane(position: Vec3, normal: Vec3, emission: Rgb, albedo: Rgb, metallic: f32, roughness: f32) -> Self {
        Self {
            position,
            shape: Shape::Plane {
                normal: normal.normalize(),
            },
            material: Material {
                emission,
                albedo,
                metallic,
                roughness,
            },
        }
    }
}
