use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

#[derive(Clone)]
pub struct Quad {
    pub q: Point3,
    pub u: Vec3,
    pub v: Vec3,
    pub mat: Arc<dyn Material>,
    pub aabb: Aabb,
    pub normal: Vec3,
    pub d: f64,
    pub w: Vec3,
}

impl Hittable for Quad {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(r.direction);

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - self.normal.dot(r.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);

        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        let (u, v) = Self::interior_uv_coords(alpha, beta)?;

        // Ray hits the 2D shape; set the rest of the hit record and return true.
        Some(HitRecord::new2(intersection, self.normal, t, r, self.mat.clone(), u, v))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.aabb
    }
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Quad {
        let n = u.cross(v);
        let normal = n.unit_vector();
        let d = normal.dot(q);
        let w = n / n.dot(n);

        Quad {
            q,
            u,
            v,
            mat,
            aabb: Self::compute_bounding_box(q, u, v),
            normal,
            d,
            w
        }
    }

    // /// Compute the bounding box of all four vertices.
    // fn set_bounding_box(&mut self) -> Quad {
    //     let bbox_diagonal1 = Aabb::from_points(self.q, self.q + self.u + self.v);
    //     let bbox_diagonal2 = Aabb::from_points(self.q + self.u, self.q + self.v);
    //     self.aabb = Aabb::from_points(bbox_diagonal1.min, bbox_diagonal2.max);
    //     *self
    // }

    // /// Compute the bounding box of all four vertices.
    // fn bounding_box(&self) -> Aabb {
    //     let bbox_diagonal1 = Aabb::from_points(self.q, self.q + self.u + self.v);
    //     let bbox_diagonal2 = Aabb::from_points(self.q + self.u, self.q + self.v);
    //     Aabb::from_points(bbox_diagonal1.min, bbox_diagonal2.max)
    // }

    /// Compute the bounding box of all four vertices.
    fn compute_bounding_box(q: Point3, u: Vec3, v: Vec3) -> Aabb {
        let bbox_diagonal1 = Aabb::from_points(q, q + u + v);
        let bbox_diagonal2 = Aabb::from_points(q + u, q + v);
        Aabb::from_aabbs(&bbox_diagonal1, &bbox_diagonal2)
    }

    fn interior_uv_coords(a: f64, b: f64) -> Option<(f64, f64)> {
        let unit_interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return None;
        }

        Some((a, b))
    }
}
