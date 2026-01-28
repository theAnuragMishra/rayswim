use crate::{math::vec3::Vec3, scene::texture::Texture};

pub struct Perlin<const N: usize> {
    randfloat: [f64; N],
    perm_x: [usize; N],
    perm_y: [usize; N],
    perm_z: [usize; N],
}

impl<const N: usize> Perlin<N> {
    pub fn new() -> Self {
        let mut randfloat = [0.0; N];
        randfloat
            .iter_mut()
            .for_each(|x| *x = rand::random_range(0.0..1.0));

        let mut perm_x = [0; N];
        let mut perm_y = [0; N];
        let mut perm_z = [0; N];

        Self::generate_perm(&mut perm_x);
        Self::generate_perm(&mut perm_y);
        Self::generate_perm(&mut perm_z);

        Self {
            randfloat,
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

    pub fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }

        return accum;
    }

    pub fn noise(&self, point: &Vec3) -> f64 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }
        Self::trilinear_interp(c, u, v, w)
    }
}

pub struct NoiseTexture {
    noise: Perlin<256>,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, point: &Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * self.noise.noise(point)
    }
}
