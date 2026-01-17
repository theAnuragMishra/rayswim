use crate::math::vec3::Vec3;
use crate::ray::ray::Ray;
use crate::scene::hittable::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant: f64 = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None; // no intersection
        }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root within t_min..t_max
        let mut root = (-b - sqrt_d) / (2.0 * a);
        if root < t_min || root > t_max {
            root = (-b + sqrt_d) / (2.0 * a);
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center).normalized();

        Some(HitRecord {
            point,
            normal,
            t: root,
        })
    }
}
