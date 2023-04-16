use crate::{entity::Entity, vec3::Vec3};

pub struct Intersection {
    pub dist: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub entity: Option<Entity>,
}

impl Intersection {
    pub fn empty() -> Self {
        Intersection {
            point: Vec3::zero(),
            dist: f32::INFINITY,
            normal: Vec3::zero(),
            entity: None,
        }
    }

    pub fn closest(a: Self, b: Self) -> Self {
        match a.dist < b.dist {
            true => a,
            false => b,
        }
    }
}
