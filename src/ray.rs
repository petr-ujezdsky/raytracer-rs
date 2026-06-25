use crate::vec3::{Point3, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 { self.origin + t * self.direction }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let ray = Ray {
            origin: Point3 { x: 1.0, y: 2.0, z: 3.0 },
            direction: Vec3 { x: 4.0, y: 5.0, z: 6.0 },
        };

        // at(0) returns the origin
        let p0 = ray.at(0.0);
        assert_eq!(p0.x, 1.0);
        assert_eq!(p0.y, 2.0);
        assert_eq!(p0.z, 3.0);

        // at(2) = origin + 2 * direction = (1+8, 2+10, 3+12) = (9, 12, 15)
        let p2 = ray.at(2.0);
        assert_eq!(p2.x, 9.0);
        assert_eq!(p2.y, 12.0);
        assert_eq!(p2.z, 15.0);

        // Negative t goes in the opposite direction: (1-4, 2-5, 3-6) = (-3, -3, -3)
        let pn = ray.at(-1.0);
        assert_eq!(pn.x, -3.0);
        assert_eq!(pn.y, -3.0);
        assert_eq!(pn.z, -3.0);
    }
}
