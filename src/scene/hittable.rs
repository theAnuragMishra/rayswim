use crate::math::vec3::Vec3;
use crate::ray::ray::Ray;

/// Stores information about a ray-object intersection
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64, // distance along the ray
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
