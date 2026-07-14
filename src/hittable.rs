use std::sync::Arc;
use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils;
use crate::vec3::{Point3, Vec3};

/// Records information about a ray-object intersection.
#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: &'a dyn Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Point3, outward_normal: Vec3, t: f64, r: Ray, mat_ptr: &'a dyn Material, u: f64, v: f64) -> HitRecord<'a> {
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
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord<'_>>;

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
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
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

pub struct RotateY {
    pub object: Arc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut bbox = *object.bounding_box();

        let mut min = Point3::new(utils::INFINITY, utils::INFINITY, utils::INFINITY);
        let mut max = Point3::new(-utils::INFINITY, -utils::INFINITY, -utils::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + ((1 - i) as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + ((1 - j) as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + ((1 - k) as f64) * bbox.z.min;

                    let newx =  cos_theta*x + sin_theta*z;
                    let newz = -sin_theta*x + cos_theta*z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }

        bbox = Aabb::from_points(min, max);

        RotateY { object, sin_theta, cos_theta, bbox }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        // Rotate the ray backwards by the angle
        let rotated_origin = Vec3::new(
            self.cos_theta * r.origin.x - self.sin_theta * r.origin.z,
            r.origin.y,
            self.sin_theta * r.origin.x + self.cos_theta * r.origin.z,
        );

        let rotated_direction = Vec3::new(
            self.cos_theta * r.direction.x - self.sin_theta * r.direction.z,
            r.direction.y,
            self.sin_theta * r.direction.x + self.cos_theta * r.direction.z,
        );

        let rotated_ray = Ray::new(rotated_origin, rotated_direction, r.time);

        // Determine whether an intersection exists in object space (and if so, where).
        let mut hit_record = self.object.hit(rotated_ray, ray_t)?;

        // Transform the intersection from object space back to world space.
        hit_record.p = Vec3::new(
            self.cos_theta * hit_record.p.x + self.sin_theta * hit_record.p.z,
            hit_record.p.y,
            -self.sin_theta * hit_record.p.x + self.cos_theta * hit_record.p.z,
        );

        hit_record.normal = Vec3::new(
            self.cos_theta * hit_record.normal.x + self.sin_theta * hit_record.normal.z,
            hit_record.normal.y,
            -self.sin_theta * hit_record.normal.x + self.cos_theta * hit_record.normal.z,
        );

        Some(hit_record)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
