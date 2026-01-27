use crate::{
    math::vec3::Vec3,
    ray::Ray,
    scene::{hittable::HitRecord, material::Material},
};
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = r_in.direction.normalized().reflect(rec.normal);

        let scattered = Ray::new_with_time(
            rec.point,
            reflected + Vec3::random_unit_vector() * self.fuzz,
            r_in.time,
        );

        // only reflect if we’re not below the surface
        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
