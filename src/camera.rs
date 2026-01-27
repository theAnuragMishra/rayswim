use std::io::{self, Write};

use rand;

use crate::{
    image::buffer::ImageBuffer,
    math::{interval::Interval, utils::degree_to_radians, vec3::Vec3},
    ray::Ray,
    scene::hittable::Hittable,
};

pub struct Camera {
    pub image_width: usize,
    pub aspect_ratio: f64,
    image_height: usize,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pub max_depth: i32,
    pub samples_per_pixel: i32,
    pixel_samples_scale: f64,
    pub vfov: f64,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    v: Vec3,
    u: Vec3,
    w: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Camera::default()
    }

    pub fn initialize(&mut self) {
        self.center = self.lookfrom;
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        let theta = degree_to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height: f64 = 2.0 * h * self.focus_dist;
        let viewport_width: f64 =
            viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (self.lookfrom - self.lookat).normalized();
        self.u = self.vup.cross(self.w).normalized();
        self.v = self.w.cross(self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
        let defocus_radius =
            self.focus_dist * f64::tan(degree_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn render(&mut self, world: &dyn Hittable) -> ImageBuffer {
        self.initialize();
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

        img
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i + offset.x) * self.pixel_delta_u)
            + ((j + offset.y) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new_with_time(ray_origin, ray_direction, rand::random_range(0.0..1.0))
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(
            rand::random_range(0.0..1.0) - 0.5,
            rand::random_range(0.0..1.0) - 0.5,
            0.0,
        )
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }

    pub fn color(&self, r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
        if depth <= 0 {
            return Vec3::default();
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
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

impl Default for Camera {
    fn default() -> Self {
        Self {
            image_width: 100,
            aspect_ratio: 1.0,
            image_height: Default::default(),
            center: Vec3::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
            max_depth: 10,
            samples_per_pixel: 10,
            pixel_samples_scale: Default::default(),
            vfov: 90.0,
            lookfrom: Vec3::default(),
            lookat: Vec3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            v: Default::default(),
            u: Default::default(),
            w: Default::default(),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Default::default(),
            defocus_disk_v: Default::default(),
        }
    }
}
