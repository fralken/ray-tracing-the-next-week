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

#[derive(Clone)]
pub struct Perlin {
    ran_float: Vec<f32>
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            ran_float: perlin_generate()
        }
    }

    pub fn noise(&self, p: &Vector3<f32>) -> f32 {
        let i = (4.0 * p.x) as usize & 7;
        let j = (4.0 * p.y) as usize & 7;
        let k = (4.0 * p.z) as usize & 7;
        self.ran_float[i ^ j ^ k]
    }
}