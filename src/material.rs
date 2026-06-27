use crate::color::Color;
use crate::hittable::HitRecord;
use crate::random::Random;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

/// Surface material.
pub trait Material {
    fn scatter(&self, r_in: Ray, rec: &HitRecord, rng: &mut Random) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub(crate) fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &HitRecord, rng: &mut Random) -> Option<ScatterRecord> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector(rng);

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, scatter_direction),
        })
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub(crate) fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord, rng: &mut Random) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(r_in.direction, rec.normal);

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, reflected),
        })
    }
}
