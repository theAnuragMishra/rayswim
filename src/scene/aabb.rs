use crate::{
    math::{interval::Interval, vec3::Vec3},
    ray::Ray,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_extrema(a: Vec3, b: Vec3) -> Self {
        Self {
            x: Interval::new(a.x.min(b.x), a.x.max(b.x)),
            y: Interval::new(a.y.min(b.y), a.y.max(b.y)),
            z: Interval::new(a.z.min(b.z), a.z.max(b.z)),
        }
    }

    pub fn enclosing(box1: Self, box2: Self) -> Self {
        Self {
            x: Interval::enclosing(box1.x, box2.x),
            y: Interval::enclosing(box1.y, box2.y),
            z: Interval::enclosing(box1.z, box2.z),
        }
    }

    pub fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn universe() -> Self {
        Self {
            x: Interval::universe(),
            y: Interval::universe(),
            z: Interval::universe(),
        }
    }

    pub fn axis_interval(&self, n: i32) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    pub fn longer_axis(&self) -> i32 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }

    pub fn hit(&self, ray_in: &Ray, mut ray_t: Interval) -> bool {
        for axis in 0..3 {
            let ray_orig = ray_in.origin;
            let ray_dir = ray_in.direction;

            let (ray_orig_axis, ray_dir_axis) = match axis {
                1 => (ray_orig.y, ray_dir.y),
                2 => (ray_orig.z, ray_dir.z),
                _ => (ray_orig.x, ray_dir.x),
            };

            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir_axis;

            let t0 = (ax.min - ray_orig_axis) * adinv;
            let t1 = (ax.max - ray_orig_axis) * adinv;

            if t0 < t1 {
                ray_t.min = ray_t.min.max(t0);
                ray_t.max = ray_t.max.min(t1);
            } else {
                ray_t.min = ray_t.min.max(t1);
                ray_t.max = ray_t.max.min(t0);
            }
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}
