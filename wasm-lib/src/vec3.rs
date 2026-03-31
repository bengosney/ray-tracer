use rand::Rng;
use std::ops::AddAssign;
use wasm_bindgen::prelude::*;

use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

use crate::rgb::Rgb;

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
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn mag(&self) -> f32 {
        self.mag_squared().sqrt()
    }

    pub fn mag_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
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
        *self - (normal * 2.0 * self.dot(normal))
    }
}

impl Vec3 {
    pub fn avg(vectors: &[Vec3]) -> Self {
        vectors.iter().fold(Vec3::zero(), |sum, &val| sum + val) * (1.0 / vectors.len() as f32)
    }

    pub fn gamma(self, gamma: f32) -> Self {
        let gamma_correction = 1.0 / gamma;
        Vec3::new(
            255.0 * (self.x / 255.0).clamp(0.0, 1.0).powf(gamma_correction),
            255.0 * (self.y / 255.0).clamp(0.0, 1.0).powf(gamma_correction),
            255.0 * (self.z / 255.0).clamp(0.0, 1.0).powf(gamma_correction),
        )
    }

    pub fn rng(rng: &mut impl Rng) -> Self {
        Vec3::new(
            rng.gen_range(-0.5..0.5),
            rng.gen_range(-0.5..0.5),
            rng.gen_range(-0.5..0.5),
        )
    }

    pub fn rng_normal(rng: &mut impl Rng) -> Self {
        loop {
            let vec = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            );
            if vec.mag_squared() < 1.0 {
                return vec;
            }
        }
    }

    pub fn rng_hemisphere(normal: Vec3, rng: &mut impl Rng) -> Self {
        let v = Self::rng_normal(rng).normalize();
        if v.dot(normal) > 0.0 {
            v
        } else {
            v * -1.0
        }
    }

    pub fn fresnel_schlick(f0: Vec3, cos_theta: f32) -> Vec3 {
        let c = (1.0 - cos_theta.clamp(0.0, 1.0)).powi(5);
        f0 + (Vec3::new(1.0, 1.0, 1.0) - f0) * c
    }

    pub fn lerp(a: Vec3, b: Vec3, t: f32) -> Vec3 {
        a * (1.0 - t) + b * t
    }
}

impl From<Rgb> for Vec3 {
    fn from(colour: Rgb) -> Self {
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

impl Mul<u32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Vec3 {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
        }
    }
}

impl Div<u32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        Vec3 {
            x: self.x / rhs as f32,
            y: self.y / rhs as f32,
            z: self.z / rhs as f32,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
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

        assert_eq!(a.reflect(b), Vec3::new(-47.0, -46.0, -45.0));
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

    #[test]
    fn test_avg_different_values() {
        let vecs = vec![Vec3::new(0.0, 10.0, 20.0), Vec3::new(10.0, 20.0, 40.0)];

        assert_eq!(Vec3::avg(&vecs), Vec3::new(5.0, 15.0, 30.0));
    }

    #[test]
    fn test_mag_squared() {
        let a = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(a.mag_squared(), 14.0);
    }

    #[test]
    fn test_mag_squared_equals_mag_times_mag() {
        let a = Vec3::new(3.0, 4.0, 5.0);

        let diff = (a.mag_squared() - a.mag() * a.mag()).abs();
        assert!(diff < 1e-5, "mag_squared should equal mag*mag, diff={}", diff);
    }

    #[test]
    fn test_zero() {
        let z = Vec3::zero();

        assert_eq!(z, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_div_u32() {
        let a = Vec3::new(10.0, 20.0, 30.0);

        assert_eq!(a / 2u32, Vec3::new(5.0, 10.0, 15.0));
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        a += Vec3::new(10.0, 20.0, 30.0);

        assert_eq!(a, Vec3::new(11.0, 22.0, 33.0));
    }

    #[test]
    fn test_add_assign_accumulates() {
        let mut a = Vec3::zero();
        a += Vec3::new(1.0, 2.0, 3.0);
        a += Vec3::new(1.0, 2.0, 3.0);
        a += Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(a, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_gamma_identity() {
        // gamma of 1.0 should return the same values
        let a = Vec3::new(128.0, 64.0, 200.0);
        let result = a.gamma(1.0);

        assert!((result.x - 128.0).abs() < 1e-3);
        assert!((result.y - 64.0).abs() < 1e-3);
        assert!((result.z - 200.0).abs() < 1e-3);
    }

    #[test]
    fn test_gamma_clamps_to_range() {
        let a = Vec3::new(300.0, -50.0, 128.0);
        let result = a.gamma(2.2);

        assert!(result.x >= 0.0 && result.x <= 255.0);
        assert!(result.y >= 0.0 && result.y <= 255.0);
        assert!(result.z >= 0.0 && result.z <= 255.0);
    }

    #[test]
    fn test_lerp_at_zero() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(10.0, 20.0, 30.0);

        assert_eq!(Vec3::lerp(a, b, 0.0), a);
    }

    #[test]
    fn test_lerp_at_one() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(10.0, 20.0, 30.0);

        assert_eq!(Vec3::lerp(a, b, 1.0), b);
    }

    #[test]
    fn test_lerp_midpoint() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(10.0, 20.0, 30.0);

        assert_eq!(Vec3::lerp(a, b, 0.5), Vec3::new(5.0, 10.0, 15.0));
    }

    #[test]
    fn test_fresnel_schlick_at_zero() {
        // At cos_theta=1 (head-on), fresnel should return f0
        let f0 = Vec3::new(0.04, 0.04, 0.04);
        let result = Vec3::fresnel_schlick(f0, 1.0);

        assert!((result.x - 0.04).abs() < 1e-5);
        assert!((result.y - 0.04).abs() < 1e-5);
        assert!((result.z - 0.04).abs() < 1e-5);
    }

    #[test]
    fn test_fresnel_schlick_at_grazing() {
        // At cos_theta=0 (grazing angle), fresnel should approach 1.0
        let f0 = Vec3::new(0.04, 0.04, 0.04);
        let result = Vec3::fresnel_schlick(f0, 0.0);

        assert!((result.x - 1.0).abs() < 1e-5);
        assert!((result.y - 1.0).abs() < 1e-5);
        assert!((result.z - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_from_rgb() {
        let rgb = Rgb::new(0.5, 0.6, 0.7);
        let v: Vec3 = Vec3::from(rgb);

        assert_eq!(v, Vec3::new(0.5, 0.6, 0.7));
    }

    #[test]
    fn test_rng_in_range() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let v = Vec3::rng(&mut rng);
            assert!(v.x >= -0.5 && v.x <= 0.5);
            assert!(v.y >= -0.5 && v.y <= 0.5);
            assert!(v.z >= -0.5 && v.z <= 0.5);
        }
    }

    #[test]
    fn test_rng_normal_in_unit_sphere() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let v = Vec3::rng_normal(&mut rng);
            assert!(
                v.mag() < 1.0,
                "rng_normal should be inside unit sphere, got mag={}",
                v.mag()
            );
        }
    }

    #[test]
    fn test_rng_hemisphere_same_side_as_normal() {
        let mut rng = rand::thread_rng();
        let normal = Vec3::new(0.0, 1.0, 0.0);
        for _ in 0..100 {
            let v = Vec3::rng_hemisphere(normal, &mut rng);
            assert!(
                v.dot(normal) > 0.0,
                "hemisphere vector should be on same side as normal"
            );
        }
    }
}
