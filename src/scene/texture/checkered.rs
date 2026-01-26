use std::sync::Arc;

use crate::{
    math::vec3::Vec3,
    scene::texture::{Texture, solid::SolidColor},
};

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, color1: Vec3, color2: Vec3) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(color1)),
            odd: Arc::new(SolidColor::new(color2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Vec3 {
        let x_int = (self.inv_scale * point.x).floor();
        let y_int = (self.inv_scale * point.y).floor();
        let z_int = (self.inv_scale * point.z).floor();

        let is_even = (x_int + y_int + z_int) % 2.0 == 0.0;

        if is_even {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}
