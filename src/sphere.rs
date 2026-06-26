use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

/// Records information about a ray-object intersection.
#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - r.origin;

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
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        // Construct the hit record
        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;

        let rec = HitRecord::new(p, outward_normal, root, r);

        Some(rec)
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        // make sure the radius is >= 0
        Sphere { center,  radius: f64::max(0.0, radius) }
    }
}
