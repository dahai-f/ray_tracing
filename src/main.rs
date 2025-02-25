extern crate rand;
extern crate ray_tracing;
extern crate thread_pool;

use std::sync::mpsc;
use std::sync::Arc;
use std::time;

use ray_tracing::bvh::BvhNode;
use ray_tracing::*;
use std::time::Instant;

fn main() {
    let nx = 1200;
    let ny = 800;
    let ns = 10;
    println!("P3\n{} {}\n255", nx, ny);

    let start_time = Instant::now();
    let (mut world, camera) = scenes::random(1200, 800);
    let camera = Arc::new(camera);

    let mut thread_pool = thread_pool::ThreadPool::new(12);
    let (color_sender, color_receiver) = mpsc::channel();
    let bvh_root = BvhNode::from_hit_list(&mut world);
    let bvh_root = Arc::new(bvh_root);

    let after_bvh_building = Instant::now();
    eprintln!(
        "bvh cost: {}s",
        (after_bvh_building - start_time).as_secs_f64()
    );

    for j in (0..ny).rev() {
        for i in 0..nx {
            for _ in 0..ns {
                let color_sender = color_sender.clone();
                let camera = camera.clone();
                let bvh_root = bvh_root.clone();
                thread_pool.execute(move || {
                    let u = (i as f32 + Random::gen::<f32>()) / nx as f32;
                    let v = (j as f32 + Random::gen::<f32>()) / ny as f32;
                    let r = camera.get_ray(u, v);
                    color_sender.send(render(&r, &bvh_root, 0)).unwrap();
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
        "ray tracing cost: {}s",
        (after_ray_tracing - after_bvh_building).as_secs_f64()
    );
    eprintln!(
        "total cost: {}s",
        (after_ray_tracing - start_time).as_secs_f64()
    );
}

fn render(r: &Ray, bvh_root: &BvhNode, depth: u32) -> Vector3 {
    // limit min to 0.001 to solve shadow acne problem
    if let Some(hit_record) = bvh_root.hit(r, 0.001, f32::MAX) {
        let emitted = hit_record
            .material
            .emitted(hit_record.u, hit_record.v, &hit_record.position);
        if depth < 50 {
            if let Some((attenuation, scattered)) = hit_record.material.scatter(r, &hit_record) {
                return emitted + &attenuation * &render(&scattered, bvh_root, depth + 1);
            }
        }
        emitted
    } else {
        // Vector3::zero()
        let t = 0.5 * (r.direction().y() + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}
