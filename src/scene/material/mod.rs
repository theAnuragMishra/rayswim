pub mod lambertian;
pub mod metal;

use crate::math::vec3::Vec3;
use crate::ray::Ray;
use crate::scene::hittable::HitRecord;

pub trait Material: Sync + Send {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered_ray: &mut Ray,
    ) -> bool;
}
