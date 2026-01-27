use std::sync::Arc;

use crate::math::vec3::Vec3;
use crate::ray::Ray;
use crate::scene::hittable::HitRecord;
use crate::scene::material::Material;
use crate::scene::texture::Texture;
use crate::scene::texture::solid::SolidColor;

pub struct Lambertian {
    pub tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn from_color(albedo: Vec3) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
    pub fn from_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal;
        }
        Some((
            self.tex.value(rec.u, rec.v, &rec.point),
            Ray::new_with_time(rec.point, scatter_direction, ray_in.time),
        ))
    }
}
