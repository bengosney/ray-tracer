use wasm_bindgen::prelude::wasm_bindgen;

use crate::vec3::Vec3;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub struct Matrix<T> {
    shape: usize,
    data: Vec<T>,
    half_range: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: Vec<T>) -> Self {
        let shape = (data.len() as f32).sqrt().ceil() as usize;
        Self {
            shape: shape,
            data: data,
            half_range: shape / 2,
        }
    }

    pub fn range(&self) -> std::ops::RangeInclusive<isize> {
        (0-(self.half_range as isize))..=self.half_range as isize
    }

    pub fn vec_range(&self, len: usize) -> std::ops::RangeInclusive<usize> {
        self.half_range..=(len - self.half_range)
    }
}

impl<T: Copy> Matrix<T> where f32: From<T> {
    pub fn apply(self, pixels: Vec<Vec<Vec3>>) -> Vec<Vec<Vec3>> {
        let mut processed: Vec<Vec<Vec3>> = pixels.clone();

        log(&format!("range: {:?}", self.range()));
        log(&format!("shape: {:?}", self.shape));
        log("~~~~");
        for d in &self.data {
            log(&format!("{:?}", f32::from(d.clone())));
        }
        log("~~~~");
        log("ok 9");
        for y in self.vec_range(pixels.len()) {
            for x in self.vec_range(pixels[y].len()) {
                processed[y][x] = Vec3::new(55.0, 0.0, 0.0);
                for u in self.range() {
                    for v in self.range() {
                        let i = (y as isize + u) as usize;
                        let j = (x as isize + v) as usize;
                        log("-===-");
                        log(&format!("x: {}, y: {}", x, y));
                        log(&format!("u: {}, v: {}", u, v));
                        log(&format!("i: {}, j: {}", i, j));

                        if y == 10 && x == 10 {
                            //log(&format!("u: {}, v: {}", u, v));
                            //log(&format!("v: {}", (pixels[i][j] * f32::from(self.data[(u * v) as usize]))));
                        }

                        processed[y][x] = processed[y][x] + (pixels[i][j] * f32::from(self.data[(u + (v * self.shape as isize)) as usize]));
                    }
                }

            }
        }

        return processed;
    }
}
