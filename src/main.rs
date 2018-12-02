#![feature(duration_float)]

extern crate rand;
extern crate ray_tracing;
extern crate thread_pool;

use std::f32;
use std::sync::mpsc;
use std::sync::Arc;
use std::time;

use ray_tracing::*;

fn main() {
    let nx = 1200;
    let ny = 800;
    let ns = 10;
    println!("P3\n{} {}\n255", nx, ny);

    let start_time = time::Instant::now();
    let (world, camera) = scenes::simple_light(1200, 800);
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
                    color_sender.send(render(&r, &*world, 0)).unwrap();
                });
            }
            let mut color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                color += &color_receiver.recv().unwrap();
            }
            color /= ns as f32;
            color = Vector3::new(color.r().sqrt(), color.g().sqrt(), color.b().sqrt()); // square root to use gamma
            let ir = ((255.99_f32 * color.r()) as i32).min(255);
            let ig = ((255.99_f32 * color.g()) as i32).min(255);
            let ib = ((255.99_f32 * color.b()) as i32).min(255);
            println!("{} {} {}", ir, ig, ib);
        }
    }

    let after_ray_tracing = time::Instant::now();
    eprintln!(
        "ray tracing duration: {}s",
        (after_ray_tracing - start_time).as_float_secs()
    );
}

fn render(r: &Ray, hit_list: &[Box<Hittable>], depth: u32) -> Vector3 {
    // limit min to 0.001 to solve shadow acne problem
    if let Some(hit_record) = hit_list.hit(r, 0.001, f32::MAX) {
        let emitted = hit_record
            .material
            .emitted(hit_record.u, hit_record.v, &hit_record.position);
        if depth < 50 {
            if let Some((attenuation, scattered)) = hit_record.material.scatter(r, &hit_record) {
                return emitted + &attenuation * &render(&scattered, hit_list, depth + 1);
            }
        }
        emitted
    } else {
        Vector3::zero()
        //        let t = 0.5 * (r.direction().y() + 1.0);
        //        &((1.0 - t) * &Vector3::new(1.0, 1.0, 1.0)) + &(t * &Vector3::new(0.5, 0.7, 1.0))
    }
}
