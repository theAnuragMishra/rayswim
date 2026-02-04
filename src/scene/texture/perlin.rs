use crate::{math::vec3::Vec3, scene::texture::Texture};

pub struct Perlin<const N: usize> {
    randvec: [Vec3; N],
    perm_x: [usize; N],
    perm_y: [usize; N],
    perm_z: [usize; N],
}

impl<const N: usize> Perlin<N> {
    pub fn new() -> Self {
        let randvec: [Vec3; N] = std::array::from_fn(|_| Vec3::random_unit_vector());

        let mut perm_x = [0; N];
        let mut perm_y = [0; N];
        let mut perm_z = [0; N];

        Self::generate_perm(&mut perm_x);
        Self::generate_perm(&mut perm_y);
        Self::generate_perm(&mut perm_z);

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn generate_perm(perm: &mut [usize]) {
        for i in 0..N {
            perm[i] = i;
        }
        Self::permute(perm);
    }
    pub fn permute(perm: &mut [usize]) {
        for i in (0..N).rev() {
            let target = rand::random_range(0..=i);
            perm.swap(i, target);
        }
    }

    pub fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += ((i as f64 * uu) + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }
        accum
    }

    pub fn noise(&self, point: Vec3) -> f64 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let c: [[[Vec3; 2]; 2]; 2] = std::array::from_fn(|di| {
            std::array::from_fn(|dj| {
                std::array::from_fn(|dk| {
                    self.randvec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]]
                })
            })
        });
        Self::perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, mut p: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(p);
            weight *= 0.5;
            p = p * 2.0;
        }
        accum.abs()
    }
}

pub struct NoiseTexture {
    noise: Perlin<256>,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, point: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * self.noise.turb(point, 7)
    }
}
