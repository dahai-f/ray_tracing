extern crate rand;
extern crate ray_tracing;

use rand::prelude::*;
use ray_tracing::*;
use std::f32;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let world = vec![
        Box::new(Sphere::new(
            &Vector3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(material::Lambertian::new(&Vector3::new(0.1, 0.2, 0.5))),
        )),
        Box::new(Sphere::new(
            &Vector3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(material::Lambertian::new(&Vector3::new(0.8, 0.8, 0.0))),
        )),
        Box::new(Sphere::new(
            &Vector3::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(material::Metal::new(&Vector3::new(0.8, 0.6, 0.2), 0.3)),
        )),
        Box::new(Sphere::new(
            &Vector3::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(material::Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            &Vector3::new(-1.0, 0.0, -1.0),
            -0.45,
            Box::new(material::Dielectric::new(1.5)),
        )),
    ];

    let camera = Camera::new(
        &Vector3::new(-2.0, 2.0, 1.0),
        &Vector3::new(0.0, 0.0, -1.0),
        &Vector3::new(0.0, 1.0, 0.0),
        90.0,
        nx as f32 / ny as f32,
    );

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + RNG.with(|rng| rng.borrow_mut().gen::<f32>())) / nx as f32;
                let v = (j as f32 + RNG.with(|rng| rng.borrow_mut().gen::<f32>())) / ny as f32;
                let r = camera.get_ray(u, v);
                color += &calc_color_by_ray(&r, &world, 0);
            }
            color /= ns as f32;
            color = Vector3::new(color.r().sqrt(), color.g().sqrt(), color.b().sqrt()); // square root to use gamma
            let ir = (255.99_f32 * color.r()) as i32;
            let ig = (255.99_f32 * color.g()) as i32;
            let ib = (255.99_f32 * color.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn calc_color_by_ray<T: Hittable>(r: &Ray, hit_list: &Vec<Box<T>>, depth: u32) -> Vector3 {
    // limit min to 0.001 to solve shadow acne problem
    let mut hit_record = HitRecord::default();
    if hit_list.hit(r, 0.001, f32::MAX, &mut hit_record) {
        let mut attenuation = Vector3::default();
        let mut scattered = Ray::default();
        if depth < 50 && hit_record.material.unwrap().scatter(
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
