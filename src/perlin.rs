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
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        // Hermite cubic
        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as i64;
        let j = p.y.floor() as i64;
        let k = p.z.floor() as i64;
        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[
                        self.perm_x[((i + di as i64) & 255) as usize] ^
                            self.perm_y[((j + dj as i64) & 255) as usize] ^
                            self.perm_z[((k + dk as i64) & 255) as usize]
                        ];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
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

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let fi = i as f64;
                    let fj = j as f64;
                    let fk = k as f64;

                    accum += (fi * u + (1.0 - fi) * (1.0 - u))
                        * (fj * v + (1.0 - fj) * (1.0 - v))
                        * (fk * w + (1.0 - fk) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
    }
}
