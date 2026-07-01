use std::ops::Range;
use rand::rngs::SmallRng;
use rand::{RngExt, SeedableRng};
use rand::distr::uniform::{SampleRange, SampleUniform};

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

    /// nondeterministic (OS entropy) or seeded
    pub fn from_os_or_seeded(seed: Option<u64>) -> Self {
        match seed {
            Some(s) => Random::new(s),
            None => Random::from_os(),
        }
    }

    /// Returns a random real in `[0, 1)`.
    pub fn f64(&mut self) -> f64 {
        self.rng.random()
    }

    /// Returns a random real in given range.
    pub fn range_f64(&mut self, range: Range<f64>) -> f64 {
        self.rng.random_range(range)
    }

    /// Returns a random usize in given range.
    pub fn range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>
    {
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
