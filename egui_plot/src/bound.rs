//! Interval utilities for plot spans,

/// A numeric interval on `R` with optional ±∞ on either side.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
    /// Lower bound in data units. Can be -∞.
    pub start: f64,
    /// Upper bound in data units. Can be +∞.
    pub end: f64,
}

impl Interval {
    /// Create a new interval from two endpoints.
    #[inline]
    pub fn new(a: f64, b: f64) -> Self {
        if a <= b {
            Self { start: a, end: b }
        } else {
            Self { start: b, end: a }
        }
    }

    #[inline]
    pub fn closed(a: f64, b: f64) -> Self {
        Self::new(a, b)
    }

    /// (-∞, b]
    #[inline]
    pub fn below(b: f64) -> Self {
        Self::new(f64::NEG_INFINITY, b)
    }

    /// [a, +∞)
    #[inline]
    pub fn above(a: f64) -> Self {
        Self::new(a, f64::INFINITY)
    }

    /// (-∞, +∞)
    #[inline]
    pub fn all() -> Self {
        Self::new(f64::NEG_INFINITY, f64::INFINITY)
    }

    /// Return true if the interval is effectively empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Returns `true` if the scalar `x` lies within [start, end].
    #[inline]
    pub fn contains(&self, x: f64) -> bool {
        x >= self.start && x <= self.end
    }
}
