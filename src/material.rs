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

pub struct Lambertian {
    albedo: Vector3,
}

impl Material for Lambertian {
    fn scatter<'a>(
        &self,
        _: &Ray,
        hit_record: &HitRecord<'a>,
        attenuation: &mut Vector3,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(
            &hit_record.position,
            &(&hit_record.normal + &RNG.with(|rng| rng.borrow_mut().gen())).normalized(),
        );
        *attenuation = self.albedo;
        true
    }
}
