use crate::{ray::Ray, vec3::Vec3};

pub trait Traceable {
    fn bounds(&self, position: Vec3) -> Result<(Vec3, Vec3), &'static str>;
    fn intersect(&self, ray: Ray, position: Vec3) -> Option<(f32, Vec3)>;
}
