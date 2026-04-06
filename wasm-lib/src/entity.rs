use wasm_bindgen::prelude::*;

use crate::plane::Plane;
use crate::sphere::Sphere;
use crate::traceable::Traceable;
use crate::{intersection::Intersection, material::Material, ray::Ray, vec3::Vec3};

#[derive(Copy, Clone, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

impl Shape {
    pub fn position(&self) -> Vec3 {
        match self {
            Shape::Sphere(s) => s.position(),
            Shape::Plane(p) => p.position(),
        }
    }
}

#[wasm_bindgen()]
#[derive(Copy, Clone, PartialEq)]
pub struct Entity {
    shape: Shape,
    material: Material,
}

impl Entity {
    pub fn bounds(self) -> Result<(Vec3, Vec3), &'static str> {
        match self.shape {
            Shape::Sphere(s) => s.bounds(),
            Shape::Plane(p) => p.bounds(),
        }
    }

    pub fn intersection(self, ray: Ray) -> Option<Intersection> {
        let (t, normal) = match self.shape {
            Shape::Sphere(s) => s.intersect(ray)?,
            Shape::Plane(p) => p.intersect(ray)?,
        };

        Some(Intersection {
            dist: t,
            point: ray.origin + (ray.direction * t),
            normal,
            entity: Some(self),
        })
    }

    pub fn material(self) -> Material {
        self.material
    }

    pub fn position(self) -> Vec3 {
        self.shape.position()
    }

    pub fn shape(self) -> Shape {
        self.shape
    }
}

#[wasm_bindgen]
impl Entity {
    pub fn new_sphere(position: Vec3, material: Material, radius: f32) -> Self {
        Self {
            shape: Shape::Sphere(Sphere::new(radius, position)),
            material,
        }
    }

    pub fn new_plane(position: Vec3, material: Material, normal: Vec3) -> Self {
        Self {
            shape: Shape::Plane(Plane::new(normal, position)),
            material,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rgb::Rgb;

    fn test_material() -> Material {
        Material::new(Rgb::new(0.0, 0.0, 0.0), Rgb::new(1.0, 1.0, 1.0), 0.0, 0.0, 0.0, 1.5)
    }

    #[test]
    fn test_entity_sphere_intersection() {
        let position = Vec3::new(0.0, 0.0, 10.0);
        let entity = Entity::new_sphere(position, test_material(), 2.0);
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };

        let intersection = entity.intersection(ray).unwrap();
        assert_eq!(intersection.dist, 8.0);
        assert!(intersection.entity.is_some());
        assert_eq!(intersection.entity.unwrap().position(), entity.position());
    }

    #[test]
    fn test_entity_plane_intersection() {
        let position = Vec3::new(0.0, -2.0, 0.0);
        let entity = Entity::new_plane(position, test_material(), Vec3::new(0.0, 1.0, 0.0));
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(0.0, -1.0, 0.0),
        };

        let intersection = entity.intersection(ray).unwrap();
        assert_eq!(intersection.dist, 2.0);
        assert!(intersection.entity.is_some());
        assert_eq!(intersection.entity.unwrap().position(), entity.position());
    }

    #[test]
    fn test_entity_accessors() {
        let position = Vec3::new(1.0, 2.0, 3.0);
        let material = test_material();
        let entity = Entity::new_sphere(position, material, 1.0);

        assert_eq!(entity.position(), position);
        let m = entity.material();
        assert_eq!(m.ior, material.ior);
        assert_eq!(m.metallic, material.metallic);

        match entity.shape() {
            Shape::Sphere(s) => assert_eq!(s.radius, 1.0),
            _ => panic!("Expected sphere"),
        }
    }

    #[test]
    fn test_entity_bounds() {
        let entity = Entity::new_sphere(Vec3::zero(), test_material(), 1.0);
        let (min, max) = entity.bounds().unwrap();
        assert_eq!(min, Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(max, Vec3::new(1.0, 1.0, 1.0));
    }
}
