use js_sys::Date;
use rayon::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::OffscreenCanvasRenderingContext2d;

use rand::rngs::SmallRng;
use rand::SeedableRng;

use crate::bvh::Tree;
use crate::log;
use crate::post_processing::PostProcess;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::tracer;
use crate::vec3::Vec3;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "requestAnimationFrame")]
    fn request_animation_frame(closure: &Closure<dyn FnMut()>) -> i32;
}

fn random_in_unit_disc(rng: &mut impl rand::Rng) -> (f32, f32) {
    loop {
        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);
        if x * x + y * y < 1.0 {
            return (x, y);
        }
    }
}

fn avg_samples_into(samples: &[Vec<Vec3>], count: u32, out: &mut [Vec<Vec3>]) {
    out.par_iter_mut()
        .zip(samples.par_iter())
        .for_each(|(row_out, row_in)| {
            for (v_out, v_in) in row_out.iter_mut().zip(row_in.iter()) {
                *v_out = *v_in / count;
            }
        });
}

fn samples_to_pixel_map_into(samples: &[Vec<Vec3>], out: &mut Vec<u8>) {
    let height = samples.len();
    let width = samples[0].len();
    out.resize(height * width * 4, 0);

    out.par_chunks_mut(width * 4)
        .zip(samples.par_iter())
        .for_each(|(row_out, row_in)| {
            for (i, sample) in row_in.iter().enumerate() {
                row_out[i * 4] = sample.x as u8;
                row_out[i * 4 + 1] = sample.y as u8;
                row_out[i * 4 + 2] = sample.z as u8;
                row_out[i * 4 + 3] = 255;
            }
        });
}

pub fn render(scene: &Scene, ctx: &OffscreenCanvasRenderingContext2d, on_sample: js_sys::Function) {
    let half_width = (scene.width / 2) as i32;
    let half_height = (scene.height / 2) as i32;

    let width = scene.width;
    let height = scene.height;
    let camera = scene.camera();
    let focal_length = camera.focal_length;
    let focal_distance = camera.focal_distance as f32;
    let aperture = camera.aperture;
    let camera_origin = camera.position;
    let camera_rotation = camera.rotation;
    let bounces = scene.bounces;
    let bvh = Tree::build(scene.entities());
    let sample_count = scene.samples;
    let post_processors: Vec<Rc<dyn PostProcess>> = scene.post_processors().iter().map(Rc::clone).collect();

    let local_context = ctx.clone();
    let on_sample = on_sample.clone();

    let origin = camera_origin;
    let mut samples: Vec<Vec<Vec3>> = vec![vec![Vec3::new(0.0, 0.0, 0.0); width as usize]; height as usize];
    let mut avg_buf: Vec<Vec<Vec3>> = vec![vec![Vec3::new(0.0, 0.0, 0.0); width as usize]; height as usize];
    let mut pixel_buf: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut s = 0;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if s >= sample_count {
            let _ = f.borrow_mut().take();
            return;
        }
        let start = Date::now();

        samples.par_iter_mut().enumerate().for_each(|(j, row)| {
            let mut rng = SmallRng::seed_from_u64((s as u64) << 32 | (j as u64));
            for (i, sample) in row.iter_mut().enumerate() {
                use rand::Rng;
                let x = (i as i32 - half_width) as f32 + rng.gen_range(-0.5..0.5);
                let y = (j as i32 - half_height) as f32 + rng.gen_range(-0.5..0.5);
                let direction = (Vec3 {
                    x,
                    y,
                    z: focal_length as f32,
                })
                .normalize()
                .rotate_vec(camera_rotation);

                let focus_point = origin + direction * focal_distance;

                let (jitter_x, jitter_y) = random_in_unit_disc(&mut rng);
                let jittered_origin = Vec3 {
                    x: origin.x + jitter_x * aperture * 0.5,
                    y: origin.y + jitter_y * aperture * 0.5,
                    z: origin.z,
                };
                let jittered_direction = (focus_point - jittered_origin).normalize();

                let res = tracer::trace(
                    Ray {
                        origin: jittered_origin,
                        direction: jittered_direction,
                    },
                    &bvh,
                    bounces,
                    &mut rng,
                );
                *sample += res;
            }
        });

        s += 1;
        avg_samples_into(&samples, s, &mut avg_buf);

        let mut pixels = std::mem::take(&mut avg_buf);
        for pp in post_processors.clone() {
            pixels = pp.process(pixels);
        }

        samples_to_pixel_map_into(&pixels, &mut pixel_buf);

        let expected_size = (width * height * 4) as usize;
        if pixel_buf.len() != expected_size {
            log(&format!(
                "Error: pixel_buf size mismatch. Expected {}, got {}",
                expected_size,
                pixel_buf.len()
            ));
            avg_buf = pixels;
            return;
        }

        match local_context.create_image_data_with_sw_and_sh(width as f64, height as f64) {
            Ok(image_data) => {
                let array: js_sys::Uint8ClampedArray = js_sys::Reflect::get(&image_data, &"data".into())
                    .unwrap()
                    .unchecked_into();
                array.copy_from(&pixel_buf);
                if let Err(e) = local_context.put_image_data(&image_data, 0.0, 0.0) {
                    log(&format!("Error putting image data: {:?}", e));
                }
            }
            Err(e) => {
                log(&format!("Error creating ImageData: {:?}", e));
            }
        }

        avg_buf = pixels;

        let end = Date::now();
        let duration_ms = end - start;
        let _ = on_sample.call2(&JsValue::NULL, &JsValue::from(s), &JsValue::from(duration_ms));
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- avg_samples_into tests ---

    fn avg_samples(samples: &[Vec<Vec3>], count: u32) -> Vec<Vec<Vec3>> {
        let mut out: Vec<Vec<Vec3>> = samples
            .iter()
            .map(|row| vec![Vec3::new(0.0, 0.0, 0.0); row.len()])
            .collect();
        avg_samples_into(samples, count, &mut out);
        out
    }

    fn samples_to_pixel_map(samples: &[Vec<Vec3>]) -> Vec<u8> {
        let mut out = Vec::new();
        samples_to_pixel_map_into(samples, &mut out);
        out
    }

    #[test]
    fn avg_samples_count_1_returns_unchanged() {
        let samples = vec![vec![Vec3::new(100.0, 200.0, 50.0)]];
        let result = avg_samples(&samples, 1);
        assert_eq!(result, vec![vec![Vec3::new(100.0, 200.0, 50.0)],]);
    }

    #[test]
    fn avg_samples_divides_by_count() {
        let samples = vec![vec![Vec3::new(100.0, 200.0, 100.0)]];
        let result = avg_samples(&samples, 2);
        assert_eq!(result, vec![vec![Vec3::new(50.0, 100.0, 50.0)],]);
    }

    #[test]
    fn avg_samples_multiple_pixels_multiple_rows() {
        let samples = vec![
            vec![Vec3::new(40.0, 60.0, 80.0), Vec3::new(200.0, 200.0, 200.0)],
            vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(300.0, 330.0, 360.0)],
        ];
        let result = avg_samples(&samples, 2);
        assert_eq!(result[0][0], Vec3::new(20.0, 30.0, 40.0));
        assert_eq!(result[0][1], Vec3::new(100.0, 100.0, 100.0));
        assert_eq!(result[1][0], Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(result[1][1], Vec3::new(150.0, 165.0, 180.0));
    }

    // --- samples_to_pixel_map tests ---

    #[test]
    fn pixel_map_single_pixel() {
        let pixels = vec![vec![Vec3::new(255.0, 128.0, 0.0)]];
        let result = samples_to_pixel_map(&pixels);
        assert_eq!(result, vec![255, 128, 0, 255]);
    }

    #[test]
    fn pixel_map_row_order() {
        let pixels = vec![vec![Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0)]];
        let result = samples_to_pixel_map(&pixels);
        assert_eq!(result, vec![1, 2, 3, 255, 4, 5, 6, 255]);
    }

    #[test]
    fn pixel_map_multiple_rows() {
        let pixels = vec![vec![Vec3::new(10.0, 20.0, 30.0)], vec![Vec3::new(40.0, 50.0, 60.0)]];
        let result = samples_to_pixel_map(&pixels);
        assert_eq!(result, vec![10, 20, 30, 255, 40, 50, 60, 255]);
    }

    #[test]
    fn pixel_map_alpha_always_255() {
        let pixels = vec![vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)]];
        let result = samples_to_pixel_map(&pixels);
        // Every 4th byte should be 255
        for (i, &byte) in result.iter().enumerate() {
            if (i + 1) % 4 == 0 {
                assert_eq!(byte, 255, "alpha at index {} should be 255", i);
            }
        }
    }

    #[test]
    fn pixel_map_correct_length() {
        let pixels = vec![vec![Vec3::new(0.0, 0.0, 0.0); 3], vec![Vec3::new(0.0, 0.0, 0.0); 3]];
        let result = samples_to_pixel_map(&pixels);
        // 2 rows * 3 cols * 4 bytes (RGBA)
        assert_eq!(result.len(), 24);
    }

    // --- round-trip: avg_samples -> samples_to_pixel_map ---

    #[test]
    fn round_trip_avg_then_pixel_map() {
        // Running sum of 2 samples that averages to (100, 150, 200)
        let samples = vec![vec![Vec3::new(200.0, 300.0, 400.0)]];
        let averaged = avg_samples(&samples, 2);
        let pixels = samples_to_pixel_map(&averaged);
        assert_eq!(pixels, vec![100, 150, 200, 255]);
    }
}
