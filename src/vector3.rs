use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::*;

use rand::distributions::Standard;
use rand::prelude::*;

use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct Vector3([f32; 3]);

impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3([x, y, z])
    }
    pub const fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }
    pub const fn up() -> Vector3 {
        Vector3::new(0.0, 1.0, 0.0)
    }
    pub const fn forward() -> Vector3 {
        Vector3::new(0.0, 0.0, 1.0)
    }
    pub const fn one() -> Vector3 {
        Vector3::new(1.0, 1.0, 1.0)
    }

    pub fn random_in_unit_disk() -> Vector3 {
        loop {
            let result = 2.0 * &Random::with_rng(|rng| Vector3::new(rng.gen(), rng.gen(), 0.0))
                - Vector3::new(1.0, 1.0, 0.0);
            if result.squared_length() < 1.0 {
                return result;
            }
        }
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }
    pub fn y(&self) -> f32 {
        self.0[1]
    }
    pub fn z(&self) -> f32 {
        self.0[2]
    }
    pub fn r(&self) -> f32 {
        self.0[0]
    }
    pub fn g(&self) -> f32 {
        self.0[1]
    }
    pub fn b(&self) -> f32 {
        self.0[2]
    }

    pub fn min(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self.x().min(other.x()),
            self.y().min(other.y()),
            self.z().min(other.z()),
        )
    }

    pub fn max(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self.x().max(other.x()),
            self.y().max(other.y()),
            self.z().max(other.z()),
        )
    }

    pub fn length(&self) -> f32 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn distance(&self, other: &Vector3) -> f32 {
        (self - other).length()
    }

    pub fn normalize(&mut self) {
        let k = 1_f32 / self.length();
        *self *= k;
    }

    pub fn normalized(&self) -> Vector3 {
        let len = self.length();
        Vector3::new(self.x() / len, self.y() / len, self.z() / len)
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn reflect(&self, normal: &Vector3) -> Vector3 {
        self - &(normal * (self.dot(normal) * 2.0))
    }

    pub fn refract(&self, normal: &Vector3, ni_over_nt: f32) -> Option<Vector3> {
        let dt = self.dot(normal);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            Some(&(ni_over_nt * &(self - &(dt * normal))) - &(discriminant.sqrt() * normal))
        } else {
            None
        }
    }
}

unsafe impl std::marker::Send for Vector3 {}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Add for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Vector3 {
        Vector3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        &self + &rhs
    }
}

impl AddAssign<&Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.0[0] += rhs.x();
        self.0[1] += rhs.y();
        self.0[2] += rhs.z();
    }
}

impl Sub for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Vector3 {
        Vector3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Sub<Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        self - &rhs
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        &self - &rhs
    }
}

impl Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Vector3 {
        &self - rhs
    }
}

impl SubAssign<&Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: &Self) {
        self.0[0] -= rhs.x();
        self.0[1] -= rhs.y();
        self.0[2] -= rhs.z();
    }
}

impl Mul for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        Vector3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl MulAssign<&Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: &Vector3) {
        self.0[0] *= rhs.x();
        self.0[1] *= rhs.y();
        self.0[2] *= rhs.z();
    }
}

impl Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        Vector3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        &self * rhs
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
    }
}

impl Mul<&Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        rhs * self
    }
}

impl Div for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: &Vector3) -> Vector3 {
        Vector3::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
    }
}

impl DivAssign<&Vector3> for Vector3 {
    fn div_assign(&mut self, rhs: &Vector3) {
        self.0[0] /= rhs.x();
        self.0[1] /= rhs.y();
        self.0[2] /= rhs.z();
    }
}

impl Div<f32> for &Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Vector3 {
        Vector3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0[0] /= rhs;
        self.0[1] /= rhs;
        self.0[2] /= rhs;
    }
}

impl Neg for &Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &<Self as Index<usize>>::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.0[index]
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl Distribution<Vector3> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vector3 {
        loop {
            let result = &(2.0
                * &Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()))
                - &Vector3::new(1.0, 1.0, 1.0);
            if result.squared_length() < 1.0 {
                return result;
            }
        }
    }
}
