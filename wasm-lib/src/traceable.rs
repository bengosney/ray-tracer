use crate::{ray::Ray, vec3::Vec3};

pub trait Traceable {
    fn bounds(&self, position: Vec3) -> Result<(Vec3, Vec3), &'static str>;
    fn intersect(&self, position: Vec3, ray: Ray) -> Option<(f32, Vec3)>;
}
