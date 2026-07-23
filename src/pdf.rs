use crate::hittable::Hittable;
use crate::onb::Onb;
use crate::random::Random;
use crate::utils;
use crate::vec3::Vec3;

/// Probability density function
pub trait Pdf: Send + Sync {
    fn value(&self, direction: Vec3, rng: &mut Random) -> f64;

    fn generate(&self, rng: &mut Random) -> Vec3;
}

/// PDF for whole sphere
pub struct SpherePdf {}

impl Pdf for SpherePdf {
    fn value(&self, _direction: Vec3, _rng: &mut Random) -> f64 {
        1.0 / (4.0 * utils::PI)
    }

    fn generate(&self, rng: &mut Random) -> Vec3 {
        Vec3::random_unit_vector(rng)
    }
}

/// PDF for hemisphere with cosine probability distribution
pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(n: Vec3) -> CosinePdf {
        CosinePdf { uvw: Onb::new(n) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3, _rng: &mut Random) -> f64 {
        let cosine_theta = Vec3::dot(direction.unit_vector(), self.uvw.w);
        f64::max(0.0, cosine_theta / utils::PI)
    }

    fn generate(&self, rng: &mut Random) -> Vec3 {
        self.uvw.transform(Vec3::random_cosine_direction(rng)).unit_vector()
    }
}

/// PDF for hittable object
pub struct HittablePdf<'a> {
    objects: &'a dyn Hittable,
    origin: Vec3,
}

impl<'a> HittablePdf<'a> {
    pub fn new(objects: &'a dyn Hittable, origin: Vec3) -> HittablePdf<'a> {
        HittablePdf { objects, origin }
    }
}

impl<'a> Pdf for HittablePdf<'a> {
    fn value(&self, direction: Vec3, rng: &mut Random) -> f64 {
        self.objects.pdf_value(self.origin, direction, rng)
    }

    fn generate(&self, rng: &mut Random) -> Vec3 {
        self.objects.random(self.origin, rng)
    }
}

pub struct MixturePdf<'a> {
    p: [&'a dyn Pdf; 2],
}

impl<'a> MixturePdf<'a> {
    pub fn new(p0: &'a dyn Pdf, p1: &'a dyn Pdf) -> MixturePdf<'a> {
        MixturePdf { p: [p0, p1] }
    }
}

impl<'a> Pdf for MixturePdf<'a> {
    fn value(&self, direction: Vec3, rng: &mut Random) -> f64 {
        0.5 * self.p[0].value(direction, rng) + 0.5 * self.p[1].value(direction, rng)
    }

    fn generate(&self, rng: &mut Random) -> Vec3 {
        if rng.f64() < 0.5 {
            self.p[0].generate(rng)
        } else {
            self.p[1].generate(rng)
        }
    }
}
