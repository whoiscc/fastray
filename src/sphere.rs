use std::{ops::Range, sync::Arc};

use glam::Vec3;

use crate::{material::Material, Hit};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl Hit for Sphere {
    fn hit(
        &self,
        ray: &crate::Ray,
        t_range: Range<f32>,
        hit_record: &mut crate::hit::HitRecord,
    ) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            for t in [
                (-b - discriminant.sqrt()) / a,
                (-b + discriminant.sqrt()) / a,
            ] {
                if t_range.contains(&t) {
                    hit_record.t = t;
                    hit_record.point = ray.p(t);
                    hit_record.normal = (hit_record.point - self.center) / self.radius;
                    hit_record.material = self.material.clone();
                    return true;
                }
            }
        }
        false
    }
}
