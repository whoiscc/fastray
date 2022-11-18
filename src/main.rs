use std::sync::Arc;

use fastray::{
    camera::DefaultCamera,
    hit::{HitList, HitRecord},
    material, Camera, Hit, Material, Ray, Sphere,
};
use glam::Vec3;
use rand::{thread_rng, Rng};

fn color(ray: &Ray, world: &HitList, depth: i32, rng: &mut impl Rng) -> Vec3 {
    let mut hit_record = HitRecord::default();
    if world.hit(ray, 0.001..f32::MAX, &mut hit_record) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::default();
        if depth < 50
            && hit_record
                .material
                .scatter(ray, &hit_record, &mut attenuation, &mut scattered, rng)
        {
            attenuation * color(&scattered, world, depth + 1, rng)
        } else {
            Vec3::ZERO
        }
    } else {
        let direction = ray.direction.normalize();
        let t = 0.5 * (direction.y + 1.);
        Vec3::ONE.lerp(Vec3::new(0.5, 0.7, 1.), t)
        // Vec3::ZERO
    }
}

fn main() {
    let n_x = 800;
    let n_y = 400;
    let n_s = 1000;
    println!("P3");
    println!("{n_x} {n_y}");
    println!("255");

    let camera = DefaultCamera {
        bottom_left: Vec3::new(-2., -1., -1.),
        horizontal: Vec3::X * 4.,
        vertical: Vec3::Y * 2.,
        origin: Vec3::ZERO,
    };
    let world = HitList(vec![
        Box::new(Sphere {
            center: Vec3::new(0., 0., -1.),
            radius: 0.5,
            material: Arc::new(Material::Lambertian(material::Lambertian {
                albedo: Vec3::new(0.8, 0.3, 0.3),
            })),
        }),
        Box::new(Sphere {
            center: Vec3::new(0., -100.5, -1.),
            radius: 100.,
            material: Arc::new(Material::Lambertian(material::Lambertian {
                albedo: Vec3::new(0.8, 0.8, 0.),
            })),
        }),
        Box::new(Sphere {
            center: Vec3::new(1., 0., -1.),
            radius: 0.4,
            material: Arc::new(Material::Metal(material::Metal {
                albedo: Vec3::new(0.8, 0.6, 0.2),
                fuzz: 0.3,
            })),
        }),
        Box::new(Sphere {
            center: Vec3::new(-1., 0., -1.),
            radius: 0.45,
            material: Arc::new(Material::Metal(material::Metal {
                albedo: Vec3::new(0.8, 0.8, 0.8),
                fuzz: 1.,
            })),
        }),
    ]);
    let mut rng = thread_rng();

    for j in (0..n_y).rev() {
        for i in 0..n_x {
            let c = (0..n_s)
                .map(|_| {
                    let u = (i as f32 + rng.gen::<f32>()) / n_x as f32;
                    let v = (j as f32 + rng.gen::<f32>()) / n_y as f32;
                    let ray = camera.get_ray(u, v);
                    color(&ray, &world, 0, &mut rng)
                })
                .sum::<Vec3>()
                / n_s as f32;
            println!(
                "{} {} {}",
                (c[0] * 255.99) as u8,
                (c[1] * 255.99) as u8,
                (c[2] * 255.99) as u8
            );
        }
    }
}
