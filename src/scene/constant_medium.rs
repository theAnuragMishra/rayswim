use std::{f64::INFINITY, sync::Arc};

use rand::random_range;

use crate::{
    math::{interval::Interval, vec3::Vec3},
    scene::{
        hittable::{HitRecord, Hittable},
        material::{Material, isotropic::Isotropic},
        texture::Texture,
    },
};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn from_texture(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_texture(tex)),
        }
    }
    pub fn from_color(boundary: Arc<dyn Hittable>, density: f64, albedo: Vec3) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self) -> super::aabb::Aabb {
        self.boundary.bounding_box()
    }
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t: crate::math::interval::Interval,
    ) -> Option<super::hittable::HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(ray, Interval::universe()) {
            if let Some(mut rec2) = self
                .boundary
                .hit(ray, Interval::new(rec1.t + 0.0001, INFINITY))
            {
                if rec1.t < ray_t.min {
                    rec1.t = ray_t.min;
                }
                if rec2.t > ray_t.max {
                    rec2.t = ray_t.max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }
                let ray_length = ray.direction.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * f64::ln(random_range(0.0..1.0));
                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let t = rec1.t + hit_distance / ray_length;
                return Some(HitRecord::new(
                    ray.at(t),
                    Vec3::new(1.0, 0.0, 0.0),
                    t,
                    self.phase_function.clone(),
                    0.0,
                    0.0,
                    ray,
                ));
            }
            return None;
        }
        return None;
    }
}
