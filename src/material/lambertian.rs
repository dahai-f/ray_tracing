use std::sync::Arc;

use crate::texture::*;
use crate::*;

pub struct Lambertian {
    texture: Arc<Texture>,
}

impl Lambertian {
    pub fn new<T: Texture + 'static, U: Into<Arc<T>>>(texture: U) -> Lambertian {
        Lambertian {
            texture: texture.into(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vector3, Ray)> {
        Some((
            self.texture
                .value(hit_record.u, hit_record.v, &hit_record.position),
            Ray::new(
                &hit_record.position,
                &(&hit_record.normal + &Random::gen()).normalized(),
                ray_in.time(),
            ),
        ))
    }
}
