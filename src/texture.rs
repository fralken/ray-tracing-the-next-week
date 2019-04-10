use nalgebra::Vector3;
use crate::perlin::Perlin;

pub trait Texture {
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
        Vector3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + f32::sin(self.scale * p.z + 10.0 * self.noise.turb(&p, 7)))
    }
}
