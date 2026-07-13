use std::sync::Arc;
use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

/// Records information about a ray-object intersection.
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, outward_normal: Vec3, t: f64, r: Ray, mat_ptr: Arc<dyn Material>, u: f64, v: f64) -> HitRecord {
        // make sure the normal is always facing against the ray
        // NOTE: the parameter `normal` is assumed to have unit length.
        let front_face = front_face(r, outward_normal);
        let normal = if front_face { outward_normal } else { -outward_normal };

        HitRecord { p, normal, mat_ptr, t, u, v, front_face }
    }
}

/// Anything a ray can intersect with.
pub trait Hittable: Send + Sync {
    /// Returns `Some(HitRecord)` if the ray `r` hits the object within given `ray_t` interval,
    /// otherwise returns `None`.
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord>;

    /// Returns bounding box for given object
    fn bounding_box(&self) -> &Aabb;
}

fn front_face(ray: Ray, outward_normal: Vec3) -> bool {
    ray.direction.dot(outward_normal) < 0.0
}

pub struct Translate {
    pub object: Arc<dyn Hittable>,
    pub offset: Vec3,
    pub bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = *object.bounding_box() + offset;
        Translate { object, offset, bbox }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        // Move the ray backwards by the offset
        let offset_r = Ray::new(r.origin - self.offset, r.direction, r.time);

        // Determine whether an intersection exists along the offset ray (and if so, where)
        let mut hit_record = self.object.hit(offset_r, ray_t)?;

        // Move the intersection point forwards by the offset
        hit_record.p += self.offset;

        Some(hit_record)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
