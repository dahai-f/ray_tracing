extern crate ray_tracing;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2_f32;
            let ir = (255.99_f32 * r) as i32;
            let ig = (255.99_f32 * g) as i32;
            let ib = (255.99_f32 * b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
