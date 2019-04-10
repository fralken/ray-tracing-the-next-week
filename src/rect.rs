use nalgebra::Vector3;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};
use crate::material::Material;
use crate::aabb::AABB;

pub struct XYRect<M: Material> {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: M
}

impl<M: Material> XYRect<M> {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: M) -> Self {
        XYRect { x0, x1, y0, y1, k, material }
    }
}

impl<M: Material> Hitable for XYRect<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin().z) / ray.direction().z;
        if t < t_min || t > t_max {
            None
        } else {
            let x = ray.origin().x + t * ray.direction().x;
            let y = ray.origin().y + t * ray.direction().y;
            if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
                None
            } else {
                let u = (x - self.x0) / (self.x1 - self.x0);
                let v = (y - self.y0) / (self.y1 - self.y0);
                let p = ray.point_at_parameter(t);
                let normal = Vector3::new(0.0, 0.0, 1.0);
                Some(HitRecord { t, u, v, p, normal, material: &self.material })
            }
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let min = Vector3::new(self.x0, self.y0, self.k - 0.0001);
        let max = Vector3::new(self.x1, self.y1, self.k + 0.0001);
        Some(AABB { min, max })
    }
}