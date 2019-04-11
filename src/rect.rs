use nalgebra::Vector3;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};
use crate::material::Material;
use crate::aabb::AABB;

pub enum Plane {
    YZ,
    ZX,
    XY
}

pub struct AARect<M: Material> {
    plane: Plane,
    a0: f32,
    a1: f32,
    b0: f32,
    b1: f32,
    k: f32,
    material: M
}

impl<M: Material> AARect<M> {
    pub fn new(plane: Plane, a0: f32, a1: f32, b0: f32, b1: f32, k: f32, material: M) -> Self {
        AARect { plane, a0, a1, b0, b1, k, material }
    }
}

impl<M: Material> Hitable for AARect<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (k_axis, a_axis, b_axis) = match &self.plane {
            Plane::YZ => (0, 1, 2),
            Plane::ZX => (1, 2, 0),
            Plane::XY => (2, 0, 1)
        };
        let t = (self.k - ray.origin()[k_axis]) / ray.direction()[k_axis];
        if t < t_min || t > t_max {
            None
        } else {
            let a = ray.origin()[a_axis] + t * ray.direction()[a_axis];
            let b = ray.origin()[b_axis] + t * ray.direction()[b_axis];
            if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
                None
            } else {
                let u = (a - self.a0) / (self.a1 - self.a0);
                let v = (b - self.b0) / (self.b1 - self.b0);
                let p = ray.point_at_parameter(t);
                let mut normal = Vector3::zeros();
                normal[k_axis] = 1.0;
                Some(HitRecord { t, u, v, p, normal, material: &self.material })
            }
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let min = Vector3::new(self.a0, self.b0, self.k - 0.0001);
        let max = Vector3::new(self.a1, self.b1, self.k + 0.0001);
        Some(AABB { min, max })
    }
}