use crate::random::Random;
use crate::vec3::Point3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new(rng: &mut Random) -> Self {
        let mut randfloat: [f64; POINT_COUNT] = [0.0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            randfloat[i] = rng.f64();
        }

        let mut perm_x: [usize; POINT_COUNT] = [0; POINT_COUNT];
        let mut perm_y: [usize; POINT_COUNT] = [0; POINT_COUNT];
        let mut perm_z: [usize; POINT_COUNT] = [0; POINT_COUNT];

        Self::perlin_generate_perm(&mut perm_x, rng);
        Self::perlin_generate_perm(&mut perm_y, rng);
        Self::perlin_generate_perm(&mut perm_z, rng);

        Self {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = ((4.0 * p.x) as i64 & 255) as usize;
        let j = ((4.0 * p.y) as i64 & 255) as usize;
        let k = ((4.0 * p.z) as i64 & 255) as usize;

        self.randfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn perlin_generate_perm(p: &mut [usize; POINT_COUNT], rng: &mut Random) {
        for i in 0..POINT_COUNT {
            p[i] = i;
        }

        Self::permute(p, POINT_COUNT, rng);
    }

    fn permute(p: &mut [usize; POINT_COUNT], n: usize, rng: &mut Random) {
        for i in (1..n).rev() {
            let target = rng.range(0..i);
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }
}
