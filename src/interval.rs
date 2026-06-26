use crate::utils::INFINITY;

/// A simple struct representing a closed interval [min, max] of floating-point numbers.
#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    /// Creates an empty interval (contains no points).
    ///
    /// `min` is `+∞` and `max` is `-∞`, so [`contains`](Interval::contains)
    /// and [`surrounds`](Interval::surrounds) always return `false`.
    pub const fn empty() -> Interval { Interval { min: INFINITY, max: -INFINITY } }

    /// Creates an interval with the given `min` and `max` bounds.
    pub const fn new(min: f64, max: f64) -> Interval { Interval { min, max } }

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
}

/// The empty interval: contains no points.
pub const EMPTY: Interval = Interval::empty();

/// The universe interval: contains every real number.
pub const UNIVERSE: Interval = Interval::new(-INFINITY, INFINITY);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let i = Interval::empty();
        // An empty interval contains nothing.
        assert!(!i.contains(0.0));
        assert!(!i.surrounds(0.0));
        // size is negative (max < min)
        assert!(i.size() < 0.0);
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
}
