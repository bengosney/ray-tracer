use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use crate::log;
use crate::post_processing::PostProcess;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::tracer;
use crate::vec3::Vec3;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn avg_samples(samples: &[Vec<Vec<Vec3>>]) -> Vec<Vec<Vec3>> {
    samples
        .iter()
        .map(|row| row.iter().map(|v| Vec3::avg(v)).collect())
        .collect()
}

fn samples_to_pixel_map(samples: &[Vec<Vec3>]) -> Vec<u8> {
    let mut pixels = Vec::with_capacity(samples.len() * samples[0].len() * 4);
    for row in samples {
        for sample in row {
            pixels.push(sample.x as u8);
            pixels.push(sample.y as u8);
            pixels.push(sample.z as u8);
            pixels.push(255);
        }
    }
    pixels
}

pub fn render(scene: &Scene, ctx: &CanvasRenderingContext2d) {
    let half_width = (scene.width / 2) as i32;
    let half_height = (scene.height / 2) as i32;

    let width = scene.width;
    let height = scene.height;
    let focal_length = scene.focal_length;
    let bounces = scene.bounces;
    let entities = scene.entities().to_vec();
    let sample_count = scene.samples;
    let post_processors: Vec<Rc<dyn PostProcess>> = scene.post_processors().iter().map(Rc::clone).collect();

    let local_context = ctx.clone();

    let origin = Vec3::zero();
    let mut samples: Vec<Vec<Vec<Vec3>>> = vec![vec![vec![]; width as usize]; height as usize];

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut s = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if s > sample_count {
            log("done.");
            let _ = f.borrow_mut().take();
            return;
        }
        log(&format!("Sample {}", s));
        let mut rng = rand::thread_rng();
        for i in 0..width as i32 {
            for j in 0..height as i32 {
                use rand::Rng;
                let x = (i - half_width) as f32 + rng.gen_range(-0.5..0.5);
                let y = (j - half_height) as f32 + rng.gen_range(-0.5..0.5);
                let direction = (Vec3 {
                    x,
                    y,
                    z: focal_length as f32,
                })
                .normalize();

                let res = tracer::trace(Ray { origin, direction }, &entities, bounces);
                samples[j as usize][i as usize].push(res);
            }
        }

        let mut pixels = avg_samples(&samples);

        for pp in post_processors.clone() {
            pixels = pp.process(pixels);
        }

        let image_data =
            ImageData::new_with_u8_clamped_array_and_sh(Clamped(&samples_to_pixel_map(&pixels)), width, height)
                .unwrap();
        local_context.put_image_data(&image_data, 0.0, 0.0).ok();

        s += 1;
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
