use crate::math::vec3::Vec3;
use crate::ray::ray::Ray;
use crate::scene::hittable::HitRecord;
use crate::scene::material::Material;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + Vec3::random_in_unit_sphere().normalized();
        *scattered = Ray::new(rec.point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
