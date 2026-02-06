use std::sync::Arc;

use crate::{
    math::{interval::Interval, vec3::Vec3},
    scene::{
        aabb::Aabb,
        hittable::{HitRecord, Hittable},
        material::Material,
    },
};

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    material: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let bbox_diag1 = Aabb::from_extrema(q, q + u + v);
        let bbox_diag2 = Aabb::from_extrema(q + u, q + v);
        let bbox = Aabb::enclosing(bbox_diag1, bbox_diag2);
        let n = u.cross(v);
        let normal = n.normalized();
        let d = normal.dot(q);
        let w = n / n.dot(n);
        Self {
            q,
            u,
            v,
            material,
            bbox,
            normal,
            d,
            w,
        }
    }

    pub fn is_interior(a: f64, b: f64) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }
        true
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t: crate::math::interval::Interval,
    ) -> Option<crate::scene::hittable::HitRecord> {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() < 1e-8 {
            return None;
        }
        let t = (self.d - self.normal.dot(ray.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }
        let intersection = ray.at(t);
        let planar_hitpoint_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hitpoint_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpoint_vector));
        if !Self::is_interior(alpha, beta) {
            return None;
        }

        Some(HitRecord::new(
            intersection,
            self.normal,
            t,
            self.material.clone(),
            alpha,
            beta,
            ray,
        ))
    }
}
