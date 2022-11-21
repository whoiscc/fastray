use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Instant,
};

use fastray::{camera::ThinLens, material, shape::BvhNode, Camera, Material, Ray, Shape, Sphere};
use glam::{vec3, Vec3};
use rand::{
    distributions::WeightedIndex, prelude::Distribution, rngs::StdRng, thread_rng, Rng, SeedableRng,
};
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

static N_RAY: AtomicU32 = AtomicU32::new(0);
fn color(ray: &Ray, world: &Shape, depth: i32, rng: &mut impl Rng) -> Vec3 {
    if depth == 0 {
        return Vec3::ZERO;
    }
    N_RAY.fetch_add(1, Ordering::Relaxed);

    let Some(hit_record) = world.hit(ray, 0.001..f32::MAX) else {
        let direction = ray.direction.normalize();
        let t = 0.5 * (direction.y + 1.);
        return Vec3::ONE.lerp(vec3(0.5, 0.7, 1.), t);
    };

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
    let image_width = 1280;
    let image_height = (image_width as f32 / aspect_ratio) as _;
    let pixel_sample = 512;
    let max_depth = 50;

    let camera = ThinLens::new(
        vec3(13., 2., 3.),
        Vec3::ZERO,
        Vec3::Y,
        0.349, // ~20deg
        aspect_ratio,
        0.1,
        10.,
    );
    let world = random_scene(&mut StdRng::seed_from_u64(0));

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    let start = Instant::now();
    let n_scanline = AtomicU32::new(0);
    let n_sample = AtomicU32::new(0);
    let report = || {
        let elapsed = Instant::now() - start;
        let n_ray = N_RAY.load(Ordering::Relaxed);
        let n_scanline = n_scanline.fetch_add(1, Ordering::Relaxed);
        eprint!(
            "\r[{:.2?}] Scanline: {}/{image_height}, {:.2}M rays/sec, Average depth: {:.2}{:12}",
            elapsed,
            n_scanline,
            n_ray as f32 / elapsed.as_secs_f32() / 1000. / 1000.,
            // no worry to divide 0 with floating point arith
            n_ray as f32 / n_sample.load(Ordering::Relaxed) as f32,
            ""
        );
    };
    (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            report();
            let mut rng = thread_rng();
            (0..image_width)
                .map(|i| {
                    let c = ((0..pixel_sample)
                        .map(|_| {
                            let u = (i as f32 + rng.gen::<f32>()) / image_width as f32;
                            let v = (j as f32 + rng.gen::<f32>()) / image_height as f32;
                            let ray = camera.get_ray(u, v, &mut rng);
                            // n_sample.fetch_add(1, Ordering::Relaxed);
                            color(&ray, &world, max_depth, &mut rng)
                        })
                        .sum::<Vec3>()
                        / pixel_sample as f32)
                        .clamp(Vec3::ZERO, Vec3::splat(0.999));
                    n_sample.fetch_add(pixel_sample, Ordering::Relaxed);
                    Vec3::new(c[0].sqrt(), c[1].sqrt(), c[2].sqrt())
                })
                .collect::<Vec<_>>()
        })
        // a little bit surprised to see rayon does not provide a way to avoid this collecting
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|scanline| {
            for color in scanline {
                println!(
                    "{} {} {}",
                    (color[0] * 256.) as u8,
                    (color[1] * 256.) as u8,
                    (color[2] * 256.) as u8
                );
            }
        });

    report();
    eprintln!();
    eprintln!("Done.");
}

fn random_scene(rng: &mut impl Rng) -> Shape {
    let mut world = vec![
        // ground
        Arc::new(Shape::Sphere(Sphere {
            center: vec3(0., -1000., 0.),
            radius: 1000.,
            material: Arc::new(Material::Lambertian(material::Lambertian {
                albedo: vec3(0.5, 0.5, 0.5),
            })),
        })),
        // main
        Arc::new(Shape::Sphere(Sphere {
            center: vec3(0., 1., 0.),
            radius: 1.,
            material: Arc::new(Material::Dielectric(material::Dielectric {
                refractive_index: 1.5,
            })),
        })),
        Arc::new(Shape::Sphere(Sphere {
            center: vec3(-4., 1., 0.),
            radius: 1.,
            material: Arc::new(Material::Lambertian(material::Lambertian {
                albedo: vec3(0.4, 0.2, 0.1),
            })),
        })),
        Arc::new(Shape::Sphere(Sphere {
            center: vec3(4., 1., 0.),
            radius: 1.,
            material: Arc::new(Material::Metal(material::Metal {
                albedo: vec3(0.7, 0.6, 0.5),
                fuzz: 0.,
            })),
        })),
    ];
    for a in -11..11 {
        for b in -11..11 {
            let center = vec3(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - vec3(4., 0.2, 0.)).length() <= 0.9 {
                continue;
            }

            let material = match WeightedIndex::new([80, 15, 5]).unwrap().sample(rng) {
                0 => Material::Lambertian(material::Lambertian {
                    albedo: vec3(rng.gen(), rng.gen(), rng.gen()),
                }),
                1 => Material::Metal(material::Metal {
                    albedo: vec3(
                        rng.gen_range(0.5..1.),
                        rng.gen_range(0.5..1.),
                        rng.gen_range(0.5..1.),
                    ),
                    fuzz: rng.gen_range(0. ..0.5),
                }),
                2 => Material::Dielectric(material::Dielectric {
                    refractive_index: 1.5,
                }),
                _ => unreachable!(),
            };
            world.push(Arc::new(Shape::Sphere(Sphere {
                center,
                radius: 0.2,
                material: Arc::new(material),
            })))
        }
    }
    Shape::Bvh(BvhNode::new(&mut world, rng))
}
