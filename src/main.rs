mod geometry;
mod math;
mod ray;
mod scene;

use geometry::sphere::Sphere;
use math::vec3::Vec3;
use ray::ray::Ray;
use scene::hittable::Hittable;

fn main() {
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

    if let Some(hit) = sphere.hit(&ray, 0.001, f64::INFINITY) {
        println!("Hit at t = {}, point = {:?}", hit.t, hit.point);
    } else {
        println!("No hit");
    }
}
