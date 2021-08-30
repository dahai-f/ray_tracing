use std::sync::Arc;

use crate::*;

pub struct Sphere {
    center: Vector3,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new<T: Material + 'static, U: Into<Arc<T>>>(
        center: &Vector3,
        radius: f32,
        material: U,
    ) -> Sphere {
        Sphere {
            center: *center,
            radius,
            material: material.into(),
        }
    }

    pub fn center(&self) -> &Vector3 {
        &self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let co = ray.origin() - self.center; // center to origin
        let a_of_equation = ray.direction().dot(ray.direction());
        let b_of_equation = 2.0 * ray.direction().dot(&co);
        let c_of_equation = co.dot(&co) - self.radius * self.radius;
        let discriminant = b_of_equation * b_of_equation - 4.0 * a_of_equation * c_of_equation;

        let t = {
            || -> Option<f32> {
                if discriminant < 0.0 {
                    return None;
                }
                let t = (-b_of_equation - discriminant.sqrt()) / (2.0 * a_of_equation);
                if t > t_min && t < t_max {
                    return Some(t);
                }
                let t = (-b_of_equation + discriminant.sqrt()) / (2.0 * a_of_equation);
                if t > t_min && t < t_max {
                    return Some(t);
                }
                None
            }()
        };

        match t {
            Some(t) => {
                let position = ray.point_at(t);
                let normal = &(position - self.center) / self.radius;
                let (u, v) = common::get_sphere_uv(&normal);
                Some(HitRecord {
                    t,
                    position,
                    normal,
                    material: self.material.clone(),
                    u,
                    v,
                })
            }
            None => None,
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        let half = Vector3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(self.center - half, self.center + half))
    }
}
