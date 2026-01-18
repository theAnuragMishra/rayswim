use crate::math::vec3::Vec3;
use crate::scene::hittable::{HitRecord, Hittable};
use crate::scene::hittable_list::HittableList;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * v.dot(n)
}

pub fn color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::default();
    }

    let mut rec = HitRecord {
        point: Vec3::default(),
        normal: Vec3::default(),
        t: 0.0,
        front_face: false,
    };

    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        // Lambertian diffuse
        let target = rec.point + rec.normal + Vec3::random_in_unit_sphere();
        return color(&Ray::new(rec.point, target - rec.point), world, depth - 1) * 0.5;
    }

    // background
    let unit = r.direction.normalized();
    let t = 0.5 * (unit.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
