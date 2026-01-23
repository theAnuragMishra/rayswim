use crate::math::interval::Interval;
use crate::ray::Ray;
use crate::scene::hittable::{HitRecord, Hittable};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
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
}
