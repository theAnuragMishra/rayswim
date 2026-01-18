mod geometry;
mod image;
mod math;
mod ray;
mod scene;

use std::sync::Arc;

use geometry::sphere::Sphere;
use image::buffer::ImageBuffer;
use math::vec3::Vec3;
use rand::Rng;
use ray::ray::Ray;
use scene::hittable_list::HittableList;

use crate::scene::material::lambertian::Lambertian;

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * v.dot(n)
}

fn main() {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    let material_center = Arc::new(Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    let image_width = 400;
    let image_height = 200;
    let mut img = ImageBuffer::new(image_width, image_height);

    // Camera setup
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let mut rng = rand::rng();

    for j in 0..image_height {
        for i in 0..image_width {
            let samples_per_pixel = 50;
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.random_range(0.0..1.0)) / (image_width - 1) as f64;
                let v = ((image_height - 1 - j) as f64 + rng.random_range(0.0..1.0))
                    / (image_height - 1) as f64;
                let direction = lower_left_corner + horizontal * u + vertical * v - origin;
                let r = Ray::new(origin, direction);
                let max_depth = 50; // max recursive bounces
                let color = crate::ray::ray::color(&r, &world, max_depth);
                pixel_color = pixel_color + color;
            }
            // average and gamma correct
            let scale = 1.0 / samples_per_pixel as f64;
            let r = (pixel_color.x * scale).sqrt();
            let g = (pixel_color.y * scale).sqrt();
            let b = (pixel_color.z * scale).sqrt();

            img.set_pixel(i, j, Vec3::new(r, g, b));
        }
    }

    img.write_ppm("output.ppm");
    println!("Rendered output.ppm");
}
