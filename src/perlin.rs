use crate::*;
use rand::prelude::*;

lazy_static! {
    pub static ref PERLIN: Perlin = Perlin::new();
}

pub struct Perlin {
    ran_float: [f32; 256],
    perm_x: [usize; 256],
    perm_y: [usize; 256],
    perm_z: [usize; 256],
}

impl Perlin {
    fn new() -> Perlin {
        Perlin {
            ran_float: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, position: &Vector3) -> f32 {
        let _u = position.x() - position.x().floor();
        let _v = position.y() - position.y().floor();
        let _w = position.z() - position.z().floor();
        let i = (position.x() * 4.0) as usize & 0xFF;
        let j = (position.y() * 4.0) as usize & 0xFF;
        let k = (position.z() * 4.0) as usize & 0xFF;
        self.ran_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}

fn perlin_generate() -> [f32; 256] {
    Random::with_rng(|rng| {
        let mut ran_float = [0.0; 256];
        for i in 0..256 {
            ran_float[i] = rng.gen::<f32>();
        }
        ran_float
    })
}

fn permute(p: &mut [usize; 256]) {
    Random::with_rng(|rng| {
        for i in (0..256).rev() {
            let target = rng.gen::<usize>() % (i + 1);
            if i != target {
                let temp = p[i];
                p[i] = p[target];
                p[target] = temp;
            }
        }
    });
}

fn perlin_generate_perm() -> [usize; 256] {
    let mut p = [0; 256];
    for i in 0..256 {
        p[i] = i;
    }
    permute(&mut p);
    p
}
