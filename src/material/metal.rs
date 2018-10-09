use crate::*;

pub struct Metal {
    albedo: Vector3,
}

impl Metal {
    pub fn new(albedo: &Vector3) -> Metal {
        Metal { albedo: *albedo }
    }
}

impl Material for Metal {
    fn scatter<'a>(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord<'a>,
        attenuation: &mut Vector3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = self.albedo;
        *scattered = Ray::new(
            &hit_record.position,
            &ray_in.direction().reflect(&hit_record.normal),
        );
        scattered.direction().dot(&hit_record.normal) > 0.0
    }
}
