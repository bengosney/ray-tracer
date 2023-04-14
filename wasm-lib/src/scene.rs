use rand::Rng;
use wasm_bindgen::prelude::*;

use crate::{rgb::RGB, vec3::Vec3};

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq)]
pub enum Shape {
    Sphere,
    Cube,
}

#[wasm_bindgen()]
#[derive(Copy, Clone, PartialEq)]
pub struct Entity {
    pub shape: Shape,
    pub position: Vec3,
    pub emission: RGB,
    pub reflectivity: RGB,
    pub roughness: f32,
    pub radius: f32,
}

impl Entity {
    pub fn intersection(self, origin: Vec3, direction: Vec3) -> Intersection {
        match self.shape {
            Shape::Sphere => self.sphere_intersection(origin, direction),
            Shape::Cube => todo!("Cube intersection"),
        }
    }

    fn sphere_intersection(self, origin: Vec3, direction: Vec3) -> Intersection {
        let mut rng = rand::thread_rng();

        let sphereRay = self.position - origin;
        let distSphereRay = sphereRay.mag();
        let distToClosestPointOnRay = sphereRay.dot(direction);
        let distFromClosestPointToSphere =
            (distSphereRay.powi(2) - distToClosestPointOnRay.powi(2)).sqrt();

        let distToIntersection = distToClosestPointOnRay
            - (self.radius.powi(2) - distFromClosestPointToSphere.powi(2))
                .abs()
                .sqrt();
        let point = origin + (direction * distToIntersection);
        let roughness = Vec3::new(
            rng.gen_range(-0.5..0.5),
            rng.gen_range(-0.5..0.5),
            rng.gen_range(-0.5..0.5),
        ) * self.roughness;
        let normal = (point - self.position).normalize() + roughness;

        if distToClosestPointOnRay > 0.0 && distFromClosestPointToSphere < self.radius {
            return Intersection {
                collided: true,
                dist: distToIntersection,
                point: point,
                normal: normal,
                entity: Some(self),
            };
        }

        Intersection::empty()
    }
}

#[wasm_bindgen]
impl Entity {
    #[wasm_bindgen(constructor)]
    pub fn new(
        shape: Shape,
        position: Vec3,
        emission: RGB,
        reflectivity: RGB,
        roughness: f32,
        radius: f32,
    ) -> Self {
        Self {
            shape,
            position,
            emission,
            reflectivity,
            roughness,
            radius,
        }
    }
}

#[wasm_bindgen]
pub struct Scene {
    entities: Vec<Entity>,
    pub width: u32,
    pub height: u32,
    pub focal_length: u32,
    pub samples: u32,
    pub bounces: u32,
}

struct Intersection {
    collided: bool,
    dist: f32,
    point: Vec3,
    normal: Vec3,
    entity: Option<Entity>,
}

impl Intersection {
    pub fn empty() -> Self {
        Intersection {
            collided: false,
            point: Vec3::new(0.0, 0.0, 0.0),
            dist: f32::INFINITY,
            normal: Vec3::new(0.0, 0.0, 0.0),
            entity: None,
        }
    }
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

    fn intersection(origin: Vec3, direction: Vec3, entities: Vec<Entity>) -> Intersection {
        let mut closestIntersection: Intersection = Intersection::empty();

        for entity in entities {
            let intersection = entity.intersection(origin, direction);
            if intersection.dist < closestIntersection.dist {
                closestIntersection = intersection;
            }
        }

        return closestIntersection;
    }

    fn trace(origin: Vec3, direction: Vec3, entities: Vec<Entity>, steps: u32) -> Vec3 {
        let intersect = Self::intersection(origin, direction, entities);

        if intersect.collided && steps > 0 {
            let reflectedDirection = direction.reflect(intersect.normal);

            let bounce = Self::trace(
                intersect.point,
                reflectedDirection,
                entities, //objects.filter((o) => o != intersect.object),
                steps - 1,
            );


            return add(
                rgbToVec3(intersect.object?.emission),
                mulParts(bounce, rgbToVec3(intersect.object.reflectivity)),
            );
        }

        return Vec3::new(0.0, 0.0, 0.0);
    }

    pub fn render(&self) -> String {
        let halfWidth: u32 = self.width / 2;
        let halfHeight: u32 = self.height / 2;

        let origin: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut samples: Vec<Vec<Vec<Vec3>>> =
            vec![vec![vec![]; self.height as usize]; self.width as usize];

        for _ in 0..self.samples {
            for i in 0..self.width {
                for j in 0..self.height {
                    let x = i - halfWidth;
                    let y = j - halfHeight;
                    let direction = (Vec3 {
                        x: x as f32,
                        y: y as f32,
                        z: self.focal_length as f32,
                    })
                    .normalize();

                    samples[x as usize][y as usize].push(direction);
                    //let colour = Vec3::avg(&samples[x as usize][y as usize]);

                    //drawPixel({ x: i, y: j }, vec3ToRGB(colour));
                }
            }
        }

        "Rendering".to_string()
    }
}
