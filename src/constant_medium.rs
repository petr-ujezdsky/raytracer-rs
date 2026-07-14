use crate::aabb::Aabb;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::{interval, utils};
use std::sync::Arc;
use crate::random::Random;

#[derive(Clone)]
pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable>,
    pub neg_inv_density: f64,
    pub phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(
        boundary: Arc<dyn Hittable>,
        density: f64,
        phase_function: Arc<dyn Material>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }

    pub fn from_color(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: Ray, ray_t: Interval, rng: &mut Random) -> Option<HitRecord<'_>> {
        let mut rec1 = self.boundary.hit(r, interval::UNIVERSE, rng)?;

        let mut rec2 = self.boundary.hit(r, Interval::new(rec1.t + 0.0001, utils::INFINITY), rng)?;
        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }

        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f64::ln(rng.f64());

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);

        let rec = HitRecord {
            p,
            // arbitrary
            normal: Vec3::new(1.0, 0.0, 0.0),
            mat_ptr: self.phase_function.as_ref(),
            t,
            u: 0.0,
            v: 0.0,
            // also arbitrary
            front_face: true,
        };

        Some(rec)
    }

    fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }
}