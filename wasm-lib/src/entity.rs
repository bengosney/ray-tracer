use wasm_bindgen::prelude::*;

use crate::{intersection::Intersection, material::Material, ray::Ray, vec3::Vec3};

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

    pub fn position(self) -> Vec3 {
        self.position
    }

    pub fn shape(self) -> Shape {
        self.shape
    }
}

#[wasm_bindgen]
impl Entity {
    //#[wasm_bindgen(constructor)]
    pub fn new_sphere(position: Vec3, material: Material, radius: f32) -> Self {
        Self {
            position,
            shape: Shape::Sphere { radius },
            material,
        }
    }

    pub fn new_plane(position: Vec3, material: Material, normal: Vec3) -> Self {
        Self {
            position,
            shape: Shape::Plane {
                normal: normal.normalize(),
            },
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
    fn test_sphere_intersection() {
        let sphere = Entity::new_sphere(Vec3::new(0.0, 0.0, 10.0), test_material(), 2.0);
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };

        let intersection = sphere.intersection(ray).unwrap();
        assert_eq!(intersection.dist, 8.0);
        assert_eq!(intersection.point, Vec3::new(0.0, 0.0, 8.0));
        assert_eq!(intersection.normal, Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_sphere_no_intersection() {
        let sphere = Entity::new_sphere(Vec3::new(0.0, 10.0, 0.0), test_material(), 2.0);
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };

        assert!(sphere.intersection(ray).is_none());
    }

    #[test]
    fn test_plane_intersection() {
        let plane = Entity::new_plane(Vec3::new(0.0, -2.0, 0.0), test_material(), Vec3::new(0.0, 1.0, 0.0));
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(0.0, -1.0, 0.0),
        };

        let intersection = plane.intersection(ray).unwrap();
        assert_eq!(intersection.dist, 2.0);
        assert_eq!(intersection.point, Vec3::new(0.0, -2.0, 0.0));
        assert_eq!(intersection.normal, Vec3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_plane_no_intersection() {
        let plane = Entity::new_plane(Vec3::new(0.0, -2.0, 0.0), test_material(), Vec3::new(0.0, 1.0, 0.0));
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(1.0, 0.0, 0.0),
        };

        assert!(plane.intersection(ray).is_none());
    }
}
