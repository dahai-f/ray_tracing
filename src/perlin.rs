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
        let i = position.x().floor();
        let j = position.y().floor();
        let k = position.z().floor();
        let u = position.x() - i;
        let v = position.y() - j;
        let w = position.z() - k;

        let (i, j, k) = (i as usize, j as usize, k as usize);
        let (u, v, w) = (
            u * u * (3.0 - 2.0 * u),
            v * v * (3.0 - 2.0 * v),
            w * w * (3.0 - 2.0 * w),
        );

        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ran_float[self.perm_x[i.wrapping_add(di) & 0xFF]
                        ^ self.perm_y[j.wrapping_add(dj) & 0xFF]
                        ^ self.perm_z[k.wrapping_add(dk) & 0xFF]];
                }
            }
        }
        tri_linear_interp(&c, u, v, w)
    }
}

fn tri_linear_interp(c: &[[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as f32 * u + (1 - i) as f32 * (1.0 - u))
                    * (j as f32 * v + (1 - j) as f32 * (1.0 - v))
                    * (k as f32 * w + (1 - k) as f32 * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }
    accum
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
