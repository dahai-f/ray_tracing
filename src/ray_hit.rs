use crate::*;

#[derive(Copy, Clone, Default)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub position: Vector3,
    pub normal: Vector3,
    pub material: Option<&'a Box<Material>>,
    pub u: f32,
    pub v: f32,
}

pub trait Hittable: Sync + Send {
    fn hit<'a, 'b: 'a>(
        &'b self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord<'a>,
    ) -> bool;

    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut AABB) -> bool;
}

impl Hittable for &[Box<Hittable>] {
    fn hit<'a, 'b: 'a>(
        &'b self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord<'a>,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for hit_ele in *self {
            if hit_ele.hit(ray, t_min, closest_so_far, hit_record) {
                hit_anything = true;
                closest_so_far = hit_record.t;
            }
        }
        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut AABB) -> bool {
        if self.len() < 1 {
            return false;
        }

        let first_true = self[0].bounding_box(t0, t1, aabb);
        if !first_true {
            return false;
        }

        let mut temp_aabb = AABB::default();
        for i in 1..self.len() {
            if self[i].bounding_box(t0, t1, &mut temp_aabb) {
                aabb.surrounding(&temp_aabb);
            } else {
                return false;
            }
        }

        true
    }
}
