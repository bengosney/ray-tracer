use crate::vec3::Vec3;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[derive(Clone, PartialEq)]
pub struct Kernel<T> {
    shape: usize,
    data: Vec<T>,
    half_range: usize,
    normalize: bool
}

#[wasm_bindgen]
pub struct ImageFilter {
    kernel: Kernel<i16>
}

#[wasm_bindgen]
impl ImageFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<i16>, normalize: bool) -> Self {
        Self { kernel: Kernel::new(data, normalize) }
    }
}

impl ImageFilter {
    pub fn get_kernel(&self) -> &Kernel<i16> {
        &self.kernel
    }
}

impl<T: Copy> Kernel<T> {
    pub fn new(data: Vec<T>, normalize: bool) -> Self {
        let shape = (data.len() as f32).sqrt().ceil() as usize;
        Self {
            shape: shape,
            data: data,
            half_range: shape / 2,
            normalize: normalize,
        }
    }

    pub fn range(&self) -> std::ops::RangeInclusive<isize> {
        (0-(self.half_range as isize))..=self.half_range as isize
    }

    pub fn vec_range(&self, len: usize) -> std::ops::Range<usize> {
        self.half_range..(len - self.half_range)
    }

    pub fn get_data(&self, u: isize, v: isize) -> T {
        let i = (u + self.half_range as isize) as usize;
        let j = (v + self.half_range as isize) as usize;

        self.data[i + (j * self.shape)]
    }
}

impl<T: Copy> Kernel<T> where f32: From<T> {
    pub fn sum(&self) -> f32 {
        self.data.iter().fold(0.0, |acc: f32, cur: &T| acc + f32::from(cur.clone()))
    }

    pub fn apply(&self, pixels: Vec<Vec<Vec3>>) -> Vec<Vec<Vec3>> {
        let mut processed: Vec<Vec<Vec3>> = pixels.clone();

        let sum = if self.normalize { self.sum() } else { 1.0 };

        for y in self.vec_range(pixels.len()) {
            for x in self.vec_range(pixels[y].len()) {
                processed[y][x] = Vec3::new(0.0, 0.0, 0.0);
                for u in self.range() {
                    for v in self.range() {
                        let i = (y as isize + u) as usize;
                        let j = (x as isize + v) as usize;

                        processed[y][x] = processed[y][x] + (pixels[i][j] * f32::from(self.get_data(u, v))) / sum;
                    }
                }

            }
        }

        return processed;
    }
}
