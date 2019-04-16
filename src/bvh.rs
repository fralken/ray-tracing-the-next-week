use std::cmp::Ordering;
use std::sync::Arc;
use rand::Rng;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};
use crate::aabb;
use crate::aabb::AABB;

pub struct BVHNode {
    left: Arc<Hitable>,
    right: Arc<Hitable>,
    bbox: AABB
}

impl BVHNode {
    pub fn new(hitable: &mut [Arc<Hitable>], time0: f32, time1: f32) -> Self {
        fn box_compare(time0: f32, time1: f32, axis: usize) -> impl FnMut(&Arc<Hitable>, &Arc<Hitable>) -> Ordering {
            move |a, b| {
                let a_bbox = a.bounding_box(time0, time1);
                let b_bbox = b.bounding_box(time0, time1);
                if a_bbox.is_none() || b_bbox.is_none() {
                    panic!["no bounding box in bvh node"]
                }
                if a_bbox.unwrap().min[axis] - b_bbox.unwrap().min[axis] < 0.0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }

        let axis = rand::thread_rng().gen_range(0, 3) as usize;

        hitable.sort_unstable_by(box_compare(time0, time1, axis));
        let len = hitable.len();
        let (left, right) = if len == 1 {
            (hitable[0].clone(), hitable[0].clone())
        } else if len == 2 {
            (hitable[0].clone(), hitable[1].clone())
        } else {
            (
                Arc::new(BVHNode::new(&mut hitable[0..len/2], time0, time1)) as Arc<Hitable>,
                Arc::new(BVHNode::new(&mut hitable[len/2..len], time0, time1)) as Arc<Hitable>
            )
        };
        let left_bbox = left.bounding_box(time0, time1);
        let right_bbox = right.bounding_box(time0, time1);
        if left_bbox.is_none() || right_bbox.is_none() {
            panic!["no bounding box in bvh node"]
        }

        BVHNode {
            left,
            right,
            bbox: aabb::surrounding_box(&left_bbox.unwrap(), &right_bbox.unwrap())
        }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(&ray, t_min, t_max) {
            let left = self.left.hit(&ray, t_min, t_max);
            let right = self.right.hit(&ray, t_min, t_max);
            match (left, right) {
                (Some(l), Some(r)) => if l.t < r.t { Some(l) } else { Some(r) },
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                _ => None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}