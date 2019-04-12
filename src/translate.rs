use nalgebra::Vector3;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};
use crate::aabb::AABB;

pub struct Translate<H: Hitable> {
    hitable: H,
    offset: Vector3<f32>
}

impl<H: Hitable> Translate<H> {
    pub fn new(hitable: H, offset: Vector3<f32>) -> Self { Translate { hitable, offset } }
}

impl<H: Hitable> Hitable for Translate<H> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
        self.hitable.hit(&moved_ray, t_min, t_max).map(|mut hit| {
            hit.p += self.offset;
            hit
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hitable.bounding_box(t0, t1).map(|mut b| {
            b.min += self.offset;
            b.max += self.offset;
            b
        })
    }
}
