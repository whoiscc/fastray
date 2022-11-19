use std::{ops::Range, sync::Arc};

use glam::Vec3;

use crate::{material::Material, Ray};

pub trait Hit {
    fn hit(&self, ray: &Ray, t_range: Range<f32>, hit_record: &mut HitRecord) -> bool;
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
}
