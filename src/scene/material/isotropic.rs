use std::sync::Arc;

use crate::{
    math::vec3::Vec3,
    ray::Ray,
    scene::{
        material::Material,
        texture::{Texture, solid::SolidColor},
    },
};

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn from_color(albedo: Vec3) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
    pub fn from_texture(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        rec: &crate::scene::hittable::HitRecord,
    ) -> Option<(Vec3, crate::ray::Ray)> {
        Some((
            self.tex.value(rec.u, rec.v, rec.point),
            Ray::new_with_time(rec.point, Vec3::random_unit_vector(), ray_in.time),
        ))
    }
}
