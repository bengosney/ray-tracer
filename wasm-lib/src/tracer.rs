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

pub fn trace(ray: Ray, entities: &[Entity], steps: u32, rng: &mut impl Rng) -> Vec3 {
    if steps == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match find_intersection(ray, entities) {
        Some(intersection) => {
            let entity: Entity = intersection.entity.unwrap();
            let material = entity.material();
            let emitted = Vec3::from(material.emission);

            let mut normal = intersection.normal;
            let mut ni_over_nt = 1.0 / material.ior;
            let mut cos_theta = (ray.direction * -1.0).dot(normal);

            if cos_theta < 0.0 {
                // Ray is inside the object, flip normal and IOR
                normal = normal * -1.0;
                cos_theta = -cos_theta;
                ni_over_nt = material.ior;
            } else {
                cos_theta = cos_theta.min(1.0);
            }

            let albedo = Vec3::from(material.albedo);
            let r = rng.gen::<f32>();

            let (bounce_ray, brdf_weight) = if material.transmission > 0.0 {
                // Handle Dielectric (Glass/Transparent)
                let reflectance = Vec3::reflectance(cos_theta, ni_over_nt);

                if r < reflectance {
                    // Reflection
                    let reflected = ray.direction.reflect(normal);
                    let direction = (reflected + Vec3::rng_normal(rng) * material.roughness).normalize();
                    let origin = intersection.point + normal * 0.001;
                    (Ray { origin, direction }, Vec3::new(1.0, 1.0, 1.0))
                } else {
                    // Refraction (Transmission)
                    let refracted = Vec3::refract(ray.direction, normal, ni_over_nt);
                    match refracted {
                        Some(refracted_dir) => {
                            let direction = (refracted_dir + Vec3::rng_normal(rng) * material.roughness).normalize();
                            let origin = intersection.point - normal * 0.001;
                            (Ray { origin, direction }, albedo)
                        }
                        None => {
                            // Total Internal Reflection
                            let reflected = ray.direction.reflect(normal);
                            let direction = (reflected + Vec3::rng_normal(rng) * material.roughness).normalize();
                            let origin = intersection.point + normal * 0.001;
                            (Ray { origin, direction }, Vec3::new(1.0, 1.0, 1.0))
                        }
                    }
                }
            } else {
                // Handle Metallic/Diffuse
                let f0_dielectric = Vec3::new(0.04, 0.04, 0.04);
                let f0 = Vec3::lerp(f0_dielectric, albedo, material.metallic);
                let fresnel = Vec3::fresnel_schlick(f0, cos_theta);
                let reflectance = ((fresnel.x + fresnel.y + fresnel.z) / 3.0).clamp(0.05, 0.95);

                if r < reflectance {
                    // Specular Reflection
                    let reflected = ray.direction.reflect(normal);
                    let direction = (reflected + Vec3::rng_normal(rng) * material.roughness).normalize();
                    let origin = intersection.point + normal * 0.001;

                    let specular_color = Vec3::lerp(Vec3::new(1.0, 1.0, 1.0), albedo, material.metallic);
                    (
                        Ray { origin, direction },
                        specular_color * (fresnel * (1.0 / reflectance)),
                    )
                } else {
                    // Diffuse Reflection
                    let diffuse_weight = 1.0 - material.metallic;
                    if diffuse_weight < 0.001 {
                        return emitted;
                    }

                    let direction = (normal + Vec3::rng_hemisphere(normal, rng)).normalize();
                    let origin = intersection.point + normal * 0.001;
                    let diffuse_prob = 1.0 - reflectance;

                    let one_minus_fresnel = Vec3::new(1.0, 1.0, 1.0) - fresnel;
                    (
                        Ray { origin, direction },
                        albedo * one_minus_fresnel * (diffuse_weight / diffuse_prob),
                    )
                }
            };

            let incoming = trace(bounce_ray, entities, steps - 1, rng);
            emitted + (incoming * brdf_weight)
        }
        _ => sky_color(ray),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Material;
    use crate::rgb::Rgb;

    fn test_material() -> Material {
        Material::new(Rgb::new(0.0, 0.0, 0.0), Rgb::new(1.0, 1.0, 1.0), 0.0, 0.0, 0.0, 1.5)
    }

    #[test]
    fn test_find_intersection() {
        let sphere1 = Entity::new_sphere(Vec3::new(0.0, 0.0, 10.0), test_material(), 2.0);
        let sphere2 = Entity::new_sphere(Vec3::new(0.0, 0.0, 5.0), test_material(), 1.0);
        let entities = vec![sphere1, sphere2];

        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };

        let intersection = find_intersection(ray, &entities).unwrap();
        assert_eq!(intersection.dist, 4.0);
        assert_eq!(intersection.point, Vec3::new(0.0, 0.0, 4.0));
    }
}
