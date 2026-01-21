use std::sync::Arc;

use raytracer::geometry::sphere::Sphere;
use raytracer::math::vec3::Vec3;
use raytracer::scene::hittable_list::HittableList;

use raytracer::scene::material::dielectric::Dielectric;
use raytracer::{
    camera::Camera,
    scene::material::{lambertian::Lambertian, metal::Metal},
};

fn main() {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    let material_center = Arc::new(Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.0 / 1.5));

    let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.2),
        0.5,
        material_center.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble.clone(),
    )));

    let mut cam = Camera::new();
    cam.image_width = 400;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 90.0;
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.lookfrom = Vec3 {
        x: -2.0,
        y: 2.0,
        z: 1.0,
    };
    cam.lookat = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    cam.render(&world);
}
