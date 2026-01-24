use std::sync::Arc;

use crate::math::interval::Interval;
use crate::math::vec3::Vec3;
use crate::ray::Ray;
use crate::scene::aabb::Aabb;
use crate::scene::hittable::{HitRecord, Hittable};
use crate::scene::material::Material;

pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub material: Arc<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(static_center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            center: Ray::new(static_center, Vec3::default()),
            radius: radius.max(0.0),
            material,
            bbox: Aabb::from_extrema(static_center - rvec, static_center + rvec),
        }
    }
    pub fn moving(center1: Vec3, center2: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        let center = Ray::new(center1, center2 - center1);
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::from_extrema(center.at(0.0) - rvec, center.at(0.0) + rvec);
        let box2 = Aabb::from_extrema(center.at(1.0) - rvec, center.at(1.0) + rvec);
        Self {
            center,
            radius,
            material,
            bbox: Aabb::enclosing(box1, box2),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center.at(ray.time);

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant: f64 = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return false; // no intersection
        }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root within t_min..t_max
        let mut root = (-b - sqrt_d) / (2.0 * a);
        if !ray_t.surrounds(root) {
            root = (-b + sqrt_d) / (2.0 * a);
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center.at(ray.time)) / self.radius;
        rec.set_face_normal(outward_normal, ray);
        rec.material = self.material.clone();
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
