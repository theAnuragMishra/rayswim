use std::path::Path;

use crate::image::srgb_to_linear;
use image::RgbImage;

use crate::{math::vec3::Vec3, scene::texture::Texture};

pub struct ImageTexture {
    image: RgbImage,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let image = image::open(path).expect("failed to load image").to_rgb8();
        let (w, h) = image.dimensions();
        Self {
            image,
            width: w,
            height: h,
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

        let pixel = self
            .image
            .get_pixel(x.clamp(0, self.width - 1), y.clamp(0, self.height - 1));

        let r = srgb_to_linear(pixel[0] as f64 / 255.0);
        let g = srgb_to_linear(pixel[1] as f64 / 255.0);
        let b = srgb_to_linear(pixel[2] as f64 / 255.0);

        Vec3::new(r, g, b)
    }
}
