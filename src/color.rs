use super::vector3::*;

struct Color(Vec3);

impl Color {
    pub fn r(&self) -> f32 { self.0.x() }
    pub fn g(&self) -> f32 { self.0.y() }
    pub fn b(&self) -> f32 { self.0.z() }
}