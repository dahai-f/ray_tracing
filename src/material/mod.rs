pub use self::dielectric::Dielectric;
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

    fn emitted(&self, _u: f32, _v: f32, _position: &Vector3) -> Vector3 {
        Vector3::zero()
    }
}

mod dielectric;
mod diffuse_light;
mod lambertian;
mod metal;
