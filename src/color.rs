use super::vector3::*;

pub struct Color(Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(Vec3::new(r, g, b))
    }

    pub fn r(&self) -> f32 { self.0.x() }
    pub fn g(&self) -> f32 { self.0.y() }
    pub fn b(&self) -> f32 { self.0.z() }
}