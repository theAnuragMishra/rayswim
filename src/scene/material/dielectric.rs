use crate::{math::vec3::Vec3, ray::Ray, scene::material::Material};

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        Self { refractive_index }
    }
    fn reflectance(cosine: f64, ri: f64) -> f64 {
        let mut r0 = (1.0 - ri) / (1.0 + ri);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        rec: &crate::scene::hittable::HitRecord,
    ) -> Option<(Vec3, Ray)> {
        let ri = if rec.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction = ray_in.direction.normalized();
        let cos_theta = f64::min((-unit_direction).dot(rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let direction;
        if ri * sin_theta > 1.0 || (Self::reflectance(cos_theta, ri) > rand::random_range(0.0..1.0))
        {
            direction = unit_direction.reflect(rec.normal);
        } else {
            direction = unit_direction.refract(rec.normal, ri);
        }
        Some((
            Vec3::new(1.0, 1.0, 1.0),
            Ray::new_with_time(rec.point, direction, ray_in.time),
        ))
    }
}
