use crate::ray::ray::Ray;
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            point: Default::default(),
            normal: Default::default(),
            t: 0.0,
            front_face: false,
        };

        let mut hit_anything = false;
        let mut closest = t_max;

        for obj in &self.objects {
            if obj.hit(r, t_min, closest, &mut temp_rec) {
                hit_anything = true;
                closest = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
