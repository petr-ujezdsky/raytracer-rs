use std::sync::Arc;
use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

/// Records information about a ray-object intersection.
#[derive(Clone)]
pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
    bbox: Aabb,
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(r.time);
        let oc = current_center - r.origin;

        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        // Construct the hit record
        let p = r.at(root);
        let outward_normal = (p - current_center) / self.radius;

        let rec = HitRecord::new(p, outward_normal, root, r, self.mat_ptr.clone());

        Some(rec)
    }

    fn bounding_box(&self) -> &Aabb { &self.bbox }
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f64, mat_ptr: Arc<dyn Material>) -> Sphere {
        let center = Ray::new(static_center, Point3::zero(), 0.0);

        // make sure the radius is >= 0
        let radius = f64::max(0.0, radius);

        let rvec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::from_points(static_center - rvec, static_center + rvec);

        Sphere { center, radius, mat_ptr, bbox }
    }

    pub fn new_moving(center1: Point3, center2: Point3, radius: f64, mat_ptr: Arc<dyn Material>) -> Sphere {
        let center = Ray::new(center1, center2 - center1, 0.0);

        // make sure the radius is >= 0
        let radius = f64::max(0.0, radius);

        let rvec = Vec3::new(radius, radius, radius);
        let bbox1 = Aabb::from_points(center.at(0.0) - rvec, center.at(0.0) + rvec);
        let bbox2 = Aabb::from_points(center.at(1.0) - rvec, center.at(1.0) + rvec);
        let bbox = Aabb::from_aabbs(&bbox1, &bbox2);

        Sphere { center, radius, mat_ptr, bbox }
    }
}
