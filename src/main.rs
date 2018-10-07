extern crate rand;
extern crate ray_tracing;

use rand::prelude::*;
use ray_tracing::*;
use std::cell::RefCell;
use std::f32;

thread_local!(static RNG: RefCell<ThreadRng> = RefCell::new(rand::thread_rng()));

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let world = vec![
        Box::new(Sphere::new(&Vector3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(&Vector3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    let camera = Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + RNG.with(|rng| rng.borrow_mut().gen::<f32>())) / nx as f32;
                let v = (j as f32 + RNG.with(|rng| rng.borrow_mut().gen::<f32>())) / ny as f32;
                let r = camera.get_ray(u, v);
                color += &calc_color_by_ray(&r, &world);
            }
            color /= ns as f32;
            color = Color::new(color.r().sqrt(), color.g().sqrt(), color.b().sqrt()); // square root to use gamma
            let ir = (255.99_f32 * color.r()) as i32;
            let ig = (255.99_f32 * color.g()) as i32;
            let ib = (255.99_f32 * color.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn calc_color_by_ray<T: Hittable>(r: &Ray, hit_list: &Vec<Box<T>>) -> Color {
    let mut hit_record = HitRecord::default();
    if hit_list.hit(r, 0.001, f32::MAX, &mut hit_record) {
        // limit min to 0.001 to solve shadow acne problem
        let target =
            hit_record.position + hit_record.normal + RNG.with(|rng| rng.borrow_mut().gen());
        return 0.5 * &calc_color_by_ray(
            &Ray::new(
                &hit_record.position,
                &(&target - &hit_record.position).normalized(),
            ),
            hit_list,
        );
    }
    let t = 0.5 * (r.direction().y() + 1.0);
    &((1.0 - t) * &Color::new(1.0, 1.0, 1.0)) + &(t * &Color::new(0.5, 0.7, 1.0))
}
