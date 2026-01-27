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

impl HitRecord {
    pub fn new(
        point: Vec3,
        outward_normal: Vec3,
        t: f64,
        material: Arc<dyn Material>,
        u: f64,
        v: f64,
        ray: &Ray,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            point,
            normal,
            t,
            front_face,
            material,
            u,
            v,
        }
    }
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

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}
