use std::sync::Arc;

use crate::math::interval::Interval;
use crate::math::vec3::Vec3;
use crate::ray::Ray;
use crate::scene::aabb::Aabb;
use crate::scene::material::Material;
use crate::scene::material::lambertian::Lambertian;

/// Stores information about a ray-object intersection
#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64, // distance along the ray
    pub front_face: bool,
    pub material: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
            material: Arc::new(Lambertian::from_color(Vec3::default())), // placeholder
            u: Default::default(),
            v: Default::default(),
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, outward_normal: Vec3, ray: &Ray) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> Aabb;
}
