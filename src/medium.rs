use std::f32;
use nalgebra::Vector3;
use rand::Rng;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};
use crate::material::Isotropic;
use crate::texture::Texture;
use crate::aabb::AABB;

pub struct ConstantMedium<H: Hitable, T: Texture> {
    boundary: H,
    density: f32,
    phase_function: Isotropic<T>
}

impl<H: Hitable, T: Texture> ConstantMedium<H, T> {
    pub fn new(boundary: H, density: f32, texture: T) -> Self {
        ConstantMedium { boundary, density, phase_function: Isotropic::new(texture) }
    }
}

impl<H: Hitable, T: Texture> Hitable for ConstantMedium<H, T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();
        if let Some(mut hit1) = self.boundary.hit(&ray, -f32::MAX, f32::MAX) {
            if let Some(mut hit2) = self.boundary.hit(&ray, hit1.t + 0.0001, f32::MAX) {
                if hit1.t < t_min { hit1.t = t_min }
                if hit2.t > t_max { hit2.t = t_max }
                if hit1.t < hit2.t {
                    let distance_inside_boundary = (hit2.t - hit1.t) * ray.direction().norm();
                    let hit_distance = -(1.0 / self.density) * rng.gen::<f32>().ln();
                    if hit_distance < distance_inside_boundary {
                        let t = hit1.t + hit_distance / ray.direction().norm();
                        return Some(HitRecord {
                            t,
                            u: 0.0,
                            v: 0.0,
                            p: ray.point_at_parameter(t),
                            normal: Vector3::new(1.0, 0.0, 0.0), // arbitrary
                            material: &self.phase_function
                        })
                    }
                }
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
