use std::ops::Range;
use rand::rngs::SmallRng;
use rand::{RngExt, SeedableRng};

/// Wrapper to decouple random implementation from the rest of the codebase
#[derive(Debug)]
pub struct Random {
    rng: SmallRng,
}

impl Random {
    /// deterministic
    pub fn new(seed: u64) -> Self {
        Self {
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    /// nondeterministic (OS entropy)
    pub fn from_os() -> Self {
        Random::new(rand::rng().random())
    }
    /// Returns a random real in `[0, 1)`.
    pub fn f64(&mut self) -> f64 {
        self.rng.random()
    }

    /// Returns a random real in given range.
    pub fn range_f64(&mut self, range: Range<f64>) -> f64 {
        self.rng.random_range(range)
    }

    //
    // pub fn random<T>(&mut self) -> T
    // where
    //     rand::distr::StandardUniform: rand::distr::Distribution<T>,
    // {
    //     self.rng.random()
    // }
    //
    // pub fn range<T>(&mut self, range: Range<T>) -> T
    // where
    //     T: SampleUniform + PartialOrd,
    // {
    //     self.rng.random_range(range)
    // }
}
