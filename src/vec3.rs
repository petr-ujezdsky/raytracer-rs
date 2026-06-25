use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
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

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, o: Vec3) -> Vec3 { Vec3 { x: self.x - o.x, y: self.y - o.y, z: self.z - o.z } }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, s: f64) -> Vec3 { Vec3 { x: self.x * s, y: self.y * s, z: self.z * s } }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, o: Vec3) -> Vec3 { Vec3 { x: self.x * o.x, y: self.y * o.y, z: self.z * o.z } }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, s: f64) -> Vec3 { Vec3 { x: self.x / s, y: self.y / s, z: self.z / s } }
}

// impl Div<f64> for Vec3 {
//     type Output = Vec3;
//     fn div(self, s: f64) -> Vec3 {
//         let inv = 1.0 / s;
//         Vec3 { x: self.x * inv, y: self.y * inv, z: self.z * inv }
//     }
// }

impl Vec3 {
    pub fn length(&self) -> f64 { self.length_squared().sqrt() }

    pub fn length_squared(&self) -> f64 { self.x * self.x + self.y * self.y + self.z * self.z }

    pub fn dot(self, o: Vec3) -> f64 { self.x * o.x + self.y * o.y + self.z * o.z }

    pub fn cross(self, o: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 { v / v.length() }

pub fn dot(u:Vec3, v: Vec3) -> f64 { u.x * v.x + u.y * v.y + u.z * v.z }

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
    fn test_mul() {
        let a = Vec3 { x: 1.0, y: -2.0, z: 3.0 };
        let r = a * 2.0;
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
    fn test_dot_2() {
        let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3 { x: 4.0, y: -5.0, z: 6.0 };
        // 1*4 + 2*(-5) + 3*6 = 4 - 10 + 18 = 12
        assert_eq!(dot(a, b), 12.0);
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
        let u = unit_vector(v);
        // length is 7, so the normalized components are (2/7, 3/7, 6/7)
        assert_eq!(u.x, 2.0 / 7.0);
        assert_eq!(u.y, 3.0 / 7.0);
        assert_eq!(u.z, 6.0 / 7.0);

        // The length of a unit vector must be 1 (allowing for floating point error)
        assert!((u.length() - 1.0).abs() < 1e-10);
    }
}
