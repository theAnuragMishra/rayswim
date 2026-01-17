use crate::scene::hittable::{HitRecord, Hittable};
use std::sync::Arc;

pub struct Scene {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(obj);
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &crate::ray::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
