use std::{ops::Range, sync::Arc};

use glam::Vec3;

use crate::{
    material::Material,
    shape::{Aabb, HitRecord},
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn hit(&self, ray: &crate::Ray, t_range: Range<f32>) -> Option<HitRecord> {
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
                    let mut hit_record = HitRecord {
                        t,
                        point: ray.at(t),
                        material: self.material.clone(),
                        ..Default::default()
                    };
                    hit_record.set_face_normal(ray, (hit_record.point - self.center) / self.radius);
                    return Some(hit_record);
                }
            }
        }
        None
    }

    pub fn bounding_box(&self) -> Option<crate::shape::Aabb> {
        Some(Aabb {
            min: self.center - self.radius,
            max: self.center + self.radius,
        })
    }
}
