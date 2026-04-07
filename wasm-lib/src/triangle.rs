use crate::ray::Ray;
use crate::traceable::Traceable;
use crate::vec3::Vec3;

#[derive(Copy, Clone, PartialEq)]
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self { a, b, c }
    }
}

impl Traceable for Triangle {
    fn bounds(&self, position: Vec3, rotation: Vec3) -> Result<(Vec3, Vec3), &'static str> {
        let pa = self.a.rotate_vec(rotation) + position;
        let pb = self.b.rotate_vec(rotation) + position;
        let pc = self.c.rotate_vec(rotation) + position;

        let min = Vec3::new(
            pa.x.min(pb.x).min(pc.x),
            pa.y.min(pb.y).min(pc.y),
            pa.z.min(pb.z).min(pc.z),
        );
        let max = Vec3::new(
            pa.x.max(pb.x).max(pc.x),
            pa.y.max(pb.y).max(pc.y),
            pa.z.max(pb.z).max(pc.z),
        );

        Ok((min, max))
    }

    fn intersect(&self, ray: Ray, position: Vec3, rotation: Vec3) -> Option<(f32, Vec3)> {
        let a = self.a.rotate_vec(rotation) + position;
        let b = self.b.rotate_vec(rotation) + position;
        let c = self.c.rotate_vec(rotation) + position;

        let edge1 = b - a;
        let edge2 = c - a;

        let h = ray.direction.cross(edge2);
        let det = edge1.dot(h);

        if det.abs() < f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.origin - a;
        let u = inv_det * s.dot(h);

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = s.cross(edge1);
        let v = inv_det * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = inv_det * edge2.dot(q);

        if t < f32::EPSILON {
            return None;
        }

        let mut normal = edge1.cross(edge2).normalize();
        if normal.dot(ray.direction) > 0.0 {
            normal = normal * -1.0;
        }

        Some((t, normal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat_triangle() -> Triangle {
        Triangle::new(
            Vec3::new(-1.0, -1.0, 5.0),
            Vec3::new(1.0, -1.0, 5.0),
            Vec3::new(0.0, 1.0, 5.0),
        )
    }

    #[test]
    fn test_intersection() {
        let t = flat_triangle();
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        let (dist, normal) = t.intersect(ray, Vec3::zero(), Vec3::zero()).unwrap();
        assert_eq!(dist, 5.0);
        assert_eq!(normal, Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_no_intersection_miss() {
        let t = flat_triangle();
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(5.0, 0.0, 1.0).normalize(),
        };
        assert!(t.intersect(ray, Vec3::zero(), Vec3::zero()).is_none());
    }

    #[test]
    fn test_no_intersection_parallel() {
        let t = flat_triangle();
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(1.0, 0.0, 0.0),
        };
        assert!(t.intersect(ray, Vec3::zero(), Vec3::zero()).is_none());
    }

    #[test]
    fn test_normal_flips_for_back_face() {
        let t = flat_triangle();
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 10.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
        };
        let (_, normal) = t.intersect(ray, Vec3::zero(), Vec3::zero()).unwrap();
        assert_eq!(normal, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_bounds() {
        let t = flat_triangle();
        let (min, max) = t.bounds(Vec3::zero(), Vec3::zero()).unwrap();
        assert_eq!(min, Vec3::new(-1.0, -1.0, 5.0));
        assert_eq!(max, Vec3::new(1.0, 1.0, 5.0));
    }
}
