use std::sync::Arc;

use crate::math::interval::Interval;
use crate::ray::Ray;
use crate::scene::aabb::Aabb;
use crate::scene::hittable::{HitRecord, Hittable};

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            bbox: Aabb::default(),
        }
    }

    pub fn from_object(object: Arc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
            bbox: Aabb::default(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = Aabb::enclosing(self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest = ray_t.max;

        for obj in &self.objects {
            if let Some(hit) = obj.hit(r, Interval::new(ray_t.min, closest)) {
                closest = hit.t;
                closest_hit = Some(hit)
            }
        }
        closest_hit
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
