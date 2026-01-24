use std::{cmp::Ordering, sync::Arc};

use crate::{
    math::interval::Interval,
    ray::Ray,
    scene::{
        aabb::Aabb,
        hittable::{HitRecord, Hittable},
        hittable_list::HittableList,
    },
};

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(mut list: HittableList) -> Self {
        Self::from_objects(&mut list.objects)
    }
    pub fn from_objects(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let axis = rand::random_range(0..=2);

        let comparator = match axis {
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => Self::box_x_compare,
        };

        let object_span = objects.len();

        let left;
        let right;

        match object_span {
            1 => {
                left = objects[0].clone();
                right = objects[0].clone();
            }
            2 => {
                left = objects[0].clone();
                right = objects[1].clone();
            }
            _ => {
                objects.sort_unstable_by(comparator);
                let mid = object_span / 2;
                left = Arc::new(BvhNode::from_objects(&mut objects[0..mid]));
                right = Arc::new(BvhNode::from_objects(&mut objects[mid..]));
            }
        }
        let bbox = Aabb::enclosing(left.bounding_box(), right.bounding_box());
        Self { left, right, bbox }
    }

    pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: i32) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap_or(Ordering::Equal)
    }

    pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }
    pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }
    pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, ray_t) {
            return false;
        }
        let hit_left = self.left.hit(ray, ray_t, rec);
        let hit_right = self.right.hit(
            ray,
            Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
