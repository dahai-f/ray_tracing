use super::Ray;
use super::Vector3;

#[derive(Default, Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub position: Vector3,
    pub normal: Vector3,
}

pub trait Hittable {
    fn hit<'a>(&self, ray: &'a Ray, t_min: f32, t_max: f32, hit_record: &'a mut HitRecord) -> bool;
}

impl<T> Hittable for Vec<Box<T>>
where
    T: Hittable,
{
    fn hit<'a>(&self, ray: &'a Ray, t_min: f32, t_max: f32, hit_record: &'a mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for hit_ele in self {
            if hit_ele.hit(ray, t_min, closest_so_far, hit_record) {
                closest_so_far = hit_record.t;
                hit_anything = true;
            }
        }
        hit_anything
    }
}
