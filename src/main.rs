use std::sync::Arc;

use fastray::{
    camera::DefaultCamera,
    hit::{HitList, HitRecord},
    material, Camera, Hit, Material, Ray, Sphere,
};
use glam::{vec3, Vec3};
use rand::{thread_rng, Rng};

fn color(ray: &Ray, world: &HitList, depth: i32, rng: &mut impl Rng) -> Vec3 {
    if depth <= 0 {
        return Vec3::ZERO;
    }

    let mut hit_record = HitRecord::default();
    if !world.hit(ray, 0.001..f32::MAX, &mut hit_record) {
        let direction = ray.direction.normalize();
        let t = 0.5 * (direction.y + 1.);
        return Vec3::ONE.lerp(vec3(0.5, 0.7, 1.), t);
    }

    let mut scattered = Ray::default();
    let mut attenuation = Vec3::default();
    if hit_record
        .material
        .scatter(ray, &hit_record, &mut attenuation, &mut scattered, rng)
    {
        attenuation * color(&scattered, world, depth - 1, rng)
    } else {
        Vec3::ZERO
    }
}

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as _;
    let n_samples = 256;
    let max_depth = 50;

    let camera = DefaultCamera::default();
    let world = HitList(vec![
        Arc::new(Sphere {
            center: vec3(0., 0., -1.),
            radius: 0.5,
            material: Arc::new(Material::Lambertian(material::Lambertian {
                albedo: vec3(0.1, 0.2, 0.5),
            })),
        }),
        Arc::new(Sphere {
            center: vec3(0., -100.5, -1.),
            radius: 100.,
            material: Arc::new(Material::Lambertian(material::Lambertian {
                albedo: vec3(0.8, 0.8, 0.),
            })),
        }),
        Arc::new(Sphere {
            center: vec3(1., 0., -1.),
            radius: 0.4,
            material: Arc::new(Material::Metal(material::Metal {
                albedo: vec3(0.8, 0.6, 0.2),
                fuzz: 0.,
            })),
        }),
        Arc::new(Sphere {
            center: vec3(-1., 0., -1.),
            radius: 0.45,
            material: Arc::new(Material::Dielectric(material::Dielectric {
                refractive_index: 1.5,
            })),
        }),
    ]);
    let mut rng = thread_rng();

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..image_width {
            let mut c = ((0..n_samples)
                .map(|_| {
                    let u = (i as f32 + rng.gen::<f32>()) / image_width as f32;
                    let v = (j as f32 + rng.gen::<f32>()) / image_height as f32;
                    let ray = camera.get_ray(u, v);
                    color(&ray, &world, max_depth, &mut rng)
                })
                .sum::<Vec3>()
                / n_samples as f32)
                .clamp(Vec3::ZERO, Vec3::splat(0.999));
            c = Vec3::new(c[0].sqrt(), c[1].sqrt(), c[2].sqrt());
            println!(
                "{} {} {}",
                (c[0] * 256.) as u8,
                (c[1] * 256.) as u8,
                (c[2] * 256.) as u8
            );
        }
    }

    eprintln!();
    eprintln!("Done.");
}
