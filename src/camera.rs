use glam::Vec3;

use crate::Ray;

pub trait Camera {
    fn get_ray(&self, u: f32, v: f32) -> Ray;
}

pub struct DefaultCamera {
    pub bottom_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera for DefaultCamera {
    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.bottom_left + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
