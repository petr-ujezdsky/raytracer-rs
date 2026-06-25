use std::ops::{Add, Mul, Sub, Div};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

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
    pub fn dot(self, o: Vec3) -> f64 { self.x * o.x + self.y * o.y + self.z * o.z }
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
    fn test_mul() {
        let a = Vec3 { x: 1.0, y: -2.0, z: 3.0 };
        let r = a * 2.0;
        assert_eq!(r.x, 2.0);
        assert_eq!(r.y, -4.0);
        assert_eq!(r.z, 6.0);
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
    fn test_dot() {
        let a = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vec3 { x: 4.0, y: -5.0, z: 6.0 };
        // 1*4 + 2*(-5) + 3*6 = 4 - 10 + 18 = 12
        assert_eq!(a.dot(b), 12.0);
    }
}

