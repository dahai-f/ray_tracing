use std::sync::Arc;

use crate::*;

pub struct HitRecord {
    pub t: f32,
    pub position: Vector3,
    pub normal: Vector3,
    pub material: Arc<dyn Material>,
    pub u: f32,
    pub v: f32,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    fn bounding_box(&self) -> Option<AABB>;
}

impl Hittable for &[Arc<dyn Hittable>] {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = t_max;
        for hit_ele in *self {
            if let Some(hit_record) = hit_ele.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                result = Some(hit_record);
            }
        }
        result
    }

    fn bounding_box(&self) -> Option<AABB> {
        if self.is_empty() {
            return None;
        }

        self[0].bounding_box().and_then(|mut result| {
            for i in 1..self.len() {
                match self[i].bounding_box() {
                    Some(aabb) => {
                        result = result.surrounding(&aabb);
                    }
                    None => {
                        return None;
                    }
                }
            }
            Some(result)
        })
    }
}
