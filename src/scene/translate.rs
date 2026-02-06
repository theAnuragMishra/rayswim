use std::sync::Arc;

use crate::{
    math::vec3::Vec3,
    ray::Ray,
    scene::{aabb::Aabb, hittable::Hittable},
};

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t: crate::math::interval::Interval,
    ) -> Option<super::hittable::HitRecord> {
        let offset_ray = Ray::new_with_time(ray.origin - self.offset, ray.direction, ray.time);
        if let Some(mut rec) = self.object.hit(&offset_ray, ray_t) {
            rec.point = rec.point + self.offset;
            return Some(rec);
        }
        return None;
    }
}
