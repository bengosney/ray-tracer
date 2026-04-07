use crate::ray::Ray;
use crate::traceable::Traceable;
use crate::vec3::Vec3;

#[derive(Copy, Clone, PartialEq)]
pub struct Plane {
    pub normal: Vec3,
}

impl Plane {
    pub fn new(normal: Vec3) -> Self {
        Self {
            normal: normal.normalize(),
        }
    }
}

impl Traceable for Plane {
    fn bounds(&self, _position: Vec3, _rotation: Vec3) -> Result<(Vec3, Vec3), &'static str> {
        Err("planes are infinate on two axis")
    }

    fn intersect(&self, ray: Ray, position: Vec3, _rotation: Vec3) -> Option<(f32, Vec3)> {
        let denom = ray.direction.dot(self.normal);
        if denom.abs() < 0.0001 {
            return None;
        }

        let t = (position - ray.origin).dot(self.normal) / denom;
        if t < 0.001 {
            return None;
        }

        let normal = if denom > 0.0 { self.normal * -1.0 } else { self.normal };

        Some((t, normal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane_intersection() {
        let plane = Plane::new(Vec3::new(0.0, 1.0, 0.0));
        let position = Vec3::new(0.0, -2.0, 0.0);
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(0.0, -1.0, 0.0),
        };

        let (dist, normal) = plane.intersect(ray, position, Vec3::zero()).unwrap();
        assert_eq!(dist, 2.0);
        assert_eq!(normal, Vec3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_plane_no_intersection() {
        let plane = Plane::new(Vec3::new(0.0, 1.0, 0.0));
        let position = Vec3::new(0.0, -2.0, 0.0);
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(1.0, 0.0, 0.0),
        };

        assert!(plane.intersect(ray, position, Vec3::zero()).is_none());
    }
}
