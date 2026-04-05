use crate::entity::Entity;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::iter::FromIterator;

#[derive(Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn intersect(&self, ray: Ray) -> bool {
        let inv_dir = Vec3::new(1.0, 1.0, 1.0) / ray.direction;
        let t0 = (self.min - ray.origin) * inv_dir;
        let t1 = (self.max - ray.origin) * inv_dir;

        let mut t_min = f32::NEG_INFINITY;
        let mut t_max = f32::INFINITY;

        // X
        t_min = t_min.max(t0.x.min(t1.x));
        t_max = t_max.min(t0.x.max(t1.x));

        // Y
        t_min = t_min.max(t0.y.min(t1.y));
        t_max = t_max.min(t0.y.max(t1.y));

        // Z
        t_min = t_min.max(t0.z.min(t1.z));
        t_max = t_max.min(t0.z.max(t1.z));

        t_min <= t_max && t_max > 0.0
    }
}

enum Axis {
    X,
    Y,
    Z,
}

impl From<Vec3> for Axis {
    fn from(v: Vec3) -> Self {
        if v.x > v.y && v.x > v.z {
            Axis::X
        } else if v.y > v.z {
            Axis::Y
        } else {
            Axis::Z
        }
    }
}

impl From<Aabb> for Axis {
    fn from(aabb: Aabb) -> Self {
        Axis::from(aabb.min - aabb.max)
    }
}

#[derive(Clone)]
pub enum Node {
    Branch {
        aabb: Aabb,
        left: Box<Node>,
        right: Box<Node>,
    },
    Leaf {
        aabb: Aabb,
        entities: Vec<Entity>,
    },
}

pub struct Tree {
    node: Node,
}

impl Tree {
    pub fn build(entities: &[Entity]) -> Self {
        let entities: Vec<Entity> = entities.iter().filter(|e| e.bounds().is_ok()).cloned().collect();
        let node = Node::build_recursive(entities);
        Self { node }
    }

    pub fn collect_entities(&self, ray: Ray) -> Vec<Entity> {
        let mut results: Vec<&Entity> = Vec::new();
        self.node.collect_entities(ray, &mut results);

        results.into_iter().cloned().collect()
    }
}

impl Node {
    fn collect_entities<'a>(&'a self, ray: Ray, results: &mut Vec<&'a Entity>) {
        match self {
            Node::Branch { left, right, aabb } => {
                if aabb.intersect(ray) {
                    left.collect_entities(ray, results);
                    right.collect_entities(ray, results);
                }
            }
            Node::Leaf { entities, aabb } => {
                if aabb.intersect(ray) {
                    results.extend(entities.iter());
                }
            }
        }
    }

    fn build_recursive(mut entities: Vec<Entity>) -> Self {
        let aabb = Self::calculate_bounds(&entities);

        if entities.len() <= 1 {
            return Node::Leaf { aabb, entities };
        }

        let axis = Axis::from(aabb);

        entities.sort_by(|a, b| {
            let a_pos = a.position();
            let b_pos = b.position();
            let (a_val, b_val) = match axis {
                Axis::X => (a_pos.x, b_pos.x),
                Axis::Y => (a_pos.y, b_pos.y),
                Axis::Z => (a_pos.z, b_pos.z),
            };
            a_val.partial_cmp(&b_val).unwrap_or(std::cmp::Ordering::Equal)
        });

        let mid = entities.len() / 2;
        let right_entities = entities.split_off(mid);

        Node::Branch {
            aabb,
            left: Box::new(Self::build_recursive(entities)),
            right: Box::new(Self::build_recursive(right_entities)),
        }
    }

    fn calculate_bounds(entities: &[Entity]) -> Aabb {
        entities.iter().filter_map(|e| e.bounds().ok()).collect()
    }
}

impl FromIterator<(Vec3, Vec3)> for Aabb {
    fn from_iter<I: IntoIterator<Item = (Vec3, Vec3)>>(iter: I) -> Self {
        iter.into_iter().fold(
            Aabb {
                min: Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
                max: Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
            },
            |acc, (e_min, e_max)| Aabb {
                min: acc.min.min(e_min),
                max: acc.max.max(e_max),
            },
        )
    }
}

impl Extend<(Vec3, Vec3)> for Aabb {
    fn extend<T: IntoIterator<Item = (Vec3, Vec3)>>(&mut self, iter: T) {
        for (e_min, e_max) in iter {
            self.min = self.min.min(e_min);
            self.max = self.max.max(e_max);
        }
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
    fn test_calculate_bounds_empty() {
        let entities = vec![];
        let bounds = Node::calculate_bounds(&entities);
        assert_eq!(bounds.min.x, f32::INFINITY);
        assert_eq!(bounds.max.x, f32::NEG_INFINITY);
    }

    #[test]
    fn test_calculate_bounds_single_sphere() {
        let sphere = Entity::new_sphere(Vec3::new(0.0, 0.0, 0.0), test_material(), 1.0);
        let entities = vec![sphere];
        let bounds = Node::calculate_bounds(&entities);
        assert_eq!(bounds.min, Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(bounds.max, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_calculate_bounds_multiple_spheres() {
        let sphere1 = Entity::new_sphere(Vec3::new(0.0, 0.0, 0.0), test_material(), 1.0);
        let sphere2 = Entity::new_sphere(Vec3::new(2.0, 2.0, 2.0), test_material(), 1.0);
        let entities = vec![sphere1, sphere2];
        let bounds = Node::calculate_bounds(&entities);
        assert_eq!(bounds.min, Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(bounds.max, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_calculate_bounds_ignores_planes() {
        let sphere = Entity::new_sphere(Vec3::new(0.0, 0.0, 0.0), test_material(), 1.0);
        let plane = Entity::new_plane(Vec3::new(0.0, -2.0, 0.0), test_material(), Vec3::new(0.0, 1.0, 0.0));
        let entities = vec![sphere, plane];
        let bounds = Node::calculate_bounds(&entities);
        // Plane returns Err on bounds(), so it should be ignored by calculate_bounds
        assert_eq!(bounds.min, Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(bounds.max, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_aabb_intersection() {
        let aabb = Aabb {
            min: Vec3::new(-1.0, -1.0, -1.0),
            max: Vec3::new(1.0, 1.0, 1.0),
        };

        // Ray hitting directly
        let ray_hit = Ray {
            origin: Vec3::new(0.0, 0.0, -5.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        assert!(aabb.intersect(ray_hit));

        // Ray missing
        let ray_miss = Ray {
            origin: Vec3::new(0.0, 2.0, -5.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        assert!(!aabb.intersect(ray_miss));

        // Ray inside
        let ray_inside = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(0.0, 1.0, 0.0),
        };
        assert!(aabb.intersect(ray_inside));
    }

    #[test]
    fn test_tree_collect_entities() {
        let sphere1 = Entity::new_sphere(Vec3::new(0.0, 0.0, 5.0), test_material(), 1.0);
        let sphere2 = Entity::new_sphere(Vec3::new(10.0, 10.0, 10.0), test_material(), 1.0);
        let entities = vec![sphere1, sphere2];
        let tree = Tree::build(&entities);

        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };

        let collected = tree.collect_entities(ray);
        assert_eq!(collected.len(), 1);
        assert_eq!(collected[0].position(), Vec3::new(0.0, 0.0, 5.0));
    }
}
