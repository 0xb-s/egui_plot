//! Interval utilities for plot spans,

/// A numeric interval on `R` with optional ±∞ on either side.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Interval {
    /// Lower bound in data units. Can be -∞.
    pub start: f64,
    /// Upper bound in data units. Can be +∞.
    pub end: f64,
}

impl Interval {
    pub fn len(&self) -> f64 {
        let start_inf = self.start.is_infinite();
        let end_inf = self.end.is_infinite();

        if start_inf && end_inf {
            // (-∞, +∞)
            if self.start.is_sign_negative() && self.end.is_sign_positive() {
                return f64::INFINITY;
            }

            // (+∞, +∞) or (-∞, -∞)
            return 0.0;
        }

        if start_inf || end_inf {
            return f64::INFINITY;
        }

        (self.end - self.start).max(0.0)
    }

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
