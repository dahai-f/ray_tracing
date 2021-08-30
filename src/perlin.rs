use rand::prelude::*;

use crate::*;

lazy_static! {
    pub static ref PERLIN: Perlin = Perlin::new();
}

pub struct Perlin {
    ran_float: [Vector3; 256],
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
        let x_floor = position.x().floor();
        let y_floor = position.y().floor();
        let z_floor = position.z().floor();
        let x_fraction = position.x() - x_floor;
        let y_fraction = position.y() - y_floor;
        let z_fraction = position.z() - z_floor;

        let (i, j, k) = (x_floor as usize, y_floor as usize, z_floor as usize);

        let mut c = [[[Vector3::zero(); 2]; 2]; 2];
        for (di, c) in c.iter_mut().enumerate() {
            for (dj, c) in c.iter_mut().enumerate() {
                for (dk, c) in c.iter_mut().enumerate() {
                    *c = self.ran_float[self.perm_x[i.wrapping_add(di) & 0xFF]
                        ^ self.perm_y[j.wrapping_add(dj) & 0xFF]
                        ^ self.perm_z[k.wrapping_add(dk) & 0xFF]];
                }
            }
        }
        perlin_interp(&c, x_fraction, y_fraction, z_fraction)
    }

    pub fn turbulence(&self, position: &Vector3, depth: i32) -> f32 {
        let mut accum = 0.0;
        let mut p = *position;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(&p);
            weight *= 0.5;
            p *= 2.0;
        }
        accum.abs()
    }
}

fn perlin_interp(c: &[[[Vector3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let (uu, vv, ww) = (
        u * u * (3.0 - 2.0 * u),
        v * v * (3.0 - 2.0 * v),
        w * w * (3.0 - 2.0 * w),
    );
    let mut accum = 0.0;
    for (i, c) in c.iter().enumerate() {
        for (j, c) in c.iter().enumerate() {
            for (k, c) in c.iter().enumerate() {
                let weight_v = Vector3::new(u - i as f32, v - j as f32, w - k as f32);
                accum += (i as f32 * uu + (1 - i) as f32 * (1.0 - uu))
                    * (j as f32 * vv + (1 - j) as f32 * (1.0 - vv))
                    * (k as f32 * ww + (1 - k) as f32 * (1.0 - ww))
                    * c.dot(&weight_v);
            }
        }
    }
    accum
}

fn perlin_generate() -> [Vector3; 256] {
    Random::with_rng(|rng| {
        let mut result = [Vector3::zero(); 256];
        for i in result.iter_mut() {
            *i = rng.gen::<Vector3>().normalized();
        }
        result
    })
}

fn permute(p: &mut [usize; 256]) {
    Random::with_rng(|rng| {
        for i in (0..256).rev() {
            let target = rng.gen::<usize>() % (i + 1);
            if i != target {
                p.swap(i, target);
            }
        }
    });
}

fn perlin_generate_perm() -> [usize; 256] {
    let mut p = [0; 256];
    for (i, x) in p.iter_mut().enumerate() {
        *x = i;
    }
    permute(&mut p);
    p
}
