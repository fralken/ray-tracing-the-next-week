use std::f32;
use nalgebra::Vector3;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};
use crate::aabb::AABB;

#[allow(dead_code)]
pub enum Axis {
    X,
    Y,
    Z
}

fn get_axis(axis: &Axis) -> (usize, usize, usize) {
    match axis {
        Axis::X => (0, 1, 2),
        Axis::Y => (1, 2, 0),
        Axis::Z => (2, 0, 1)
    }
}

pub struct Rotate<H: Hitable> {
    axis: Axis,
    sin_theta: f32,
    cos_theta: f32,
    hitable: H,
    bbox: Option<AABB>
}

impl<H: Hitable> Rotate<H> {
    pub fn new(axis: Axis, hitable: H, angle: f32) -> Self {
        let (r_axis, a_axis, b_axis) = get_axis(&axis);
        let radians = (f32::consts::PI / 180.0) * angle;
        let sin_theta = f32::sin(radians);
        let cos_theta = f32::cos(radians);
        let bbox = hitable.bounding_box(0.0, 1.0).map(|mut b| {
            let mut min = Vector3::new(f32::MAX, f32::MAX, f32::MAX);
            let mut max = Vector3::new(-f32::MAX, -f32::MAX, -f32::MAX);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let r = k as f32 * b.max[r_axis] + (1 - k) as f32 * b.min[r_axis];
                        let a = i as f32 * b.max[a_axis] + (1 - i) as f32 * b.min[a_axis];
                        let b = j as f32 * b.max[b_axis] + (1 - j) as f32 * b.min[b_axis];
                        let new_a = cos_theta * a - sin_theta * b;
                        let new_b = sin_theta * a + cos_theta * b;

                        if new_a < min[a_axis] { min[a_axis] = new_a }
                        if new_b < min[b_axis] { min[b_axis] = new_b }
                        if r < min[r_axis] { min[r_axis] = r }

                        if new_a > max[a_axis] { max[a_axis] = new_a }
                        if new_b > max[b_axis] { max[b_axis] = new_b }
                        if r > max[r_axis] { max[r_axis] = r }
                    }
                }
            }
            b.min = min;
            b.max = max;
            b
        });
        Rotate { axis, sin_theta, cos_theta, hitable, bbox }
    }
}

impl<H: Hitable> Hitable for Rotate<H> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (_, a_axis, b_axis) = get_axis(&self.axis);
        let mut origin = ray.origin();
        let mut direction = ray.direction();
        origin[a_axis] = self.cos_theta * ray.origin()[a_axis] + self.sin_theta * ray.origin()[b_axis];
        origin[b_axis] = -self.sin_theta * ray.origin()[a_axis] + self.cos_theta * ray.origin()[b_axis];
        direction[a_axis] = self.cos_theta * ray.direction()[a_axis] + self.sin_theta * ray.direction()[b_axis];
        direction[b_axis] = -self.sin_theta * ray.direction()[a_axis] + self.cos_theta * ray.direction()[b_axis];
        let rotated_ray = Ray::new(origin, direction, ray.time());
        self.hitable.hit(&rotated_ray, t_min, t_max).map(|mut hit| {
            let mut p = hit.p;
            let mut normal = hit.normal;
            p[a_axis] = self.cos_theta * hit.p[a_axis] - self.sin_theta * hit.p[b_axis];
            p[b_axis] = self.sin_theta * hit.p[a_axis] + self.cos_theta * hit.p[b_axis];
            normal[a_axis] = self.cos_theta * hit.normal[a_axis] - self.sin_theta * hit.normal[b_axis];
            normal[b_axis] = self.sin_theta * hit.normal[a_axis] + self.cos_theta * hit.normal[b_axis];
            hit.p = p;
            hit.normal = normal;
            hit
        })
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> { self.bbox.clone() }
}