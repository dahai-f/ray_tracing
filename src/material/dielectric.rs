use crate::*;

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter<'a>(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord<'a>,
        attenuation: &mut Vector3,
        scattered: &mut Ray,
    ) -> bool {
        let (out_normal, ni_over_nt) = if hit_record.normal.dot(ray_in.direction()) > 0.0 {
            (-&hit_record.normal, self.ref_idx)
        } else {
            (hit_record.normal, 1_f32 / self.ref_idx)
        };
        *attenuation = Vector3::new(1.0, 1.0, 1.0);
        *scattered = Ray::new(
            &hit_record.position,
            &match ray_in.direction().refract(&out_normal, ni_over_nt) {
                Some(refracted) => refracted,
                None => ray_in.direction().reflect(&out_normal),
            },
        );
        true
    }
}
