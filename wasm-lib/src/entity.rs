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
        let origin_to_center = ray.origin - self.position;
        let a = ray.direction.mag_squared();
        let half_b = origin_to_center.dot(ray.direction);
        let c = origin_to_center.mag_squared() - radius * radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut t = (-half_b - sqrt_d) / a;

        if t < 0.001 {
            t = (-half_b + sqrt_d) / a;
        }

        if t < 0.001 {
            return None;
        }

        let point = ray.origin + (ray.direction * t);
        let normal = (point - self.position).normalize();

        Some(Intersection {
            dist: t,
            point,
            normal,
            entity: Some(self),
        })
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
    #[allow(clippy::too_many_arguments)]
    pub fn new_sphere(
        position: Vec3,
        emission: Rgb,
        albedo: Rgb,
        metallic: f32,
        roughness: f32,
        transmission: f32,
        ior: f32,
        radius: f32,
    ) -> Self {
        Self {
            position,
            shape: Shape::Sphere { radius },
            material: Material {
                emission,
                albedo,
                metallic,
                roughness,
                transmission,
                ior,
            },
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_plane(
        position: Vec3,
        normal: Vec3,
        emission: Rgb,
        albedo: Rgb,
        metallic: f32,
        roughness: f32,
        transmission: f32,
        ior: f32,
    ) -> Self {
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
                transmission,
                ior,
            },
        }
    }
}
