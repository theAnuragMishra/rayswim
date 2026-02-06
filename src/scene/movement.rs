use std::{f64::INFINITY, sync::Arc};

use crate::{
    math::{utils::degree_to_radians, vec3::Vec3},
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

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degree_to_radians(angle);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);
        let mut bbox = object.bounding_box();

        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1.0 - k as f64) * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    min.x = min.x.min(tester.x);
                    min.y = min.y.min(tester.y);
                    min.z = min.z.min(tester.z);
                    max.x = max.x.max(tester.x);
                    max.y = max.y.max(tester.y);
                    max.z = max.z.max(tester.z);
                }
            }
        }
        bbox = Aabb::from_extrema(min, max);
        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
    fn hit(
        &self,
        ray: &Ray,
        ray_t: crate::math::interval::Interval,
    ) -> Option<super::hittable::HitRecord> {
        let origin = Vec3::new(
            (self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z),
            ray.origin.y,
            self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z,
        );
        let direction = Vec3::new(
            (self.cos_theta * ray.direction.x) - (self.sin_theta * ray.direction.z),
            ray.direction.y,
            (self.sin_theta * ray.direction.x) + (self.cos_theta * ray.direction.z),
        );
        let rotated_ray = Ray::new_with_time(origin, direction, ray.time);

        if let Some(mut rec) = self.object.hit(&rotated_ray, ray_t) {
            rec.point = Vec3::new(
                self.cos_theta * rec.point.x + self.sin_theta * rec.point.z,
                rec.point.y,
                -self.sin_theta * rec.point.x + self.cos_theta * rec.point.z,
            );

            rec.normal = Vec3::new(
                self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
                rec.normal.y,
                -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
            );
            return Some(rec);
        }
        return None;
    }
}
