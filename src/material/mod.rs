pub use self::lambertian::Lambertian;
pub use self::metal::Metal;
use crate::*;

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vector3,
        scattered: &mut Ray,
    ) -> bool;
}

mod lambertian;
mod metal;
