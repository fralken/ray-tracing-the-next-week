use nalgebra::Vector3;
use crate::perlin::Perlin;

pub trait Texture: Sync {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32>;
}

#[derive(Clone)]
pub struct ConstantTexture {
    color: Vector3<f32>
}

impl ConstantTexture {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        ConstantTexture {
            color: Vector3::new(r, g, b)
        }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: &Vector3<f32>) -> Vector3<f32> { self.color }
}

#[derive(Clone)]
pub struct CheckerTexture<T: Texture, U: Texture> {
    odd: T,
    even: U
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    #[allow(dead_code)]
    pub fn new(odd: T, even: U) -> Self { CheckerTexture { odd, even } }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        let sines = f32::sin(10.0 * p.x) * f32::sin(10.0 * p.y) * f32::sin(10.0 * p.z);
        if sines < 0.0 { self.odd.value(u, v, p) } else { self.even.value(u, v, p) }
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f32
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self { NoiseTexture { noise: Perlin::new(), scale }}
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        Vector3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + f32::sin(self.scale * p.x + 5.0 * self.noise.turb(&p, 7)))
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    data: Vec<u8>,
    nx: u32,
    ny: u32
}

impl ImageTexture {
    pub fn new(data: Vec<u8>, nx: u32, ny: u32) -> Self { ImageTexture { data, nx, ny } }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Vector3<f32>) -> Vector3<f32> {
        let nx = self.nx as usize;
        let ny = self.ny as usize;
        let mut i = (u * nx as f32) as usize;
        let mut j = ((1.0 - v) * ny as f32) as usize;
        if i > nx - 1 { i = nx - 1 }
        if j > ny - 1 { j = ny - 1 }
        let idx = 3 * i + 3 * nx * j;
        let r = self.data[idx] as f32 / 255.0;
        let g = self.data[idx + 1] as f32 / 255.0;
        let b = self.data[idx + 2] as f32 / 255.0;
        Vector3::new(r, g, b)
    }
}
