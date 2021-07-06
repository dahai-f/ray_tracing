use std::sync::Arc;

use crate::texture::Texture;
use crate::*;

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new<T: Texture + 'static, U: Into<Arc<T>>>(emit: U) -> DiffuseLight {
        DiffuseLight { emit: emit.into() }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<(Vector3, Ray)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, position: &Vector3) -> Vector3 {
        self.emit.value(u, v, position)
    }
}
