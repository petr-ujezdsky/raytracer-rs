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
    pub fuzz: f64,
}

impl Metal {
    pub(crate) fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord, rng: &mut Random) -> Option<ScatterRecord> {
        let mut reflected = Vec3::reflect(r_in.direction, rec.normal);

        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector(rng));

        let scattered = Ray::new(rec.p, reflected);

        if scattered.direction.dot(rec.normal) <= 0.0 {
            return None;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered,
        })
    }
}
