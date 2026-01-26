use crate::math::vec3::Vec3;

pub mod checkered;
pub mod solid;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Vec3;
}
