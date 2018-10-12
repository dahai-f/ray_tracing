use crate::*;
use std::f32;

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * (f32::consts::PI / 180.0);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        Camera {
            origin: Vector3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vector3::new(-half_width, -half_height, -1.0),
            horizontal: Vector3::new(2.0 * half_width, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0 * half_height, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            &self.origin,
            &(self.lower_left_corner + &self.horizontal * u + &self.vertical * v),
        )
    }
}
