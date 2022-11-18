use glam::Vec3;
use rand::Rng;

use crate::{hit::HitRecord, Ray};

#[derive(Default)]
pub enum Material {
    #[default]
    Unspecified,
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut impl Rng,
    ) -> bool {
        match self {
            Self::Unspecified => unreachable!(),
            Self::Lambertian(material) => {
                material.scatter(ray_in, hit_record, attenuation, scattered, rng)
            }
            Self::Metal(material) => {
                material.scatter(ray_in, hit_record, attenuation, scattered, rng)
            }
        }
    }
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut impl Rng,
    ) -> bool {
        let target = hit_record.point + hit_record.normal + random_in_unit_sphere(rng);
        *scattered = Ray {
            origin: hit_record.point,
            direction: target - hit_record.point,
        };
        *attenuation = self.albedo;
        true
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut impl Rng,
    ) -> bool {
        let reflected = reflect(ray_in.direction.normalize(), hit_record.normal);
        *scattered = Ray {
            origin: hit_record.point,
            direction: reflected + self.fuzz * random_in_unit_sphere(rng),
        };
        *attenuation = self.albedo;
        scattered.direction.dot(hit_record.normal) > 0.
    }
}

fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    let mut p;
    while {
        p = 2. * Vec3::new(rng.gen(), rng.gen(), rng.gen());
        p.length_squared() >= 1.
    } {}
    p
}
