use std::sync::Arc;

use crate::{
    math::vec3::Vec3,
    scene::{
        material::Material,
        texture::{Texture, solid::SolidColor},
    },
};

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
    pub fn color_source(emit: Vec3) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.tex.value(u, v, p)
    }
}
