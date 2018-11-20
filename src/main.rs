#![feature(duration_float)]

extern crate rand;
extern crate ray_tracing;
extern crate thread_pool;

use ray_tracing::*;
use std::f32;
use std::sync::mpsc;
use std::sync::Arc;
use std::time;

fn main() {
    let nx = 1200;
    let ny = 800;
    let ns = 10;
    println!("P3\n{} {}\n255", nx, ny);

    let start_time = time::Instant::now();
    //    let world = Arc::new(scenes::two_perlin_sphere());
    //
    //    let look_from = Vector3::new(13.0, 2.0, 3.0);
    //    let look_at = Vector3::new(0.0, 0.0, 0.0);
    //    let focus_dist = 10.0;
    //    let aperture = 0.0;

    //    let camera = Arc::new(Camera::new(
    //        &look_from,
    //        &look_at,
    //        &Vector3::new(0.0, 1.0, 0.0),
    //        20.0,
    //        nx as f32 / ny as f32,
    //        aperture,
    //        focus_dist,
    //        0.0,
    //        1.0,
    //    ));

    let (world, camera) = scenes::earth_other_half();
    let (world, camera) = (Arc::new(world), Arc::new(camera));

    let thread_pool = thread_pool::ThreadPool::new(12);
    let (color_sender, color_receiver) = mpsc::channel();
    for j in (0..ny).rev() {
        for i in 0..nx {
            for _ in 0..ns {
                let world = world.clone();
                let camera = camera.clone();
                let color_sender = color_sender.clone();
                thread_pool.execute(move || {
                    let u = (i as f32 + Random::gen::<f32>()) / nx as f32;
                    let v = (j as f32 + Random::gen::<f32>()) / ny as f32;
                    let r = camera.get_ray(u, v);
                    color_sender
                        .send(calc_color_by_ray(&r, &*world, 0))
                        .unwrap();
                });
            }
            let mut color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                color += &color_receiver.recv().unwrap();
            }
            color /= ns as f32;
            color = Vector3::new(color.r().sqrt(), color.g().sqrt(), color.b().sqrt()); // square root to use gamma
            let ir = (255.99_f32 * color.r()) as i32;
            let ig = (255.99_f32 * color.g()) as i32;
            let ib = (255.99_f32 * color.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }

    let after_ray_tracing = time::Instant::now();
    eprintln!(
        "ray tracing duration: {}s",
        (after_ray_tracing - start_time).as_float_secs()
    );
}

fn calc_color_by_ray(r: &Ray, hit_list: &[Box<Hittable>], depth: u32) -> Vector3 {
    // limit min to 0.001 to solve shadow acne problem
    let mut hit_record = HitRecord::default();
    if hit_list.hit(r, 0.001, f32::MAX, &mut hit_record) {
        let mut attenuation = Vector3::default();
        let mut scattered = Ray::default();
        if depth < 50
            && hit_record.material.unwrap().scatter(
                r,
                &hit_record,
                &mut attenuation,
                &mut scattered,
            ) {
            return &attenuation * &calc_color_by_ray(&scattered, hit_list, depth + 1);
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    } else {
        let t = 0.5 * (r.direction().y() + 1.0);
        &((1.0 - t) * &Vector3::new(1.0, 1.0, 1.0)) + &(t * &Vector3::new(0.5, 0.7, 1.0))
    }
}
