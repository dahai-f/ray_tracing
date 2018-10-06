use super::Vector3;
use super::Ray;

pub fn hit_sphere(center: &Vector3, radius: f32, ray: &Ray) -> bool {
    let co = ray.origin() - center; // center to origin
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * ray.direction().dot(&co);
    let c = co.dot(&co) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}