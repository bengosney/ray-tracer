use rand::Rng;
use wasm_bindgen::prelude::*;

use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

use crate::rgb::RGB;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.x, self.y, self.z)
    }
}

#[wasm_bindgen]
impl Vec3 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Vec3 {
        Vec3 {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    pub fn normalize(self) -> Self {
        self * (1.0 / self.mag())
    }

    pub fn reflect(&self, normal: Self) -> Self {
        *self - (normal * self.dot(normal))
    }
}

impl Vec3 {
    pub fn avg(vectors: &Vec<Vec3>) -> Self {
        vectors.iter().fold(Vec3::zero(), |sum, &val| sum + val) * (1.0 / vectors.len() as f32)
    }

    pub fn rng() -> Self {
        let mut rng = rand::thread_rng();

        Vec3::new(
            rng.gen_range(-0.5..0.5),
            rng.gen_range(-0.5..0.5),
            rng.gen_range(-0.5..0.5),
        )
    }

    pub fn rng_normal() -> Self {
        let mut rng = rand::thread_rng();

        loop {
            let vec = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            );
            if vec.mag() < 1.0 {
                return vec;
            }
        }
    }
}

impl From<RGB> for Vec3 {
    fn from(colour: RGB) -> Self {
        Vec3 {
            x: colour.r,
            y: colour.g,
            z: colour.b,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(self, distance: f32) -> Vec3 {
        self.origin + (self.direction * distance)
    }

    pub fn scatter(self, by: f32) -> Self {
        let scatter = Vec3::rng() * by;
        Self {
            origin: self.origin,
            direction: Vec3 {
                x: self.direction.x + scatter.x,
                y: self.direction.y + scatter.y,
                z: self.direction.z,
            },
        }
    }

    pub fn defuse_scatter(self) -> Self {
        Self {
            origin: self.origin,
            direction: self.direction + Vec3::rng_normal(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vec3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(a + b, Vec3::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn test_add_f32() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;

        assert_eq!(a + b, Vec3::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn test_sub_vec3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(a - b, Vec3::new(-1.0, 0.0, 1.0));
    }

    #[test]
    fn test_sub_f32() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;

        assert_eq!(a - b, Vec3::new(-1.0, 0.0, 1.0));
    }

    #[test]
    fn test_mul_vec3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(a * b, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_f32() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;

        assert_eq!(a * b, Vec3::new(2.0, 4.0, 6.0));
    }
    #[test]
    fn test_div_vec3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(a / b, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_div_f32() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;

        assert_eq!(a / b, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_mag() {
        let a = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(a.mag(), 3.7416573867739413);
    }

    #[test]
    fn test_dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(a.dot(b), 12.0);
    }

    #[test]
    fn test_cross() {
        let a = Vec3::new(2.0, 3.0, 4.0);
        let b = Vec3::new(5.0, 6.0, 7.0);

        assert_eq!(a.cross(b), Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_normalize() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let normalized = a.normalize();

        assert_eq!(normalized.mag().round(), 1.0);
        assert_eq!(normalized, Vec3::new(0.26726124, 0.5345225, 0.8017837));
    }

    #[test]
    fn test_reflect() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(a.reflect(b), Vec3::new(-23.0, -22.0, -21.0));
    }

    #[test]
    fn test_avg() {
        let vecs = vec![
            Vec3::new(2.0, 2.0, 2.0),
            Vec3::new(2.0, 2.0, 2.0),
            Vec3::new(2.0, 2.0, 2.0),
        ];

        assert_eq!(Vec3::avg(&vecs), Vec3::new(2.0, 2.0, 2.0));
    }
}
