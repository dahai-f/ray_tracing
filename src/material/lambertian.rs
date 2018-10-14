use crate::*;

pub struct Lambertian {
    albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: &Vector3) -> Lambertian {
        Lambertian { albedo: *albedo }
    }
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
            &(&hit_record.normal + &Random::gen()).normalized(),
        );
        *attenuation = self.albedo;
        true
    }
}
