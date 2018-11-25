use std::cmp::Ordering;

use crate::Ray;
use crate::*;

pub struct Metal {
    albedo: Vector3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: &Vector3, fuzz: f32) -> Metal {
        Metal {
            albedo: *albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vector3, Ray)> {
        let direction = &(&ray_in.direction().reflect(&hit_record.normal)
            + &(self.fuzz * &Random::gen::<Vector3>()))
            .normalized();
        match direction.dot(&hit_record.normal).partial_cmp(&0.0f32) {
            None => None,
            Some(ordering) => match ordering {
                Ordering::Greater => Some((
                    self.albedo,
                    Ray::new(&hit_record.position, direction, ray_in.time()),
                )),
                _ => None,
            },
        }
    }
}
