use crate::texture::Texture;
use crate::*;

pub struct DiffuseLight {
    emit: Box<Texture>,
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<(Vector3, Ray)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, position: &Vector3) -> Vector3 {
        self.emit.value(u, v, position)
    }
}
