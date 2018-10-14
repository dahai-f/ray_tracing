use crate::*;

pub struct Metal {
    albedo: Vector3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: &Vector3, fuzz: f32) -> Metal {
        Metal {
            albedo: *albedo,
            fuzz,
        }
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
            &(&ray_in.direction().reflect(&hit_record.normal)
                + &(self.fuzz * &Random::gen::<Vector3>()))
                .normalized(),
        );
        scattered.direction().dot(&hit_record.normal) > 0.0
    }
}
