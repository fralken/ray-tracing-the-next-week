use nalgebra::Vector3;
use rand::Rng;

fn perlin_generate() -> Vec<f32> {
    let mut rng = rand::thread_rng();
    let mut p = Vec::with_capacity(256);
    for _ in 0..256 {
        p.push(rng.gen::<f32>());
    }
    p
}

fn permute(p: &mut [usize], n: usize) {
    let mut rng = rand::thread_rng();
    for i in (0..n as usize).rev() {
        let target = rng.gen_range(0, i+1);
        p.swap(i, target);
    }
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut p = Vec::with_capacity(256);
    for i in 0..256 {
        p.push(i);
    }
    permute(&mut p, 256);
    p
}

fn trilinear_interop(c: &[[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as f32 * u + (1 - i) as f32 * (1.0 - u)) *
                    (j as f32 * v + (1 - j) as f32 * (1.0 - v)) *
                    (k as f32 * w + (1 - k) as f32 * (1.0 - w)) * c[i][j][k];
            }
        }
    }
    accum
}

#[derive(Clone)]
pub struct Perlin {
    ran_float: Vec<f32>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            ran_float: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm()
        }
    }

    pub fn noise(&self, p: &Vector3<f32>) -> f32 {
        let u = p.x - f32::floor(p.x);
        let v = p.y - f32::floor(p.y);
        let w = p.z - f32::floor(p.z);
        let i = f32::floor(p.x) as usize;
        let j = f32::floor(p.y) as usize;
        let k = f32::floor(p.z) as usize;
        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] =
                        self.ran_float[self.perm_x[(i + di) & 255] ^ self.perm_y[(j + dj) & 255] ^ self.perm_z[(k + dk) & 255]]
                }
            }
        };
        trilinear_interop(&c, u, v, w)
    }
}