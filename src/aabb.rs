use std::f32;
use nalgebra::Vector3;
use crate::ray::Ray;

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let min = Vector3::new(
        f32::min(box0.min.x, box1.min.x),
        f32::min(box0.min.y, box1.min.y),
        f32::min(box0.min.z, box1.min.z));
    let max = Vector3::new(
        f32::max(box0.max.x, box1.max.x),
        f32::max(box0.max.y, box1.max.y),
        f32::max(box0.max.z, box1.max.z));
    AABB { min, max }
}

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>
}

impl AABB {
    pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Self { AABB { min, max } }

    pub fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction()[a];
            let t0 = (self.min[a] - ray.origin()[a]) * inv_d;
            let t1 = (self.max[a] - ray.origin()[a]) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false
            }
        }
        true
    }
}