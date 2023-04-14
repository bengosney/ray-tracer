use wasm_bindgen::prelude::*;

use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[wasm_bindgen()]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Vec2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
    }

    pub fn mag(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn normalize(self) -> Self {
        self * (1.0 / self.mag())
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - (normal * self.dot(normal))
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<f32> for Vec2 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<f32> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vec2() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(2.0, 2.0);

        assert_eq!(a + b, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_add_f32() {
        let a = Vec2::new(1.0, 2.0);
        let b = 2.0;

        assert_eq!(a + b, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_sub_vec2() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(2.0, 2.0);

        assert_eq!(a - b, Vec2::new(-1.0, 0.0));
    }

    #[test]
    fn test_sub_f32() {
        let a = Vec2::new(1.0, 3.0);
        let b = 2.0;

        assert_eq!(a - b, Vec2::new(-1.0, 1.0));
    }

    #[test]
    fn test_mul_vec2() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(2.0, 2.0);

        assert_eq!(a * b, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn test_mul_f32() {
        let a = Vec2::new(1.0, 2.0);
        let b = 2.0;

        assert_eq!(a * b, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn test_mag() {
        let a = Vec2::new(1.0, 2.0);

        assert_eq!(a.mag(), 2.236068);
    }

    #[test]
    fn test_dot() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(2.0, 2.0);

        assert_eq!(a.dot(b), 6.0);
    }

    #[test]
    fn test_normalize() {
        let a = Vec2::new(1.0, 2.0);
        let normalized = a.normalize();

        assert_eq!(normalized.mag().round(), 1.0);
        assert_eq!(normalized, Vec2::new(0.4472136, 0.8944272));
    }

    #[test]
    fn test_reflect() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(2.0, 2.0);

        assert_eq!(a.reflect(b), Vec2::new(-11.0, -10.0));
    }
}
