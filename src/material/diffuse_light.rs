use crate::texture::Texture;
use crate::*;

pub struct DiffuseLight {
    emit: Box<Texture>,
}

impl Material for DiffuseLight {
    fn scatter<'a>(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord<'a>,
        _attenuation: &mut Vector3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f32, v: f32, position: &Vector3) -> Vector3 {
        self.emit.value(u, v, position)
    }
}
