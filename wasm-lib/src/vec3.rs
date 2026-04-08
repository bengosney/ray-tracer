use rand::Rng;
use std::ops::AddAssign;
use wasm_bindgen::prelude::*;

use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

use crate::rgb::Rgb;

#[cfg(target_arch = "wasm32")]
use core::arch::wasm32::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct Vec3 {
    data: [f32; 4],
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.data[0] == other.data[0] && self.data[1] == other.data[1] && self.data[2] == other.data[2]
    }
}

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vec3")
            .field("x", &self.data[0])
            .field("y", &self.data[1])
            .field("z", &self.data[2])
            .finish()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.data[0], self.data[1], self.data[2])
    }
}

#[cfg(target_arch = "wasm32")]
impl Vec3 {
    #[inline(always)]
    fn load(&self) -> v128 {
        unsafe { v128_load(self.data.as_ptr() as *const v128) }
    }

    #[inline(always)]
    fn from_v128(v: v128) -> Self {
        let mut out = Vec3 { data: [0.0; 4] };
        unsafe { v128_store(out.data.as_mut_ptr() as *mut v128, v) };
        out
    }
}

#[wasm_bindgen]
impl Vec3 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { data: [x, y, z, 0.0] }
    }

    pub fn zero() -> Self {
        Vec3 { data: [0.0; 4] }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f32 {
        self.data[0]
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f32 {
        self.data[1]
    }

    #[wasm_bindgen(getter)]
    pub fn z(&self) -> f32 {
        self.data[2]
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, val: f32) {
        self.data[0] = val;
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, val: f32) {
        self.data[1] = val;
    }

    #[wasm_bindgen(setter)]
    pub fn set_z(&mut self, val: f32) {
        self.data[2] = val;
    }

    pub fn mag(&self) -> f32 {
        self.mag_squared().sqrt()
    }

    pub fn mag_squared(&self) -> f32 {
        self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        self.data[0] * rhs.data[0] + self.data[1] * rhs.data[1] + self.data[2] * rhs.data[2]
    }

    pub fn cross(&self, rhs: Self) -> Vec3 {
        Vec3::new(
            self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
            self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2],
            self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0],
        )
    }

    pub fn normalize(self) -> Self {
        self * (1.0 / self.mag())
    }

    pub fn reflect(&self, normal: Self) -> Self {
        *self - (normal * 2.0 * self.dot(normal))
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Option<Vec3> {
        let cos_theta = (uv * -1.0).dot(n).min(1.0);
        let sin2_theta = 1.0 - cos_theta * cos_theta;
        let sin2_theta_t = etai_over_etat * etai_over_etat * sin2_theta;
        if sin2_theta_t > 1.0 {
            return None;
        }
        let cos_theta_t = (1.0 - sin2_theta_t).sqrt();
        Some(uv * etai_over_etat + n * (etai_over_etat * cos_theta - cos_theta_t))
    }

    pub fn reflectance(cos_theta: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

impl Vec3 {
    pub fn avg(vectors: &[Vec3]) -> Self {
        vectors.iter().fold(Vec3::zero(), |sum, &val| sum + val) * (1.0 / vectors.len() as f32)
    }

    pub fn gamma(self, gamma: f32) -> Self {
        let gamma_correction = 1.0 / gamma;
        Vec3::new(
            255.0 * (self.data[0] / 255.0).clamp(0.0, 1.0).powf(gamma_correction),
            255.0 * (self.data[1] / 255.0).clamp(0.0, 1.0).powf(gamma_correction),
            255.0 * (self.data[2] / 255.0).clamp(0.0, 1.0).powf(gamma_correction),
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

    pub fn min(&self, other: Vec3) -> Vec3 {
        #[cfg(target_arch = "wasm32")]
        {
            Self::from_v128(f32x4_min(self.load(), other.load()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Vec3::new(
                self.data[0].min(other.data[0]),
                self.data[1].min(other.data[1]),
                self.data[2].min(other.data[2]),
            )
        }
    }

    pub fn max(&self, other: Vec3) -> Vec3 {
        #[cfg(target_arch = "wasm32")]
        {
            Self::from_v128(f32x4_max(self.load(), other.load()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Vec3::new(
                self.data[0].max(other.data[0]),
                self.data[1].max(other.data[1]),
                self.data[2].max(other.data[2]),
            )
        }
    }
}

impl From<Rgb> for Vec3 {
    fn from(colour: Rgb) -> Self {
        Vec3::new(colour.r, colour.g, colour.b)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        #[cfg(target_arch = "wasm32")]
        {
            Self::from_v128(f32x4_add(self.load(), rhs.load()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Vec3 {
                data: [
                    self.data[0] + rhs.data[0],
                    self.data[1] + rhs.data[1],
                    self.data[2] + rhs.data[2],
                    0.0,
                ],
            }
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: f32) -> Self::Output {
        #[cfg(target_arch = "wasm32")]
        {
            Self::from_v128(f32x4_add(self.load(), f32x4_splat(rhs)))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Vec3 {
                data: [self.data[0] + rhs, self.data[1] + rhs, self.data[2] + rhs, 0.0],
            }
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        #[cfg(target_arch = "wasm32")]
        {
            Self::from_v128(f32x4_sub(self.load(), rhs.load()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Vec3 {
                data: [
                    self.data[0] - rhs.data[0],
                    self.data[1] - rhs.data[1],
                    self.data[2] - rhs.data[2],
                    0.0,
                ],
            }
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: f32) -> Self::Output {
        #[cfg(target_arch = "wasm32")]
        {
            Self::from_v128(f32x4_sub(self.load(), f32x4_splat(rhs)))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Vec3 {
                data: [self.data[0] - rhs, self.data[1] - rhs, self.data[2] - rhs, 0.0],
            }
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        #[cfg(target_arch = "wasm32")]
        {
            Self::from_v128(f32x4_mul(self.load(), rhs.load()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Vec3 {
                data: [
                    self.data[0] * rhs.data[0],
                    self.data[1] * rhs.data[1],
                    self.data[2] * rhs.data[2],
                    0.0,
                ],
            }
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        #[cfg(target_arch = "wasm32")]
        {
            Self::from_v128(f32x4_mul(self.load(), f32x4_splat(rhs)))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Vec3 {
                data: [self.data[0] * rhs, self.data[1] * rhs, self.data[2] * rhs, 0.0],
            }
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Vec3) -> Self::Output {
        #[cfg(target_arch = "wasm32")]
        {
            Self::from_v128(f32x4_div(self.load(), rhs.load()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Vec3 {
                data: [
                    self.data[0] / rhs.data[0],
                    self.data[1] / rhs.data[1],
                    self.data[2] / rhs.data[2],
                    0.0,
                ],
            }
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        #[cfg(target_arch = "wasm32")]
        {
            Self::from_v128(f32x4_div(self.load(), f32x4_splat(rhs)))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Vec3 {
                data: [self.data[0] / rhs, self.data[1] / rhs, self.data[2] / rhs, 0.0],
            }
        }
    }
}

impl Mul<u32> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: u32) -> Self::Output {
        self * (rhs as f32)
    }
}

impl Div<u32> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: u32) -> Self::Output {
        self / (rhs as f32)
    }
}

impl AddAssign for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
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
        let a = Vec3::new(128.0, 64.0, 200.0);
        let result = a.gamma(1.0);

        assert!((result.x() - 128.0).abs() < 1e-3);
        assert!((result.y() - 64.0).abs() < 1e-3);
        assert!((result.z() - 200.0).abs() < 1e-3);
    }

    #[test]
    fn test_gamma_clamps_to_range() {
        let a = Vec3::new(300.0, -50.0, 128.0);
        let result = a.gamma(2.2);

        assert!(result.x() >= 0.0 && result.x() <= 255.0);
        assert!(result.y() >= 0.0 && result.y() <= 255.0);
        assert!(result.z() >= 0.0 && result.z() <= 255.0);
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
        let f0 = Vec3::new(0.04, 0.04, 0.04);
        let result = Vec3::fresnel_schlick(f0, 1.0);

        assert!((result.x() - 0.04).abs() < 1e-5);
        assert!((result.y() - 0.04).abs() < 1e-5);
        assert!((result.z() - 0.04).abs() < 1e-5);
    }

    #[test]
    fn test_fresnel_schlick_at_grazing() {
        let f0 = Vec3::new(0.04, 0.04, 0.04);
        let result = Vec3::fresnel_schlick(f0, 0.0);

        assert!((result.x() - 1.0).abs() < 1e-5);
        assert!((result.y() - 1.0).abs() < 1e-5);
        assert!((result.z() - 1.0).abs() < 1e-5);
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
            assert!(v.x() >= -0.5 && v.x() <= 0.5);
            assert!(v.y() >= -0.5 && v.y() <= 0.5);
            assert!(v.z() >= -0.5 && v.z() <= 0.5);
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
