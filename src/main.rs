use std::sync::Arc;

use raytracer::geometry::sphere::Sphere;
use raytracer::math::vec3::Vec3;
use raytracer::scene::hittable_list::HittableList;

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
        albedo: Vec3::new(0.8, 0.3, 0.3),
    });

    let material_metal = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.0));
    let material_metal_fuzzy = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3));

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
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_metal.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_metal_fuzzy.clone(),
    )));

    let mut cam = Camera::new(400, 2.0);
    cam.render(&world);
}
