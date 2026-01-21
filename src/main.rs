use std::sync::Arc;

use raytracer::geometry::sphere::Sphere;
use raytracer::math::vec3::Vec3;
use raytracer::scene::hittable_list::HittableList;

use raytracer::scene::material::Material;
use raytracer::scene::material::dielectric::Dielectric;
use raytracer::{
    camera::Camera,
    scene::material::{lambertian::Lambertian, metal::Metal},
};

fn main() {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    });
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));

    for i in -11..11 {
        for j in -11..11 {
            let choose_material = rand::random_range(0.0..1.0);
            let center = Vec3::new(
                i as f64 + 0.9 * rand::random_range(0.0..1.0),
                0.2,
                j as f64 + 0.9 * rand::random_range(0.0..1.0),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;
                if choose_material < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    sphere_material = Arc::new(Lambertian { albedo });
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_material < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rand::random_range(0.0..0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    });
    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam = Camera::new();
    cam.image_width = 1200;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.lookfrom = Vec3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    cam.lookat = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    cam.render(&world);
}
