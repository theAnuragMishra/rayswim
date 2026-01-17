mod geometry;
mod image;
mod math;
mod ray;
mod scene;

use geometry::sphere::Sphere;
use image::buffer::ImageBuffer;
use math::vec3::Vec3;
use ray::ray::Ray;
use scene::hittable::Hittable;
use scene::scene::Scene;

fn ray_color(ray: &Ray, scene: &Scene) -> Vec3 {
    if let Some(hit) = scene.hit(ray, 0.001, f64::INFINITY) {
        return (hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction = ray.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
fn main() {
    use std::sync::Arc;
    let mut scene = Scene::new();
    scene.add(Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    scene.add(Arc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))); // ground sphere
    let image_width = 400;
    let image_height = 200;
    let mut img = ImageBuffer::new(image_width, image_height);

    // Camera setup
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in 0..image_height {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = (image_height - 1 - j) as f64 / (image_height - 1) as f64;
            let direction = lower_left_corner + horizontal * u + vertical * v - origin;
            let r = Ray::new(origin, direction);
            let color = ray_color(&r, &scene);
            img.set_pixel(i, j, color);
        }
    }

    img.write_ppm("output.ppm");
    println!("Rendered output.ppm");
}

