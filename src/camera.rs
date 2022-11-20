use glam::{vec3, Vec3};
use rand::Rng;

use crate::Ray;

pub trait Camera {
    fn get_ray(&self, u: f32, v: f32, rng: &mut impl Rng) -> Ray;
}

pub struct ThinLens {
    pub lower_left_corner: Vec3,
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

impl ThinLens {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        theta: f32, // vertical field-of-view radians
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Self {
        let h = (theta / 2.).tan();
        let viewpoint_height = 2. * h;
        let viewpoint_width = aspect_ratio * viewpoint_height;

        let w = (look_from - look_at).normalize();
        let u = view_up.cross(w).normalize();
        let v = w.cross(u);
        let origin = look_from;
        let horizontal = focus_distance * viewpoint_width * u;
        let vertical = focus_distance * viewpoint_height * v;
        Self {
            origin,
            lower_left_corner: origin - horizontal / 2. - vertical / 2. - focus_distance * w,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.,
        }
    }
}

impl Camera for ThinLens {
    fn get_ray(&self, s: f32, t: f32, rng: &mut impl Rng) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}

fn random_in_unit_disk(rng: &mut impl Rng) -> Vec3 {
    let mut p;
    while {
        p = vec3(rng.gen_range(-1. ..1.), rng.gen_range(-1. ..1.), 0.);
        p.length_squared() >= 1.
    } {}
    p
}
