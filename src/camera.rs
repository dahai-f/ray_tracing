use crate::*;
use std::f32;

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(
        look_from: &Vector3,
        look_at: &Vector3,
        view_up: &Vector3,
        vfov: f32,
        aspect: f32,
    ) -> Camera {
        let theta = vfov * (f32::consts::PI / 180.0);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).normalized();
        let u = view_up.cross(&w);
        let v = w.cross(&u);

        Camera {
            origin: *look_from,
            lower_left_corner: look_from - &w - &u * half_width - &v * half_height,
            horizontal: 2.0 * half_width * &u,
            vertical: 2.0 * half_height * &v,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            &self.origin,
            &(self.lower_left_corner + &self.horizontal * u + &self.vertical * v - &self.origin)
                .normalized(),
        )
    }
}
