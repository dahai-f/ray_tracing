use crate::*;

pub trait Texture {
    fn value(&self, u: f32, v: f32, position: &Vector3) -> Vector3;
}

#[derive(Clone)]
pub struct ConstantTexture {
    color: Vector3,
}

impl ConstantTexture {
    pub fn new(r: f32, g: f32, b: f32) -> ConstantTexture {
        ConstantTexture {
            color: Vector3::new(r, g, b),
        }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _position: &Vector3) -> Vector3 {
        self.color
    }
}

#[derive(Clone)]
pub struct CheckerTexture<T: Texture, U: Texture> {
    odd: T,
    even: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    pub fn new(odd: T, even: U) -> CheckerTexture<T, U> {
        CheckerTexture { odd, even }
    }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn value(&self, u: f32, v: f32, position: &Vector3) -> Vector3 {
        let sign =
            (10.0 * position.x()).sin() * (10.0 * position.y()).sin() * (10.0 * position.z()).sin();
        if sign < 0.0 {
            self.odd.value(u, v, position)
        } else {
            self.even.value(u, v, position)
        }
    }
}
