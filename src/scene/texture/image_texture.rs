use std::path::Path;

use crate::image::srgb_to_linear;

use crate::{math::vec3::Vec3, scene::texture::Texture};

pub struct ImageTexture {
    pixels: Vec<Vec3>,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let image = image::open(path).expect("failed to load image").to_rgb32f();
        let (w, h) = image.dimensions();
        Self {
            width: w,
            height: h,
            pixels: image
                .pixels()
                .map(|p| {
                    Vec3::new(
                        srgb_to_linear(p[0] as f64),
                        srgb_to_linear(p[1] as f64),
                        srgb_to_linear(p[2] as f64),
                    )
                })
                .collect(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _: &Vec3) -> Vec3 {
        if self.height <= 0 {
            return Vec3::new(0.0, 1.0, 1.0);
        }
        u = u.clamp(0.0, 1.0);
        v = 1.0 - v.clamp(0.0, 1.0);

        let x = (u * self.width as f64) as u32;
        let y = (v * self.height as f64) as u32;

        self.pixels
            [(x.clamp(0, self.width - 1) + self.width * y.clamp(0, self.height - 1)) as usize]
    }
}
