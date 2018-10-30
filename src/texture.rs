use crate::*;

pub trait Texture {
    fn value(&self, u: f32, v: f32, position: &Vector3) -> Vector3;
}

pub struct ConstantTexture {
    color: Vector3
}

impl ConstantTexture {
    pub fn new(r: f32, g: f32, b: f32) -> ConstantTexture {
        ConstantTexture {
            color: Vector3::new(r, g, b)
        }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, u: f32, v: f32, position: &Vector3) -> Vector3 {
        self.color
    }
}
