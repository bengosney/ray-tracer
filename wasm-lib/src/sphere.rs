use crate::ray::Ray;
use crate::traceable::Traceable;
use crate::vec3::Vec3;

#[derive(Copy, Clone, PartialEq)]
pub struct Sphere {
    pub radius: f32,
    position: Vec3,
}

impl Sphere {
    pub fn new(radius: f32, position: Vec3) -> Self {
        Self { radius, position }
    }
}

impl Traceable for Sphere {
    fn bounds(&self) -> Result<(Vec3, Vec3), &'static str> {
        Ok((self.position - self.radius, self.position + self.radius))
    }

    fn intersect(&self, ray: Ray) -> Option<(f32, Vec3)> {
        let origin_to_center = ray.origin - self.position;
        let a = ray.direction.mag_squared();
        let half_b = origin_to_center.dot(ray.direction);
        let c = origin_to_center.mag_squared() - (self.radius * self.radius);

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

        Some((t, normal))
    }

    fn position(&self) -> Vec3 {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_intersection() {
        let sphere = Sphere::new(2.0, Vec3::new(0.0, 0.0, 10.0));
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };

        let (dist, normal) = sphere.intersect(ray).unwrap();
        assert_eq!(dist, 8.0);
        assert_eq!(normal, Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_sphere_no_intersection() {
        let sphere = Sphere::new(2.0, Vec3::new(0.0, 10.0, 0.0));
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };

        assert!(sphere.intersect(ray).is_none());
    }
}
