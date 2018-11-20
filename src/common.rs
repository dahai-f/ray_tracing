use std::f32;

use crate::*;

pub fn get_sphere_uv(p: &Vector3) -> (f32, f32) {
    let phi = f32::atan2(p.z(), p.x());
    let theta = p.y().asin();
    (
        1.0 - (phi + f32::consts::PI) / (2.0 * f32::consts::PI),
        (theta + f32::consts::FRAC_PI_2) / f32::consts::PI,
    )
}
