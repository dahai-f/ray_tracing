use crate::*;

pub struct MovingSphere {
    center0: Vector3,
    center1: Vector3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: Box<Material>,
}

impl MovingSphere {
    pub fn new(
        center0: &Vector3,
        center1: &Vector3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Box<Material>,
    ) -> MovingSphere {
        MovingSphere {
            center0: *center0,
            center1: *center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    fn center(&self, time: f32) -> Vector3 {
        let t = (time - self.time0) / (self.time1 - self.time0);
        (1.0 - t) * self.center0 + t * self.center1
    }
}

unsafe impl Sync for MovingSphere {}

unsafe impl Send for MovingSphere {}

impl Hittable for MovingSphere {
    fn hit<'a, 'b: 'a>(
        &'b self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord<'a>,
    ) -> bool {
        let center = self.center(ray.time());
        let co = ray.origin() - center;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(&co);
        let c = co.dot(&co) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return false;
        }

        let t = (|| {
            let t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t > t_min && t < t_max {
                return Some(t);
            }
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > t_min && t < t_max {
                return Some(t);
            }
            None
        })();
        match t {
            Some(t) => {
                hit_record.position = ray.point_at(t);
                hit_record.normal = &(&hit_record.position - &center) / self.radius;
                hit_record.t = t;
                hit_record.material = Some(&self.material);
                true
            }
            None => false,
        }
    }
}
