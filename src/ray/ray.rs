use crate::math::vec3::Vec3;
use crate::scene::hittable::{HitRecord, Hittable};
use crate::scene::hittable_list::HittableList;
use crate::scene::material::Material;
use crate::scene::material::lambertian::Lambertian;

#[derive(Clone, Copy, Debug, Default)]
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

pub fn color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
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
            return attenuation * color(&scattered, world, depth - 1);
        }
        return Vec3::default();
    }

    // background
    let unit = r.direction.normalized();
    let t = 0.5 * (unit.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
