use nalgebra::Vector3;

pub struct Ray {
    a: Vector3<f32>,
    b: Vector3<f32>,
    time: f32
}

impl Ray {
    pub fn new (a: Vector3<f32>, b: Vector3<f32>, time: f32) -> Self {
        Ray { a, b, time }
    }

    pub fn origin(&self) -> Vector3<f32> { self.a }
    pub fn direction(&self) -> Vector3<f32> { self.b }
    pub fn time(&self) -> f32 { self.time }
    pub fn point_at_parameter(&self, t: f32) -> Vector3<f32> { self.a + t * self.b }
}