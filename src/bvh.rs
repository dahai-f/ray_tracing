use crate::*;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BvhNode {
    aabb: AABB,
    left_child: Arc<Hittable>,
    right_child: Arc<Hittable>,
}

impl BvhNode {
    fn new(aabb: AABB, left_child: Arc<Hittable>, right_child: Arc<Hittable>) -> BvhNode {
        BvhNode {
            aabb,
            left_child,
            right_child,
        }
    }

    pub fn from_hit_list(hittable_list: &mut [Arc<Hittable>], t0: f32, t1: f32) -> BvhNode {
        if hittable_list.len() == 0 {
            panic!("no hit list");
        }

        let axis = Random::gen::<usize>() % 3;
        hittable_list.sort_by(|a, b| {
            let mut a_box = AABB::default();
            let mut b_box = AABB::default();
            if !a.bounding_box(t0, t1, &mut a_box) || !b.bounding_box(t0, t1, &mut b_box) {
                panic!("no bounding box");
            }
            if a_box.min()[axis] < b_box.min()[axis] {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let (left_child, right_child): (Arc<Hittable>, Arc<Hittable>) = {
            if hittable_list.len() == 1 {
                (hittable_list[0].clone(), hittable_list[0].clone())
            } else if hittable_list.len() == 2 {
                (hittable_list[0].clone(), hittable_list[1].clone())
            } else {
                let mid = hittable_list.len() / 2;
                (
                    if mid == 1 {
                        hittable_list[0].clone()
                    } else {
                        Arc::new(BvhNode::from_hit_list(&mut hittable_list[..mid], t0, t1))
                    },
                    Arc::new(BvhNode::from_hit_list(&mut hittable_list[mid..], t0, t1)),
                )
            }
        };

        let mut left_box = AABB::default();
        let mut right_box = AABB::default();
        if !left_child.bounding_box(t0, t1, &mut left_box)
            || !right_child.bounding_box(t0, t1, &mut right_box)
        {
            panic!("no bounding box");
        }

        BvhNode::new(left_box.surrounding(&right_box), left_child, right_child)
    }
}

unsafe impl Send for BvhNode {}

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
