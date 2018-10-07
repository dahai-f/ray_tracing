use super::*;

pub struct Sphere {
    center: Vector3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: &Vector3, radius: f32) -> Sphere {
        Sphere {
            center: *center,
            radius,
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
    fn hit<'a>(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &'a mut HitRecord) -> bool {
        let co = ray.origin() - &self.center; // center to origin
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * ray.direction().dot(&co);
        let c = co.dot(&co) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > t_min && t < t_max {
                hit_record.t = t;
                hit_record.position = ray.point_at(t);
                hit_record.normal = &(&hit_record.position - &self.center) / self.radius;
                return true;
            }

            let t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t > t_min && t < t_max {
                hit_record.t = t;
                hit_record.position = ray.point_at(t);
                hit_record.normal = &(&hit_record.position - &self.center) / self.radius;
                return true;
            }
        }
        false
    }
}
