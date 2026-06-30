use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};
use crate::random::Random;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Type alias for Vec3 used to represent a point in 3D space.
pub type Point3 = Vec3;

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, o: Vec3) -> Vec3 { Vec3 { x: self.x + o.x, y: self.y + o.y, z: self.z + o.z } }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) { *self = *self + rhs; }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, o: Vec3) -> Vec3 { Vec3 { x: self.x - o.x, y: self.y - o.y, z: self.z - o.z } }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) { *self = *self - rhs; }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 { Vec3 { x: -self.x, y: -self.y, z: -self.z } }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, o: Vec3) -> Vec3 { Vec3 { x: self.x * o.x, y: self.y * o.y, z: self.z * o.z } }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) { *self = *self * rhs; }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) { *self = *self / rhs; }
}

/// Generates scalar `Mul` and `Div` for Vec3 for every listed numeric type,
/// so you can write `v * 2`, `2 * v`, `v / 2`, `v * 2.0`, ... without manual `as f64`.
macro_rules! impl_scalar_ops {
    ($($t:ty),* $(,)?) => {$(
        impl Mul<$t> for Vec3 {
            type Output = Vec3;
            fn mul(self, s: $t) -> Vec3 {
                let s = s as f64;
                Vec3 { x: self.x * s, y: self.y * s, z: self.z * s }
            }
        }

        impl Mul<Vec3> for $t {
            type Output = Vec3;
            fn mul(self, v: Vec3) -> Vec3 { v * self }
        }

        impl Div<$t> for Vec3 {
            type Output = Vec3;
            fn div(self, s: $t) -> Vec3 {
                let s = s as f64;
                Vec3 { x: self.x / s, y: self.y / s, z: self.z / s }
            }
        }

        // impl Div<$t> for Vec3 {
        //     type Output = Vec3;
        //     fn div(self, s: $t) -> Vec3 {
        //         let inv = 1.0 / s as f64;
        //         Vec3 { x: self.x * inv, y: self.y * inv, z: self.z * inv }
        //     }
        // }
    )*};
}

impl_scalar_ops!(f64, i32);


impl Vec3 {
    pub fn zero() -> Vec3 { Vec3 { x: 0.0, y: 0.0, z: 0.0 } }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 { Vec3 { x, y, z } }

    pub fn random(rng: &mut Random) -> Vec3 {
        Vec3 {
            x: rng.f64(),
            y: rng.f64(),
            z: rng.f64(),
        }
    }

    pub fn random_range(rng: &mut Random, range: Range<f64>) -> Vec3 {
        Vec3 {
            x: rng.range_f64(range.clone()),
            y: rng.range_f64(range.clone()),
            z: rng.range_f64(range),
        }
    }

    pub fn random_unit_vector(rng: &mut Random) -> Vec3 {
        loop {
            let p = Vec3::random_range(rng, -1.0..1.0);
            let lensq = p.length_squared();

            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_in_unit_disk(rng: &mut Random) -> Vec3 {
        loop {
            let p = Vec3::new(rng.range_f64(-1.0..1.0), rng.range_f64(-1.0..1.0), 0.0);

            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_on_hemisphere(rng: &mut Random, normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector(rng);

        // In the same hemisphere as the normal
        if on_unit_sphere.dot(normal) > 0.0 { on_unit_sphere } else { -on_unit_sphere }
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min((-uv).dot(n), 1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }

    pub fn unit_vector(self) -> Vec3 { self / self.length() }

    pub fn length(self) -> f64 { self.length_squared().sqrt() }

    pub fn length_squared(self) -> f64 { self.x * self.x + self.y * self.y + self.z * self.z }

    /// Return true if the vector is close to zero in all dimensions.
    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn dot(self, o: Vec3) -> f64 { self.x * o.x + self.y * o.y + self.z * o.z }

    pub fn cross(self, o: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        let r = a + b;
        assert_eq!(r.x, 5.0);
        assert_eq!(r.y, 7.0);
        assert_eq!(r.z, 9.0);
    }

    #[test]
    fn test_sub() {
        let a = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        let b = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let r = a - b;
        assert_eq!(r.x, 3.0);
        assert_eq!(r.y, 3.0);
        assert_eq!(r.z, 3.0);
    }

    #[test]
    fn test_neg() {
        let a = Vec3 { x: 1.0, y: -2.0, z: 3.0 };
        let r = -a;
        assert_eq!(r.x, -1.0);
        assert_eq!(r.y, 2.0);
        assert_eq!(r.z, -3.0);
    }

    #[test]
    fn test_mul() {
        let a = Vec3 { x: 1.0, y: -2.0, z: 3.0 };
        let r = a * 2.0;
        assert_eq!(r.x, 2.0);
        assert_eq!(r.y, -4.0);
        assert_eq!(r.z, 6.0);
    }

    #[test]
    fn test_mul_scalar_left() {
        let a = Vec3 { x: 1.0, y: -2.0, z: 3.0 };
        let r = 2.0 * a;
        assert_eq!(r.x, 2.0);
        assert_eq!(r.y, -4.0);
        assert_eq!(r.z, 6.0);
    }

    #[test]
    fn test_mul_i32_left() {
        let a = Vec3 { x: 1.0, y: -2.0, z: 3.0 };
        let r = 2 * a; // i32 * Vec3
        assert_eq!(r.x, 2.0);
        assert_eq!(r.y, -4.0);
        assert_eq!(r.z, 6.0);
    }

    #[test]
    fn test_mul_vec3() {
        let a = Vec3 { x: 1.0, y: -2.0, z: 3.0 };
        let b = Vec3 { x: 4.0, y: 5.0, z: -6.0 };
        let r = a * b;
        assert_eq!(r.x, 4.0);
        assert_eq!(r.y, -10.0);
        assert_eq!(r.z, -18.0);
    }

    #[test]
    fn test_div() {
        let a = Vec3 { x: 2.0, y: -4.0, z: 6.0 };
        let r = a / 2.0;
        assert_eq!(r.x, 1.0);
        assert_eq!(r.y, -2.0);
        assert_eq!(r.z, 3.0);
    }

    #[test]
    fn test_div_i32() {
        let a = Vec3 { x: 2.0, y: -4.0, z: 6.0 };
        let r = a / 2; // Vec3 / i32
        assert_eq!(r.x, 1.0);
        assert_eq!(r.y, -2.0);
        assert_eq!(r.z, 3.0);
    }

    #[test]
    fn test_length_squared() {
        let v = Vec3 { x: 2.0, y: 3.0, z: 6.0 };
        // 2*2 + 3*3 + 6*6 = 4 + 9 + 36 = 49
        assert_eq!(v.length_squared(), 49.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3 { x: 2.0, y: 3.0, z: 6.0 };
        // sqrt(49) = 7
        assert_eq!(v.length(), 7.0);
    }

    #[test]
    fn test_dot() {
        let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3 { x: 4.0, y: -5.0, z: 6.0 };
        // 1*4 + 2*(-5) + 3*6 = 4 - 10 + 18 = 12
        assert_eq!(a.dot(b), 12.0);
    }

    #[test]
    fn test_cross() {
        // Standard basis: x × y = z
        let x = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
        let y = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
        let r = x.cross(y);
        assert_eq!(r.x, 0.0);
        assert_eq!(r.y, 0.0);
        assert_eq!(r.z, 1.0);

        // General case
        let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        let c = a.cross(b);
        // (2*6 - 3*5, 3*4 - 1*6, 1*5 - 2*4) = (-3, 6, -3)
        assert_eq!(c.x, -3.0);
        assert_eq!(c.y, 6.0);
        assert_eq!(c.z, -3.0);
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3 { x: 2.0, y: 3.0, z: 6.0 };
        let u = v.unit_vector();
        // length is 7, so the normalized components are (2/7, 3/7, 6/7)
        assert_eq!(u.x, 2.0 / 7.0);
        assert_eq!(u.y, 3.0 / 7.0);
        assert_eq!(u.z, 6.0 / 7.0);

        // The length of a unit vector must be 1 (allowing for floating point error)
        assert!((u.length() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_zero() {
        let z = Vec3::zero();
        assert_eq!(z.x, 0.0);
        assert_eq!(z.y, 0.0);
        assert_eq!(z.z, 0.0);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        a += Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        assert_eq!(a.x, 5.0);
        assert_eq!(a.y, 7.0);
        assert_eq!(a.z, 9.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
        a -= Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!(a.x, 3.0);
        assert_eq!(a.y, 3.0);
        assert_eq!(a.z, 3.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Vec3 { x: 1.0, y: -2.0, z: 3.0 };
        a *= 2.0;
        assert_eq!(a.x, 2.0);
        assert_eq!(a.y, -4.0);
        assert_eq!(a.z, 6.0);
    }

    #[test]
    fn test_div_assign() {
        let mut a = Vec3 { x: 2.0, y: -4.0, z: 6.0 };
        a /= 2.0;
        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, -2.0);
        assert_eq!(a.z, 3.0);
    }
}
