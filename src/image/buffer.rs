use crate::math::vec3::Vec3;
use std::fs::File;
use std::io::Write;

pub struct ImageBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec3>, // store color as Vec3
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Vec3::new(0.0, 0.0, 0.0); width * height];
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    pub fn write_ppm(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        write!(file, "P3\n{} {}\n255\n", self.width, self.height).unwrap();

        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.pixels[y * self.width + x];
                let r = (255.99 * color.x) as u8;
                let g = (255.99 * color.y) as u8;
                let b = (255.99 * color.z) as u8;
                write!(file, "{} {} {} ", r, g, b).unwrap();
            }
            write!(file, "\n").unwrap();
        }
    }
}
