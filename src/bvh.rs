use std::cmp::Ordering;
use std::sync::Arc;

use crate::*;

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
            let a_box = a.bounding_box(t0, t1).unwrap();
            let b_box = b.bounding_box(t0, t1).unwrap();
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

        let left_box = left_child.bounding_box(t0, t1).unwrap();
        let right_box = right_child.bounding_box(t0, t1).unwrap();
        BvhNode::new(left_box.surrounding(&right_box), left_child, right_child)
    }
}

unsafe impl Send for BvhNode {}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None;
        }

        match (
            self.left_child.hit(ray, t_min, t_max),
            self.left_child.hit(ray, t_min, t_max),
        ) {
            (Some(left_record), Some(right_record)) => {
                if left_record.t <= right_record.t {
                    Some(left_record)
                } else {
                    Some(right_record)
                }
            }
            (Some(left_record), None) => Some(left_record),
            (None, Some(right_record)) => Some(right_record),
            (None, None) => None,
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.aabb)
    }
}
