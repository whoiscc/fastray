use std::{mem::swap, ops::Range, sync::Arc};

use glam::Vec3;
use rand::Rng;

use crate::{material::Material, Ray};

pub trait Hit {
    fn hit(&self, ray: &Ray, t_range: Range<f32>, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> Option<Aabb>;
}

#[derive(Clone, Default)]
pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Arc<Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

// maybe called `World` directly
pub struct HitList(pub Vec<Arc<dyn Hit + Send + Sync>>);

impl Hit for HitList {
    fn hit(&self, ray: &Ray, mut t_range: Range<f32>, hit_record: &mut HitRecord) -> bool {
        let mut do_hit = false;
        for entity in &self.0 {
            if !entity.hit(ray, t_range.clone(), hit_record) {
                continue;
            }
            do_hit = true;
            t_range.end = hit_record.t;
        }
        do_hit
    }

    fn bounding_box(&self) -> Option<Aabb> {
        self.0
            .iter()
            .map(|entity| entity.bounding_box())
            .reduce(|bounding_box, another_bounding_box| {
                bounding_box.zip(another_bounding_box).map(
                    |(bounding_box, another_bounding_box)| {
                        Aabb::surrounding_box(bounding_box, another_bounding_box)
                    },
                )
            })
            .flatten()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn surrounding_box(self, another_box: Self) -> Self {
        Self {
            min: Vec3::min(self.min, another_box.min),
            max: Vec3::max(self.max, another_box.max),
        }
    }

    pub fn hit(&self, ray: &Ray, mut t_range: Range<f32>) -> bool {
        for a in 0..3 {
            let inv_d = 1. / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;
            if inv_d < 0. {
                swap(&mut t0, &mut t1);
            }
            t_range.start = f32::max(t_range.start, t0);
            t_range.end = f32::min(t_range.end, t1);
            if t_range.is_empty() {
                return false;
            }
        }
        true
    }
}

pub struct BvhNode {
    pub left: Arc<dyn Hit + Send + Sync>,
    pub right: Arc<dyn Hit + Send + Sync>,
    pub bounding_box: Aabb,
}

impl Hit for BvhNode {
    fn hit(&self, ray: &Ray, mut t_range: Range<f32>, hit_record: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(ray, t_range.clone()) {
            return false;
        }
        let hit_left = self.left.hit(ray, t_range.clone(), hit_record);
        if hit_left {
            t_range.end = hit_record.t;
        }
        let hit_right = self.right.hit(ray, t_range, hit_record);
        hit_left || hit_right
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}

impl BvhNode {
    pub fn new(entities: &mut [Arc<dyn Hit + Send + Sync>], rng: &mut impl Rng) -> Self {
        let axis = rng.gen_range(0..3);
        let k = |entity: &Arc<dyn Hit + Send + Sync>| entity.bounding_box().unwrap().min[axis];
        let (left, right) = match entities {
            [] => unreachable!(),
            [entity] => (entity.clone(), entity.clone()),
            [left_entity, right_entity] => {
                if k(left_entity) < k(right_entity) {
                    (left_entity.clone(), right_entity.clone())
                } else {
                    (right_entity.clone(), left_entity.clone())
                }
            }
            entities => {
                entities.sort_unstable_by(|entity, another_entity| {
                    k(entity).total_cmp(&k(another_entity))
                });
                let (left_node, right_node) = entities.split_at_mut(entities.len() / 2);
                (
                    Arc::new(Self::new(left_node, rng)) as Arc<dyn Hit + Send + Sync>,
                    Arc::new(Self::new(right_node, rng)) as Arc<dyn Hit + Send + Sync>,
                )
            }
        };
        Self {
            bounding_box: Aabb::surrounding_box(
                left.bounding_box().unwrap(),
                right.bounding_box().unwrap(),
            ),
            left,
            right,
        }
    }
}
