use std::sync::Arc;

use crate::*;

pub struct FlipNormals {
    hittable_obj: Arc<dyn Hittable>,
}

impl FlipNormals {
    pub fn new<T: Hittable + 'static>(hittable_obj: impl Into<Arc<T>>) -> FlipNormals {
        FlipNormals {
            hittable_obj: hittable_obj.into(),
        }
    }
}

impl Hittable for FlipNormals {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hittable_obj
            .hit(ray, t_min, t_max)
            .map(|hit_record| HitRecord {
                normal: -hit_record.normal,
                ..hit_record
            })
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.hittable_obj.bounding_box()
    }
}
