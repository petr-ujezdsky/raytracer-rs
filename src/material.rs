use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::random::Random;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::utils;
use crate::vec3::Vec3;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

/// Surface material.
pub trait Material: Send + Sync {
    fn scatter(&self, r_in: Ray, rec: &HitRecord<'_>, rng: &mut Random) -> Option<ScatterRecord>;

    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Color {
        Color::zero()
    }

    fn scattering_pdf(&self, _r_in: Ray, _rec: &HitRecord<'_>, _scattered: Ray, _rng: &mut Random) -> f64 {
        0.0
    }
}

pub struct Lambertian {
    pub tex: Arc<dyn Texture>,

}

impl Lambertian {
    pub fn new(tex: Arc<dyn Texture>) -> Lambertian {
        Lambertian { tex }
    }

    pub fn from_color(color: Color) -> Lambertian {
        Lambertian { tex: Arc::new(SolidColor::new(color)) }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &HitRecord<'_>, rng: &mut Random) -> Option<ScatterRecord> {
        let mut scatter_direction = Vec3::random_on_hemisphere(rng, rec.normal);

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(ScatterRecord {
            attenuation: self.tex.value(rec.u, rec.v, &rec.p),
            scattered: Ray::new(rec.p, scatter_direction, r_in.time),
        })
    }

    // fn scattering_pdf(&self, _r_in: Ray, rec: &HitRecord<'_>, scattered: Ray, _rng: &mut Random) -> f64 {
    //     let cos_theta = Vec3::dot(rec.normal, scattered.direction.unit_vector());
    //
    //     if cos_theta < 0.0 {
    //         0.0
    //     } else {
    //         cos_theta / utils::PI
    //     }
    // }

    fn scattering_pdf(&self, _r_in: Ray, _rec: &HitRecord<'_>, _scattered: Ray, _rng: &mut Random) -> f64 {
        1.0 / (2.0 * utils::PI)
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
    fn scatter(&self, r_in: Ray, rec: &HitRecord<'_>, rng: &mut Random) -> Option<ScatterRecord> {
        let mut reflected = Vec3::reflect(r_in.direction, rec.normal);

        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector(rng));

        let scattered = Ray::new(rec.p, reflected, r_in.time);

        if scattered.direction.dot(rec.normal) <= 0.0 {
            return None;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered,
        })
    }
}

pub struct Dielectric {
    /// Refractive index in vacuum or air, or the ratio of the material's refractive index over
    /// the refractive index of the enclosing media
    pub refraction_index: f64,
}

impl Dielectric {
    pub(crate) fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord<'_>, rng: &mut Random) -> Option<ScatterRecord> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = f64::min((-unit_direction).dot(rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > rng.f64() {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, ri)
        };

        let scattered = Ray::new(rec.p, direction, r_in.time);

        Some(ScatterRecord {
            attenuation,
            scattered,
        })
    }
}

pub struct DiffuseLight {
    pub tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub(crate) fn new(tex: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { tex }
    }

    pub(crate) fn from_color(color: Color) -> DiffuseLight {
        DiffuseLight { tex: Arc::new(SolidColor::new(color)) }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: Ray, _rec: &HitRecord<'_>, _rng: &mut Random) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Color {
        self.tex.value(u, v, &p)
    }
}

pub struct Isotropic {
    pub tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(tex: Arc<dyn Texture>) -> Isotropic {
        Isotropic { tex }
    }

    pub fn from_color(albedo: Color) -> Isotropic {
        Isotropic { tex: Arc::new(SolidColor::new(albedo)) }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: Ray, rec: &HitRecord<'_>, rng: &mut Random) -> Option<ScatterRecord> {
        let scattered = Ray::new(rec.p, Vec3::random_unit_vector(rng), r_in.time);
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);

        Some(ScatterRecord {
            attenuation,
            scattered,
        })
    }
}
