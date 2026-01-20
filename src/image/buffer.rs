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

    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }

    pub fn write_ppm(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        write!(file, "P3\n{} {}\n255\n", self.width, self.height).unwrap();

        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.pixels[y * self.width + x];
                let r = Self::linear_to_gamma(color.x);
                let g = Self::linear_to_gamma(color.y);
                let b = Self::linear_to_gamma(color.z);

                let r_byte = (256.0 * r.clamp(0.0, 0.99)) as u8;
                let g_byte = (256.0 * g.clamp(0.0, 0.99)) as u8;
                let b_byte = (256.0 * b.clamp(0.0, 0.99)) as u8;
                write!(file, "{} {} {} ", r_byte, g_byte, b_byte).unwrap();
            }
            write!(file, "\n").unwrap();
        }
    }
}
