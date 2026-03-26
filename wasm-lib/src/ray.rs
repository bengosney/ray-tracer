use crate::vec3::Vec3;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(self, distance: f32) -> Vec3 {
        self.origin + (self.direction * distance)
    }
}
