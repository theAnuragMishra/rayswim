use crate::{math::vec3::Vec3, scene::texture::Texture};

#[derive(Clone, Copy, Debug)]
pub struct SolidColor {
    albedo: Vec3,
}

impl SolidColor {
    pub fn new(color: Vec3) -> Self {
        Self { albedo: color }
    }
    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            albedo: Vec3::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: &Vec3) -> Vec3 {
        self.albedo
    }
}
