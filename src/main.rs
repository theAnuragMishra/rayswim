use std::sync::Arc;

use rand::random_range;
use raytracer::geometry::quad::{Quad, dabba};
use raytracer::geometry::sphere::Sphere;
use raytracer::image::buffer::ImageBuffer;
use raytracer::math::vec3::Vec3;
use raytracer::scene::bvh::BvhNode;
use raytracer::scene::constant_medium::ConstantMedium;
use raytracer::scene::hittable::Hittable;
use raytracer::scene::hittable_list::HittableList;

use raytracer::scene::material::Material;
use raytracer::scene::material::dielectric::Dielectric;
use raytracer::scene::material::diffuse_light::DiffuseLight;
use raytracer::scene::movement::{RotateY, Translate};
use raytracer::scene::texture::checkered::CheckerTexture;
use raytracer::scene::texture::image_texture::ImageTexture;
use raytracer::scene::texture::perlin::NoiseTexture;
use raytracer::{
    camera::Camera,
    scene::material::{lambertian::Lambertian, metal::Metal},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let scene_name = args.get(1).map(|x| x.as_str()).unwrap_or("output");

    let img = cornell_smoke();
    let path = format!("images/{}.ppm", scene_name);
    img.write_ppm(path);
    print!("\rRendered {}.ppm!                        \n", scene_name);
}

fn bouncing_spheres() -> ImageBuffer {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Vec3::new(0.3, 0.2, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));

    let material_ground = Arc::new(Lambertian::from_texture(checker));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
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
                    sphere_material = Arc::new(Lambertian::from_color(albedo));
                    world.add(Arc::new(Sphere::moving(
                        center,
                        center + Vec3::new(0.0, rand::random_range(0.0..0.5), 0.0),
                        0.2,
                        sphere_material,
                    )));
                } else if choose_material < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rand::random_range(0.0..0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::from_color(Vec3::new(0.4, 0.2, 0.1)));
    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world = HittableList::from_object(Arc::new(BvhNode::new(world)));

    let mut cam = Camera::new();
    cam.image_width = 400;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 100;
    cam.max_depth = 20;
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
    cam.background = Vec3::new(0.7, 0.8, 1.0);
    cam.render(&world)
}

fn checkered_spheres() -> ImageBuffer {
    let mut world = HittableList::new();

    let checkered = Arc::new(CheckerTexture::from_colors(
        0.32,
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from_texture(checkered.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::from_texture(checkered.clone())),
    )));

    world = HittableList::from_object(Arc::new(BvhNode::new(world)));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.background = Vec3::new(0.7, 0.8, 1.0);
    cam.render(&world)
}

fn earth() -> ImageBuffer {
    let texture = Arc::new(ImageTexture::new("images/earthmap.jpg"));
    let surface = Arc::new(Lambertian::from_texture(texture));
    let globe = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, surface));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(0.0, 0.0, 12.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.background = Vec3::new(0.7, 0.8, 1.0);
    cam.render(&HittableList::from_object(globe))
}

fn changed_pov() -> ImageBuffer {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::from_color(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
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
                    sphere_material = Arc::new(Lambertian::from_color(albedo));
                    world.add(Arc::new(Sphere::new(
                        center,
                        rand::random_range(0.2..0.5),
                        sphere_material,
                    )));
                } else if choose_material < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rand::random_range(0.0..0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(
                        center,
                        rand::random_range(0.2..0.5),
                        sphere_material,
                    )));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(
                        center,
                        rand::random_range(0.2..0.5),
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::from_color(Vec3::new(0.2, 0.7, 0.3)));
    let material3 = Arc::new(Metal::new(Vec3::new(0.6, 0.3, 0.5), 0.0));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world = HittableList::from_object(Arc::new(BvhNode::new(world)));
    let mut cam = Camera::new();
    cam.image_width = 1200;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 10;
    cam.max_depth = 20;
    cam.vfov = 20.0;
    cam.vup = Vec3::new(0.0, 0.0, -1.0);
    cam.lookfrom = Vec3 {
        x: 0.0,
        y: 20.0,
        z: 0.0,
    };
    cam.lookat = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    cam.defocus_angle = 0.6;
    cam.background = Vec3::new(0.7, 0.8, 1.0);
    cam.focus_dist = 10.0;
    cam.render(&world)
}

fn perlin_spheres() -> ImageBuffer {
    let mut world = HittableList::new();
    let per_tex = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(per_tex.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(per_tex.clone())),
    )));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.background = Vec3::new(0.7, 0.8, 1.0);

    cam.render(&world)
}

fn a_final_render() -> ImageBuffer {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::from_color(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
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
                    sphere_material = Arc::new(Lambertian::from_color(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_material < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rand::random_range(0.0..0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::from_color(Vec3::new(0.4, 0.2, 0.1)));
    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world = HittableList::from_object(Arc::new(BvhNode::new(world)));

    let mut cam = Camera::new();
    cam.image_width = 400;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 100;
    cam.max_depth = 20;
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
    cam.background = Vec3::new(0.7, 0.8, 1.0);
    cam.render(&world)
}

fn earth_and_moon() -> ImageBuffer {
    let texture = Arc::new(ImageTexture::new("images/earthmap.jpg"));
    let texture_moon = Arc::new(ImageTexture::new("images/moontex.jpg"));
    let surface = Arc::new(Lambertian::from_texture(texture));
    let surface_moon = Arc::new(Lambertian::from_texture(texture_moon));
    let globe = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1000.0, surface));
    let moon = Arc::new(Sphere::new(
        Vec3::new(1500.0, 1500.0, 0.0),
        270.0,
        surface_moon,
    ));

    let mut world = HittableList::new();
    world.add(globe);
    world.add(moon);

    let mut cam = Camera::new();
    cam.image_width = 1200;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 10;
    cam.max_depth = 50;
    cam.vfov = 90.0;
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.lookfrom = Vec3 {
        x: 0.0,
        y: 0000.0,
        z: 2000.0,
    };
    cam.lookat = Vec3 {
        x: 1500.0,
        y: 1500.0,
        z: 0.0,
    };
    cam.defocus_angle = 0.6;
    cam.background = Vec3::new(0.7, 0.8, 1.0);
    cam.focus_dist = 2000.0;
    cam.render(&world)
}

fn quads() -> ImageBuffer {
    let mut world = HittableList::new();
    let left_red = Arc::new(Lambertian::from_color(Vec3::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::from_color(Vec3::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::from_color(Vec3::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::from_color(Vec3::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::from_color(Vec3::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 80.0;
    cam.lookfrom = Vec3::new(0.0, 0.0, 9.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.background = Vec3::new(0.7, 0.8, 1.0);
    cam.defocus_angle = 0.0;

    cam.render(&world)
}

fn simple_light() -> ImageBuffer {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));

    let difflight = Arc::new(DiffuseLight::color_source(Vec3::new(4.0, 4.0, 4.0)));

    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight,
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.0, 0.0, 0.0);

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(26.0, 3.0, 6.0);
    cam.lookat = Vec3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world)
}

fn cornell_box() -> ImageBuffer {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from_color(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::color_source(Vec3::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable> = dabba(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    );

    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    world.add(box1);

    let mut box2: Arc<dyn Hittable> = dabba(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white,
    );

    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(box2);

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Vec3::new(278.0, 278.0, -800.0);
    cam.lookat = Vec3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world)
}

fn cornell_smoke() -> ImageBuffer {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from_color(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::color_source(Vec3::new(7.0, 7.0, 7.0)));

    // Walls + light
    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // Boxes
    let mut box1: Arc<dyn Hittable> = dabba(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: Arc<dyn Hittable> = dabba(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white,
    );
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    // Smoke volumes
    world.add(Arc::new(ConstantMedium::from_color(
        box1,
        0.01,
        Vec3::new(0.0, 0.0, 0.0),
    )));

    world.add(Arc::new(ConstantMedium::from_color(
        box2,
        0.01,
        Vec3::new(1.0, 1.0, 1.0),
    )));

    // Camera
    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = Vec3::new(278.0, 278.0, -800.0);
    cam.lookat = Vec3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world)
}
