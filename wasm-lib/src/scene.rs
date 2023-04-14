use wasm_bindgen::prelude::*;

use crate::{entity::Entity, intersection::Intersection, vec3::Vec3, rgb::{RGB, RGBA_tuple}};

#[wasm_bindgen]
pub struct Scene {
    entities: Vec<Entity>,
    pub width: u32,
    pub height: u32,
    pub focal_length: u32,
    pub samples: u32,
    pub bounces: u32,
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, focal_length: u32, samples: u32, bounces: u32) -> Self {
        Self {
            entities: vec![],
            width,
            height,
            focal_length,
            samples,
            bounces,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    fn intersection(origin: Vec3, direction: Vec3, entities: &Vec<Entity>) -> Intersection {
        let mut closest_intersection: Intersection = Intersection::empty();

        for entity in entities {
            let intersection = entity.intersection(origin, direction);
            if intersection.dist < closest_intersection.dist {
                closest_intersection = intersection;
            }
        }

        return closest_intersection;
    }

    fn trace(origin: Vec3, direction: Vec3, entities: &Vec<Entity>, steps: u32) -> Vec3 {
        let intersect = Self::intersection(origin, direction, entities);

        if intersect.collided && steps > 0 {
            let reflected_direction = direction.reflect(intersect.normal);

            let bounce = Self::trace(
                intersect.point,
                reflected_direction,
                entities, //objects.filter((o) => o != intersect.object),
                steps - 1,
            );

            let entity = intersect.entity.unwrap();

            return Vec3::from(entity.emission) + (bounce * Vec3::from(entity.reflectivity));
        }

        return Vec3::new(0.0, 0.0, 0.0);
    }

    pub fn render(&self) -> Vec<u16> {
        let half_width: i32 = (self.width / 2) as i32;
        let half_height: i32 = (self.height / 2) as i32;

        let origin: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut samples: Vec<Vec<Vec<Vec3>>> =
            vec![vec![vec![]; self.height as usize]; self.width as usize];

        for _ in 0..self.samples {
            for i in 0..self.width as i32 {
                for j in 0..self.height as i32 {
                    let x: i32 = i - half_width;
                    let y: i32 = j - half_height;
                    let direction = (Vec3 {
                        x: x as f32,
                        y: y as f32,
                        z: self.focal_length as f32,
                    })
                    .normalize();

                    let res = Self::trace(origin, direction, &self.entities, self.bounces);
                    samples[x as usize][y as usize].push(res);
                    //let colour = Vec3::avg(&samples[x as usize][y as usize]);

                    //drawPixel({ x: i, y: j }, vec3ToRGB(colour));
                }
            }
        }

        todo!("returned rendered pixles")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let scene = Scene {
            entities: vec![],
            width: 10,
            height: 10,
            focal_length: 50,
            samples: 10,
            bounces: 4,
        };
        let rendered = scene.render();

        assert_eq!(rendered, vec![0;(scene.width * scene.height * 4) as usize]);
    }
}
