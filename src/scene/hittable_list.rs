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
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();

        let mut hit_anything = false;
        let mut closest = ray_t.max;

        for obj in &self.objects {
            if obj.hit(r, Interval::new(ray_t.min, closest), &mut temp_rec) {
                hit_anything = true;
                closest = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
