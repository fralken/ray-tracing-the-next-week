use std::f32;
use nalgebra::Vector3;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};
use crate::material::Material;
use crate::aabb;
use crate::aabb::AABB;

fn get_sphere_uv(p: &Vector3<f32>) -> (f32, f32) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + f32::consts::PI) / (2.0 * f32::consts::PI);
    let v = (theta + f32::consts::FRAC_PI_2) / f32::consts::PI;
    (u, v)
}

#[derive(Clone)]
pub struct Sphere<M: Material> {
    center: Vector3<f32>,
    radius: f32,
    material: M
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vector3<f32>, radius: f32, material: M) -> Self { Sphere {center, radius, material} }
}

impl<M: Material> Hitable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                return Some(HitRecord { t, u, v, p, normal, material: &self.material })
            }
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                return Some(HitRecord { t, u, v, p, normal, material: &self.material })
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let radius = Vector3::new(self.radius, self.radius, self. radius);
        let min = self.center - radius;
        let max = self.center + radius;
        Some(AABB { min, max })
    }
}

pub struct MovingSphere<M: Material> {
    center0: Vector3<f32>,
    center1: Vector3<f32>,
    time0: f32,
    time1: f32,
    radius: f32,
    material: M
}

impl<M: Material> MovingSphere<M> {
    pub fn new(center0: Vector3<f32>, center1: Vector3<f32>, time0: f32, time1: f32, radius: f32, material: M) -> Self {
        MovingSphere {center0, center1, time0, time1, radius, material}
    }

    pub fn center(&self, time: f32) -> Vector3<f32> {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl<M: Material> Hitable for MovingSphere<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let center = self.center(ray.time());
        let oc = ray.origin() - center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                let normal = (p - center) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                return Some(HitRecord { t, u, v, p, normal, material: &self.material })
            }
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                let normal = (p - center) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                return Some(HitRecord { t, u, v, p, normal, material: &self.material })
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let radius = Vector3::new(self.radius, self.radius, self. radius);
        let min0 = self.center(t0) - radius;
        let max0 = self.center(t0) + radius;
        let min1 = self.center(t1) - radius;
        let max1 = self.center(t0) + radius;
        let aabb0 = AABB::new(min0, max0);
        let aabb1 = AABB::new(min1, max1);
        Some(aabb::surrounding_box(&aabb0, &aabb1))
    }
}
