use crate::utils::INFINITY;
use std::ops::{Add};

/// A simple struct representing a closed interval [min, max] of floating-point numbers.
#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self { EMPTY }
}

impl Interval {
    /// Creates an interval with the given `min` and `max` bounds.
    pub const fn new(min: f64, max: f64) -> Interval { Interval { min, max } }

    /// Creates new interval that tightly enclose the two input intervals
    pub fn from_enclosing(a: Interval, b: Interval) -> Interval {
        Interval {
            min: f64::min(a.min, b.min),
            max: f64::max(a.max, b.max),
        }
    }

    /// Returns the length of the interval (`max - min`).
    ///
    /// The result is negative for an empty interval.
    pub fn size(&self) -> f64 { self.max - self.min }

    /// Returns true if `x` lies within the closed interval [min, max].
    pub fn contains(self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Returns true if `x` lies strictly within the open interval (min, max).
    pub fn surrounds(self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Clamps `x` to the interval: returns `min` if `x < min`, `max` if `x > max`,
    /// otherwise `x` unchanged.
    pub fn clamp(self, x: f64) -> f64 {
        x.clamp(self.min, self.max)

        // or not panic variant for NaN etc
        // x.max(self.min).min(self.max)
    }

    /// Returns interval expanded by `delta / 2` on each end
    pub fn expand(self, delta: f64) -> Interval {
        let padding = delta / 2.0;

        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

impl Add<f64> for Interval {
    type Output = Interval;

    fn add(self, displacement: f64) -> Interval {
        Interval::new(self.min + displacement, self.max + displacement)
    }
}

/// The empty interval: contains no points.
///
/// `min` is `+∞` and `max` is `-∞`, so [`contains`](Interval::contains)
/// and [`surrounds`](Interval::surrounds) always return `false`.
pub const EMPTY: Interval = Interval::new(INFINITY, -INFINITY);

/// The universe interval: contains every real number.
///
/// `min` is `-∞` and `max` is `+∞`, so [`contains`](Interval::contains)
/// and [`surrounds`](Interval::surrounds) always return `true`.
pub const UNIVERSE: Interval = Interval::new(-INFINITY, INFINITY);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let i = EMPTY;
        // An empty interval contains nothing.
        assert!(!i.contains(0.0));
        assert!(!i.surrounds(0.0));
        // size is negative (max < min)
        assert!(i.size() < 0.0);
    }

    #[test]
    fn test_universe() {
        let i = UNIVERSE;
        // The universe contains every real number.
        assert!(i.contains(0.0));
        assert!(i.contains(1e300));
        assert!(i.contains(-1e300));
        assert!(i.surrounds(0.0));
        // size is positive infinity
        assert_eq!(i.size(), INFINITY);
    }

    #[test]
    fn test_size() {
        let i = Interval::new(1.0, 4.0);
        assert_eq!(i.size(), 3.0);
    }

    #[test]
    fn test_contains() {
        let i = Interval::new(1.0, 4.0);
        // inside
        assert!(i.contains(2.0));
        // boundaries are included (closed interval)
        assert!(i.contains(1.0));
        assert!(i.contains(4.0));
        // outside
        assert!(!i.contains(0.0));
        assert!(!i.contains(5.0));
    }

    #[test]
    fn test_surrounds() {
        let i = Interval::new(1.0, 4.0);
        // inside
        assert!(i.surrounds(2.0));
        // boundaries are excluded (open interval)
        assert!(!i.surrounds(1.0));
        assert!(!i.surrounds(4.0));
        // outside
        assert!(!i.surrounds(0.0));
        assert!(!i.surrounds(5.0));
    }

    #[test]
    fn test_clamp() {
        let i = Interval::new(1.0, 4.0);
        // inside the interval -> unchanged
        assert_eq!(i.clamp(2.0), 2.0);
        // below min -> clamped to min
        assert_eq!(i.clamp(-5.0), 1.0);
        // above max -> clamped to max
        assert_eq!(i.clamp(10.0), 4.0);
        // exactly on the boundaries -> unchanged
        assert_eq!(i.clamp(1.0), 1.0);
        assert_eq!(i.clamp(4.0), 4.0);
    }

    #[test]
    fn test_expand() {
        let i = Interval::new(1.0, 4.0);
        // expand by 2.0 on each end (delta/2 = 1.0)
        let expanded = i.expand(2.0);
        assert_eq!(expanded.min, 0.0);
        assert_eq!(expanded.max, 5.0);
        assert_eq!(expanded.size(), 5.0);
    }

    #[test]
    fn test_expand_zero_delta() {
        let i = Interval::new(1.0, 4.0);
        // expand with zero delta should keep the same interval
        let expanded = i.expand(0.0);
        assert_eq!(expanded.min, i.min);
        assert_eq!(expanded.max, i.max);
        assert_eq!(expanded.size(), i.size());
    }

    #[test]
    fn test_expand_negative_delta() {
        let i = Interval::new(1.0, 4.0);
        // negative delta contracts the interval
        let contracted = i.expand(-2.0);
        assert_eq!(contracted.min, 2.0);
        assert_eq!(contracted.max, 3.0);
        assert_eq!(contracted.size(), 1.0);
    }

    #[test]
    fn test_expand_empty() {
        let i = EMPTY;
        let expanded = i.expand(10.0);
        // expanding empty interval: min = +∞ - 5 = +∞, max = -∞ + 5 = -∞
        // still empty
        assert!(expanded.size() < 0.0);
    }

    #[test]
    fn test_expand_universe() {
        let i = UNIVERSE;
        let expanded = i.expand(10.0);
        // expanding infinite interval
        assert_eq!(expanded.min, -INFINITY);
        assert_eq!(expanded.max, INFINITY);
    }
}
