use std::cmp::Ordering;
use std::sync::Arc;

use crate::*;

pub struct BvhNode {
    aabb: AABB,
    left_child: Arc<dyn Hittable>,
    right_child: Arc<dyn Hittable>,
}

impl BvhNode {
    fn new(aabb: AABB, left_child: Arc<dyn Hittable>, right_child: Arc<dyn Hittable>) -> BvhNode {
        BvhNode {
            aabb,
            left_child,
            right_child,
        }
    }

    pub fn from_hit_list(hittable_list: &mut [Arc<dyn Hittable>]) -> BvhNode {
        if hittable_list.is_empty() {
            panic!("no hit list");
        }

        let axis = Random::gen::<usize>() % 3;
        hittable_list.sort_by(|a, b| {
            let a_box = a.bounding_box().unwrap();
            let b_box = b.bounding_box().unwrap();
            if a_box.min()[axis] < b_box.min()[axis] {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let (left_child, right_child): (Arc<dyn Hittable>, Arc<dyn Hittable>) = {
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
                        Arc::new(BvhNode::from_hit_list(&mut hittable_list[..mid]))
                    },
                    Arc::new(BvhNode::from_hit_list(&mut hittable_list[mid..])),
                )
            }
        };

        let left_box = left_child.bounding_box().unwrap();
        let right_box = right_child.bounding_box().unwrap();
        BvhNode::new(left_box.surrounding(&right_box), left_child, right_child)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None;
        }

        match (
            self.left_child.hit(ray, t_min, t_max),
            self.right_child.hit(ray, t_min, t_max),
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

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.aabb)
    }
}
