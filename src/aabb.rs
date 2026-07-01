use crate::interval::{Interval, EMPTY};
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Debug, Copy, Clone, Default)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn empty() -> Aabb {
        Aabb { x: EMPTY, y: EMPTY, z: EMPTY }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb { x, y, z }
    }

    pub fn from_points(a: Point3, b: Point3) -> Aabb {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.

        let x = if a.x <= b.x { Interval::new(a.x, b.x) } else { Interval::new(b.x, a.x) };
        let y = if a.y <= b.y { Interval::new(a.y, b.y) } else { Interval::new(b.y, a.y) };
        let z = if a.z <= b.z { Interval::new(a.z, b.z) } else { Interval::new(b.z, a.z) };

        Aabb { x, y, z }
    }

    pub fn from_aabbs(a: &Aabb, b: &Aabb) -> Aabb {
        Aabb {
            x: Interval::from_enclosing(a.x, b.x),
            y: Interval::from_enclosing(a.y, b.y),
            z: Interval::from_enclosing(a.z, b.z),
        }
    }

    pub fn axis_interval(&self, axis: usize) -> Interval {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("invalid axis value: {axis}")
        }
    }

    pub fn hit(&self, r: Ray, ray_t: &mut Interval) -> bool {
        let ray_orig = r.origin;
        let ray_dir = r.direction;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}