#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> Vec3 {
        let len = self.length_squared().sqrt();
        if len < 1e-12 {
            // avoid divide-by-zero
            self // returning unnormalized is safer than NaNs
        } else {
            self / len
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::new(
                rand::random_range(-1.0..1.0),
                rand::random_range(-1.0..1.0),
                rand::random_range(-1.0..1.0),
            );

            if 1e-160 < p.length_squared() && p.length_squared() < 1.0 {
                return p.normalized();
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                rand::random_range(-1.0..1.0),
                rand::random_range(-1.0..1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn is_near_zero(&self) -> bool {
        self.x.abs() < 1e-8 && self.y.abs() < 1e-8 && self.z.abs() < 1e-8
    }

    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - n * 2.0 * self.dot(n)
    }

    pub fn refract(self, n: Vec3, ri: f64) -> Vec3 {
        let cos_theta = f64::min((-self).dot(n), 1.0);
        let rout_perp = ri * (self + cos_theta * n);
        let rout_parallel = -(1.0 - rout_perp.length_squared()).abs().sqrt() * n;
        rout_parallel + rout_perp
    }
}

use std::ops::{Add, Div, Mul, Neg, Sub};

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
