use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

/// Records information about a ray-object intersection.
#[derive(Debug, Copy, Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, outward_normal: Vec3, t: f64, r: Ray) -> HitRecord {
        // make sure the normal is always facing against the ray
        // NOTE: the parameter `normal` is assumed to have unit length.
        let front_face = front_face(r, outward_normal);
        let normal = if front_face { outward_normal } else { -outward_normal };

        HitRecord { p, normal, t, front_face }
    }
}

/// Anything a ray can intersect with.
pub trait Hittable {
    /// Returns `Some(HitRecord)` if the ray `r` hits the object within given `ray_t` interval,
    /// otherwise returns `None`.
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord>;
}

fn front_face(ray: Ray, outward_normal: Vec3) -> bool {
    ray.direction.dot(outward_normal) < 0.0
}