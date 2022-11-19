use glam::{vec3, Vec3};

use crate::Ray;

pub trait Camera {
    fn get_ray(&self, u: f32, v: f32) -> Ray;
}

pub struct DefaultCamera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Default for DefaultCamera {
    fn default() -> Self {
        let aspect_ratio = 16. / 9.;
        let viewpoint_height = 2.;
        let viewpoint_width = aspect_ratio * viewpoint_height;
        let focal_length = 1.;

        let origin = Vec3::ZERO;
        let horizontal = vec3(viewpoint_width, 0., 0.);
        let vertical = vec3(0., viewpoint_height, 0.);
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.
                - vertical / 2.
                - vec3(0., 0., focal_length),
        }
    }
}

impl Camera for DefaultCamera {
    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
