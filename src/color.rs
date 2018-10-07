use super::vector3::*;
use std::ops::*;

pub struct Color(Vector3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(Vector3::new(r, g, b))
    }

    pub fn r(&self) -> f32 {
        self.0.x()
    }
    pub fn g(&self) -> f32 {
        self.0.y()
    }
    pub fn b(&self) -> f32 {
        self.0.z()
    }
}

impl Mul<&Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Color {
        Color(&rhs.0 * self)
    }
}

impl Add for &Color {
    type Output = Color;

    fn add(self, rhs: &Color) -> Color {
        Color(&self.0 + &rhs.0)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color(&self.0 + &rhs.0)
    }
}

impl AddAssign<&Color> for Color {
    fn add_assign(&mut self, rhs: &Color) {
        self.0 += &rhs.0
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs
    }
}
