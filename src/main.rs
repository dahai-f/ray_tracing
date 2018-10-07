extern crate ray_tracing;

use ray_tracing::*;
use std::f32;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    let world = vec![
        Box::new(Sphere::new(&Vector3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(&Vector3::new(0.0, -100.5, -1.0), 100.0))
    ];

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(&origin, &(lower_left_corner + &horizontal * u + &vertical * v).normalized());
            let color = calc_color_by_ray(&r, &world);
            let ir = (255.99_f32 * color.r()) as i32;
            let ig = (255.99_f32 * color.g()) as i32;
            let ib = (255.99_f32 * color.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn calc_color_by_ray<T: Hittable>(r: &Ray, hit_list: &Vec<Box<T>>) -> Color {
    let mut hit_record = HitRecord::default();
    if hit_list.hit(r, 0.0, f32::MAX, &mut hit_record) {
        let n = &hit_record.normal;
        return Color::new(0.5 * (n.x() + 1.0), 0.5 * (n.y() + 1.0), 0.5 * (n.y() + 1.0));
    }
    let t = 0.5 * (r.direction().y() + 1.0);
    &((1.0 - t) * &Color::new(1.0, 1.0, 1.0)) + &(t * &Color::new(0.5, 0.7, 1.0))
}