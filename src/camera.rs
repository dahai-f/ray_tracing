use crate::*;
use std::f32;

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    _w: Vector3,
    lens_radius: f32,
    time0: f32,
    time1: f32,
}

impl Camera {
    pub fn new(
        look_from: &Vector3,
        look_at: &Vector3,
        view_up: &Vector3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        t0: f32,
        t1: f32,
    ) -> Camera {
        let theta = vfov * (f32::consts::PI / 180.0);
        let half_height = (theta / 2.0).tan() * focus_dist;
        let half_width = aspect * half_height;

        let w = (look_from - look_at).normalized();
        let u = view_up.cross(&w);
        let v = w.cross(&u);

        Camera {
            origin: *look_from,
            lower_left_corner: look_from - &w * focus_dist - &u * half_width - &v * half_height,
            horizontal: 2.0 * half_width * &u,
            vertical: 2.0 * half_height * &v,
            u,
            v,
            _w: w,
            lens_radius: aperture / 2.0,
            time0: t0,
            time1: t1,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let offset = self.lens_radius * Vector3::random_in_unit_disk();
        let origin = self.origin + offset.x() * self.u + offset.y() * self.v;
        let time = self.time0 + (self.time1 - self.time0) * Random::gen::<f32>();
        Ray::new(
            &origin,
            &(self.lower_left_corner + &self.horizontal * u + &self.vertical * v - &origin)
                .normalized(),
            time,
        )
    }
}
