use crate::*;

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r = r * r;
    r + (1.0 - r) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter<'a>(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord<'a>,
        attenuation: &mut Vector3,
        scattered: &mut Ray,
    ) -> bool {
        let dt = hit_record.normal.dot(ray_in.direction());
        let (out_normal, ni_over_nt) = if dt > 0.0 {
            (-&hit_record.normal, self.ref_idx)
        } else {
            (hit_record.normal, 1_f32 / self.ref_idx)
        };
        *attenuation = Vector3::new(1.0, 1.0, 1.0);
        *scattered = Ray::new(
            &hit_record.position,
            &match ray_in.direction().refract(&out_normal, ni_over_nt) {
                Some(refracted)
                    if RNG.with(|rng| rng.borrow_mut().gen::<f32>())
                        >= schlick(
                            if dt > 0.0 { self.ref_idx * dt } else { -dt },
                            self.ref_idx,
                        ) =>
                {
                    refracted
                }
                _ => ray_in.direction().reflect(&out_normal),
            },
        );
        true
    }
}
