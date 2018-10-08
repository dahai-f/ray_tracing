extern crate rand;

pub use self::camera::Camera;
pub use self::color::Color;
pub use self::material::Material;
pub use self::ray::Ray;
pub use self::ray_hit::{HitRecord, Hittable};
pub use self::sphere::Sphere;
pub use self::vector3::Vector3;
use rand::prelude::*;
use std::cell::RefCell;

mod camera;
mod color;
mod material;
mod ray;
pub mod ray_hit;
mod sphere;
mod vector3;

thread_local!(pub static RNG: RefCell<ThreadRng> = RefCell::new(rand::thread_rng()));
