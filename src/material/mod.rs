use crate::*;

pub use self::dielectric::Dielectric;
pub use self::lambertian::Lambertian;
pub use self::metal::Metal;

pub trait Material {
    /// scatter a ray, and return (attenuation, scattered_ray)
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vector3, Ray)>;

    fn emitted(&self, _u: f32, _v: f32, _position: &Vector3) -> Vector3 {
        Vector3::zero()
    }
}

mod dielectric;
mod diffuse_light;
mod lambertian;
mod metal;
