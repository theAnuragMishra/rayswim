use crate::math::vec3::Vec3;
use crate::ray::Ray;
use crate::scene::hittable::HitRecord;
use crate::scene::material::Material;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new_with_time(rec.point, scatter_direction, ray_in.time);
        *attenuation = self.albedo;
        true
    }
}
