use crate::{ray::Ray, vec3::Vec3};

pub trait Traceable {
    fn bounds(&self) -> Result<(Vec3, Vec3), &'static str>;
    fn intersect(&self, ray: Ray) -> Option<(f32, Vec3)>;
    fn position(&self) -> Vec3;
}
