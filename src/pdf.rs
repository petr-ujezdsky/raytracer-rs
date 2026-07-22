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
