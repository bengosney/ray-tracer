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
    pub fn intersect(&self, ray: Ray, inv_dir: Vec3) -> Option<f32> {
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

        if t_min <= t_max && t_max > 0.0 {
            Some(t_min.max(0.0))
        } else {
            None
        }
    }
}

enum Axis {
    X,
    Y,
    Z,
}

impl From<Vec3> for Axis {
    fn from(v: Vec3) -> Self {
        let v_abs = Vec3::new(v.x.abs(), v.y.abs(), v.z.abs());
        if v_abs.x > v_abs.y && v_abs.x > v_abs.z {
            Axis::X
        } else if v_abs.y > v_abs.z {
            Axis::Y
        } else {
            Axis::Z
        }
    }
}

impl From<Aabb> for Axis {
    fn from(aabb: Aabb) -> Self {
        Axis::from(aabb.max - aabb.min)
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

impl Node {
    pub fn aabb(&self) -> Aabb {
        match self {
            Node::Branch { aabb, .. } => *aabb,
            Node::Leaf { aabb, .. } => *aabb,
        }
    }
}

pub struct Tree {
    node: Node,
    unbound: Vec<Entity>,
}

impl Tree {
    pub fn build(entities: &[Entity]) -> Self {
        let (with_bounds, without_bounds): (Vec<&Entity>, Vec<&Entity>) =
            entities.iter().partition(|e| e.bounds().is_ok());

        let entities_with_bounds: Vec<Entity> = with_bounds.into_iter().cloned().collect();
        let node = Node::build_recursive(entities_with_bounds, 0);

        let unbound: Vec<Entity> = without_bounds.into_iter().cloned().collect();

        Self { node, unbound }
    }

    pub fn find_intersection(&self, ray: Ray) -> Option<crate::intersection::Intersection> {
        let mut closest = crate::intersection::Intersection::empty();

        for entity in &self.unbound {
            if let Some(hit) = entity.intersection(ray) {
                closest = crate::intersection::Intersection::closest(hit, closest);
            }
        }

        let inv_dir = Vec3::new(1.0, 1.0, 1.0) / ray.direction;
        if self.node.aabb().intersect(ray, inv_dir).is_some() {
            self.node.find_intersection(ray, inv_dir, &mut closest);
        }

        if closest.entity.is_some() {
            Some(closest)
        } else {
            None
        }
    }
}

impl Node {
    fn find_intersection(&self, ray: Ray, inv_dir: Vec3, closest: &mut crate::intersection::Intersection) {
        match self {
            Node::Branch { left, right, .. } => {
                let t_left = left.aabb().intersect(ray, inv_dir);
                let t_right = right.aabb().intersect(ray, inv_dir);

                match (t_left, t_right) {
                    (Some(tl), Some(tr)) => {
                        if tl < tr {
                            if tl < closest.dist {
                                left.find_intersection(ray, inv_dir, closest);
                            }
                            if tr < closest.dist {
                                right.find_intersection(ray, inv_dir, closest);
                            }
                        } else {
                            if tr < closest.dist {
                                right.find_intersection(ray, inv_dir, closest);
                            }
                            if tl < closest.dist {
                                left.find_intersection(ray, inv_dir, closest);
                            }
                        }
                    }
                    (Some(tl), None) => {
                        if tl < closest.dist {
                            left.find_intersection(ray, inv_dir, closest);
                        }
                    }
                    (None, Some(tr)) => {
                        if tr < closest.dist {
                            right.find_intersection(ray, inv_dir, closest);
                        }
                    }
                    (None, None) => {}
                }
            }
            Node::Leaf { entities, .. } => {
                for entity in entities {
                    if let Some(hit) = entity.intersection(ray) {
                        *closest = crate::intersection::Intersection::closest(hit, *closest);
                    }
                }
            }
        }
    }

    fn build_recursive(entities: Vec<Entity>, depth: usize) -> Self {
        let aabb = Self::calculate_bounds(&entities);

        if entities.len() <= (depth * 2) {
            return Node::Leaf { aabb, entities };
        }

        let axis = Axis::from(aabb);
        let midpoint = (aabb.min + aabb.max) * 0.5;
        let mid_val = match axis {
            Axis::X => midpoint.x,
            Axis::Y => midpoint.y,
            Axis::Z => midpoint.z,
        };

        let mut left_entities = Vec::new();
        let mut right_entities = Vec::new();

        for entity in entities {
            let pos = entity.position();
            let val = match axis {
                Axis::X => pos.x,
                Axis::Y => pos.y,
                Axis::Z => pos.z,
            };

            if val < mid_val {
                left_entities.push(entity);
            } else {
                right_entities.push(entity);
            }
        }

        // If split failed (e.g. all entities on one side), fallback to simple median split
        if left_entities.is_empty() || right_entities.is_empty() {
            let mut all = if left_entities.is_empty() {
                right_entities
            } else {
                left_entities
            };

            all.sort_by(|a, b| {
                let a_pos = a.position();
                let b_pos = b.position();
                let (a_val, b_val) = match axis {
                    Axis::X => (a_pos.x, b_pos.x),
                    Axis::Y => (a_pos.y, b_pos.y),
                    Axis::Z => (a_pos.z, b_pos.z),
                };
                a_val.partial_cmp(&b_val).unwrap_or(std::cmp::Ordering::Equal)
            });

            let mid = all.len() / 2;
            right_entities = all.split_off(mid);
            left_entities = all;
        }

        Node::Branch {
            aabb,
            left: Box::new(Self::build_recursive(left_entities, depth + 1)),
            right: Box::new(Self::build_recursive(right_entities, depth + 1)),
        }
    }

    fn calculate_bounds(entities: &[Entity]) -> Aabb {
        entities.iter().filter_map(|e| e.bounds().ok()).fold(
            Aabb {
                min: Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
                max: Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
            },
            |mut acc, (e_min, e_max)| {
                acc.min = acc.min.min(e_min);
                acc.max = acc.max.max(e_max);
                acc
            },
        )
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
        let inv_dir_hit = Vec3::new(1.0, 1.0, 1.0) / ray_hit.direction;
        assert!(aabb.intersect(ray_hit, inv_dir_hit).is_some());

        // Ray missing
        let ray_miss = Ray {
            origin: Vec3::new(0.0, 2.0, -5.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        let inv_dir_miss = Vec3::new(1.0, 1.0, 1.0) / ray_miss.direction;
        assert!(aabb.intersect(ray_miss, inv_dir_miss).is_none());

        // Ray inside
        let ray_inside = Ray {
            origin: Vec3::zero(),
            direction: Vec3::new(0.0, 1.0, 0.0),
        };
        let inv_dir_inside = Vec3::new(1.0, 1.0, 1.0) / ray_inside.direction;
        assert!(aabb.intersect(ray_inside, inv_dir_inside).is_some());
    }
}
