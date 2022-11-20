use glam::Vec3;
use rand::Rng;

use crate::{shape::HitRecord, Ray};

#[derive(Default)]
pub enum Material {
    #[default]
    Unspecified,
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
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
            Self::Dielectric(material) => {
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
        let mut direction = hit_record.normal + random_unit_in_sphere(rng).normalize();
        // consider extract a util
        if direction.max_element() < 1e-8 {
            direction = hit_record.normal;
        }
        *scattered = Ray {
            origin: hit_record.point,
            direction,
        };
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
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
            direction: reflected + self.fuzz * random_unit_in_sphere(rng),
        };
        *attenuation = self.albedo;
        scattered.direction.dot(hit_record.normal) > 0.
    }
}

pub struct Dielectric {
    pub refractive_index: f32,
}

fn refract(v: Vec3, n: Vec3, refraction_ratio: f32) -> Vec3 {
    // assert v is normalized
    let r_out_perp = refraction_ratio * (v + (-v.dot(n) * n));
    let r_out_parallel = -(1. - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f32, refraction_ratio: f32) -> f32 {
    let r0 = ((1. - refraction_ratio) / (1. + refraction_ratio)).powi(2);
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

impl Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut impl Rng,
    ) -> bool {
        *attenuation = Vec3::ONE;

        let refraction_ratio = if hit_record.front_face {
            // seems like we always assume the material is adjacent to air
            // probably ok, since A -> B refraction should be equivalent to A -> air -> B
            1. / self.refractive_index
        } else {
            self.refractive_index / 1.
        };
        let direction_in = ray_in.direction.normalize();
        let cos_theta = f32::min(direction_in.dot(-hit_record.normal), 1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        *scattered = Ray {
            origin: hit_record.point,
            direction: if cannot_refract
                || reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>()
            {
                reflect(direction_in, hit_record.normal)
            } else {
                refract(direction_in, hit_record.normal, refraction_ratio)
            },
        };
        true
    }
}

fn random_unit_in_sphere(rng: &mut impl Rng) -> Vec3 {
    let mut p;
    while {
        p = 2. * Vec3::new(rng.gen(), rng.gen(), rng.gen());
        p.length_squared() >= 1.
    } {}
    p
}
