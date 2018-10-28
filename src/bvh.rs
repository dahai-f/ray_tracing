use crate::*;

pub struct BvhNode {
    aabb: AABB,
    left_child: Box<Hittable>,
    right_child: Box<Hittable>,
}

impl BvhNode {
    pub fn new(aabb: AABB, left_child: Box<Hittable>, right_child: Box<Hittable>) -> BvhNode {
        BvhNode {
            aabb,
            left_child,
            right_child,
        }
    }
}

impl Hittable for BvhNode {
    fn hit<'a, 'b: 'a>(
        &'b self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord<'a>,
    ) -> bool {
        if !self.aabb.hit(ray, t_min, t_max) {
            return false;
        }

        let mut left_record = HitRecord::default();
        let mut right_record = HitRecord::default();
        let left_hit = self.left_child.hit(ray, t_min, t_max, &mut left_record);
        let right_hit = self.left_child.hit(ray, t_min, t_max, &mut right_record);
        if left_hit && right_hit {
            *hit_record = if left_record.t <= right_record.t {
                left_record
            } else {
                right_record
            };
            return true;
        } else if left_hit {
            *hit_record = left_record;
            return true;
        } else if right_hit {
            *hit_record = right_record;
            return true;
        }

        false
    }

    fn bounding_box(&self, _t0: f32, _t1: f32, aabb: &mut AABB) -> bool {
        *aabb = self.aabb;
        true
    }
}
