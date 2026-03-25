use wasm_bindgen::prelude::*;

use crate::{
    intersection::Intersection,
    rgb::Rgb,
    vec3::{Ray, Vec3},
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
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
    pub emission: Rgb,
    pub albedo: Rgb,
    pub metallic: f32,
    pub roughness: f32,
    pub transparency: f32,
    pub ior: f32,
}

impl Entity {
    pub fn intersection(self, ray: Ray) -> Option<Intersection> {
        let oc = ray.origin - self.position;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < 0.001 {
            root = (-half_b + sqrtd) / a;
            if root < 0.001 {
                return None;
            }
        }

        let point = ray.origin + ray.direction * root;
        let outward_normal = (point - self.position) * (1.0 / self.radius);
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            outward_normal * -1.0
        };

        Some(Intersection {
            dist: root,
            point,
            normal,
            entity: Some(self),
            front_face,
        })
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
        emission: Rgb,
        albedo: Rgb,
        metallic: f32,
        roughness: f32,
        radius: f32,
        transparency: f32,
        ior: f32,
    ) -> Self {
        Self {
            position,
            radius,
            material: Material {
                emission,
                albedo,
                metallic,
                roughness,
                transparency,
                ior,
            },
        }
    }
}
