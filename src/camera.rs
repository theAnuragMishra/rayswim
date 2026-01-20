use std::io::{self, Write};

use rand;

use crate::{
    image::buffer::ImageBuffer,
    math::vec3::Vec3,
    ray::Ray,
    scene::hittable::{HitRecord, Hittable},
};

pub struct Camera {
    pub image_width: usize,
    pub aspect_ratio: f32,
    image_height: usize,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    max_depth: i32,
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new(image_width: usize, aspect_ratio: f32) -> Self {
        let center = Vec3::default();
        let mut image_height = (image_width as f32 / aspect_ratio) as usize;
        image_height = if image_height < 1 { 1 } else { image_height };

        let focal_length = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (image_width / image_height) as f64;

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let samples_per_pixel = 100;
        Self {
            image_width,
            aspect_ratio,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            max_depth: 10,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
        }
    }
    pub fn render(&mut self, world: &dyn Hittable) {
        let mut img = ImageBuffer::new(self.image_width, self.image_height);
        for j in 0..self.image_height {
            print!("\rScanlines remaining: {} ", self.image_height - j);
            io::stdout().flush().unwrap();
            for i in 0..self.image_width {
                let mut pixel_color = Vec3::default();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i as f64, j as f64);
                    pixel_color = pixel_color + self.color(&r, world, self.max_depth);
                }
                img.set_pixel(i, j, pixel_color * self.pixel_samples_scale);
            }
        }

        img.write_ppm("output.ppm");
        print!("\rRendered output.ppm!             \n");
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i + offset.x) * self.pixel_delta_u)
            + ((j + offset.y) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(
            rand::random_range(0.0..1.0) - 0.5,
            rand::random_range(0.0..1.0) - 0.5,
            0.0,
        )
    }

    pub fn color(&self, r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
        if depth <= 0 {
            return Vec3::default();
        }

        let mut rec = HitRecord::default();
        if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Vec3::default();
            if rec
                .material
                .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * self.color(&scattered, world, depth - 1);
            }
            return Vec3::default();
        }

        // background
        let unit = r.direction.normalized();
        let t = 0.5 * (unit.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}
