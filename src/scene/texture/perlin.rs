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

    pub fn noise(&self, point: &Vec3) -> f64 {
        let x = (4.0 * point.x) as i32 & 255;
        let y = (4.0 * point.y) as i32 & 255;
        let z = (4.0 * point.z) as i32 & 255;

        self.randfloat[self.perm_x[x as usize] ^ self.perm_y[y as usize] ^ self.perm_z[z as usize]]
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
