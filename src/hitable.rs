use nalgebra::Vector3;
use crate::ray::Ray;
use crate::material::Material;
use crate::aabb;
use crate::aabb::AABB;

pub struct HitRecord<'a> {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub material: &'a Material
}

pub trait Hitable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

#[derive(Default)]
pub struct HitableList {
    list: Vec<Box<Hitable>>
}

impl HitableList {
    pub fn push(&mut self, hitable: impl Hitable + 'static) {
        self.list.push(Box::new(hitable))
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;
        for h in self.list.iter() {
            if let Some(hit) = h.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        match self.list.first() {
            Some(first) =>
                match first.bounding_box(t0, t1) {
                    Some(bbox) =>
                        self.list.iter().skip(1).try_fold(bbox, |acc, hitable|
                            match hitable.bounding_box(t0, t1) {
                                Some(bbox) => Some(aabb::surrounding_box(&acc, &bbox)),
                                _ => None
                            }
                        ),
                    _ => None
                },
            _ => None
        }
    }
}

pub struct FlipNormals<H: Hitable> {
    hitable: H
}

impl<H: Hitable> FlipNormals<H> {
    pub fn new(hitable: H) -> Self { FlipNormals { hitable } }
}

impl<H: Hitable> Hitable for FlipNormals<H> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hitable.hit(&ray, t_min, t_max).map(|mut hit| {
            hit.normal = -hit.normal;
            hit
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> { self.hitable.bounding_box(t0, t1) }
}