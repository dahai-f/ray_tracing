use crate::*;
use rand::prelude::*;


pub struct Perlin {
    ran_float: [f32; 256],
    perm_x: [i32; 256],
    perm_y: [i32; 256],
    perm_z: [i32; 256],
}


impl Perlin {
    const fn new() -> Perlin {
        Perlin {
            ran_float: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }
}

const fn perlin_generate() -> [f32; 256] {
    Random::with_rng(|rng| {
        let mut ran_float = [0.0; 256];
        let a = ran_float;
        let b = ran_float;
        for i in 0..256 {
            ran_float[i] = rng.gen();
        }
        ran_float
    })
}

const fn permute(p: &mut [i32; 256]) {}

const fn perlin_generate_perm() -> [i32; 256] {
    let mut p = [0_i32; 256];
    for i in 0_i32..256 {
        p[i as usize] = i;
    }
    permute(&mut p);
    p
}
