use std::ops::Deref;

use image::*;

use crate::*;

pub trait Texture: Sync + Send {
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

#[derive(Clone)]
pub struct NoiseTexture {
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> NoiseTexture {
        NoiseTexture { scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, position: &Vector3) -> Vector3 {
        use crate::perlin::PERLIN;
        //        Vector3::new(1.0, 1.0, 1.0) * ((PERLIN.turbulence(&(position * self.scale), 7) + 1.0) * 0.5)
        //        Vector3::new(1.0, 1.0, 1.0) * PERLIN.turbulence(&(position * self.scale), 7)
        Vector3::new(1.0, 1.0, 1.0)
            * (0.5
                * (1.0 + (self.scale * position.z() + 10.0 * PERLIN.turbulence(position, 7)).sin()))
    }
}

pub struct ImageTexture(DynamicImage);

impl ImageTexture {
    pub fn open(path: &str) -> ImageTexture {
        ImageTexture(image::open(path).unwrap())
    }
}

impl Deref for ImageTexture {
    type Target = DynamicImage;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _: &Vector3) -> Vector3 {
        let (width, height) = self.0.dimensions();
        let x = (u * width as f32) as i32;
        let y = ((1.0 - v) * height as f32 - 0.001) as i32;
        let x = (x.max(0) as u32).min(width - 1);
        let y = (y.max(0) as u32).min(height - 1);
        let pixel = self.get_pixel(x, y);
        Vector3::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        )
    }
}
