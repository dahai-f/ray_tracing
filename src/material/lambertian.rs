use crate::texture::*;
use crate::*;

pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Lambertian<T> {
        Lambertian { albedo }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter<'a>(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord<'a>,
        attenuation: &mut Vector3,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(
            &hit_record.position,
            &(&hit_record.normal + &Random::gen()).normalized(),
            ray_in.time(),
        );
        *attenuation = self.albedo.value(0.0, 0.0, &hit_record.position);
        true
    }
}
