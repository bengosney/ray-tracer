use rand::Rng;

use crate::entity::Entity;
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn find_intersection(ray: Ray, entities: &[Entity]) -> Option<Intersection> {
    let intersection = entities.iter().fold(Intersection::empty(), |previous, entity| {
        match entity.intersection(ray) {
            Some(intersection) => Intersection::closest(intersection, previous),
            None => previous,
        }
    });

    match intersection {
        Intersection { entity: None, .. } => None,
        intersection => Some(intersection),
    }
}

fn sky_color(ray: Ray) -> Vec3 {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t) * 175.0
}

pub fn trace(ray: Ray, entities: &[Entity], steps: u32) -> Vec3 {
    if steps == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match find_intersection(ray, entities) {
        Some(intersection) => {
            let entity: Entity = intersection.entity.unwrap();
            let material = entity.material();
            let emitted = Vec3::from(material.emission);

            let normal = intersection.normal;
            let view_dir = (ray.direction * -1.0).normalize();
            let cos_theta = normal.dot(view_dir).max(0.0);

            let albedo = Vec3::from(material.albedo);
            let dielectric_f0 = Vec3::new(0.04, 0.04, 0.04);
            let f0 = Vec3::lerp(dielectric_f0, albedo, material.metallic);

            let fresnel = Vec3::fresnel_schlick(f0, cos_theta);
            let specular_prob = ((fresnel.x + fresnel.y + fresnel.z) / 3.0).clamp(0.05, 0.95);

            let mut rng = rand::thread_rng();
            let origin = intersection.point + normal * 0.001;

            let (bounce_ray, brdf_weight) = if rng.gen::<f32>() < specular_prob {
                let reflected = ray.direction.reflect(normal);
                let direction = (reflected + Vec3::rng_normal() * material.roughness).normalize();

                let specular_color = Vec3::lerp(Vec3::new(1.0, 1.0, 1.0), albedo, material.metallic);

                (
                    Ray { origin, direction },
                    specular_color * (fresnel * (1.0 / specular_prob)),
                )
            } else {
                let diffuse_weight = 1.0 - material.metallic;
                if diffuse_weight < 0.001 {
                    return emitted;
                }

                let direction = (normal + Vec3::rng_hemisphere(normal)).normalize();
                let diffuse_prob = 1.0 - specular_prob;

                let one_minus_fresnel = Vec3::new(1.0, 1.0, 1.0) - fresnel;
                (
                    Ray { origin, direction },
                    albedo * one_minus_fresnel * (diffuse_weight / diffuse_prob),
                )
            };

            let incoming = trace(bounce_ray, entities, steps - 1);
            emitted + (incoming * brdf_weight)
        }
        _ => sky_color(ray),
    }
}
