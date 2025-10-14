#![allow(rustdoc::missing_crate_level_docs)]
use crate::transform::PlotBounds;
use core::fmt;
use core::ops::Range;
use core::ops::{Bound, RangeBounds};

// Segments
#[derive(Clone, Copy, Debug)]
pub struct Segments<'a> {
    pub offsets: &'a [u32],
}

#[derive(Copy, Clone, Debug)]
pub struct SegmentedSeries<'a> {
    pub base: ColumnarSeries<'a>,
    pub segments: Option<Segments<'a>>,
    pub valid: Option<&'a [bool]>,
}

// methods on ColumnarSeries to build a SegmentedSeries:
pub trait ColumnarSeriesExt<'a> {
    fn with_segments(self, offsets: &'a [u32]) -> SegmentedSeries<'a>;
    fn with_mask(self, valid: &'a [bool]) -> SegmentedSeries<'a>;
    fn segmented(
        self,
        offsets: Option<&'a [u32]>,
        valid: Option<&'a [bool]>,
    ) -> SegmentedSeries<'a>;
}

impl<'a> ColumnarSeriesExt<'a> for ColumnarSeries<'a> {
    #[inline]
    fn with_segments(self, offsets: &'a [u32]) -> SegmentedSeries<'a> {
        SegmentedSeries {
            base: self,
            segments: Some(Segments { offsets }),
            valid: None,
        }
    }
    #[inline]
    fn with_mask(self, valid: &'a [bool]) -> SegmentedSeries<'a> {
        SegmentedSeries {
            base: self,
            segments: None,
            valid: Some(valid),
        }
    }
    #[inline]
    fn segmented(
        self,
        offsets: Option<&'a [u32]>,
        valid: Option<&'a [bool]>,
    ) -> SegmentedSeries<'a> {
        SegmentedSeries {
            base: self,
            segments: offsets.map(|o| Segments { offsets: o }),
            valid,
        }
    }
}
impl<'a> SegmentedSeries<'a> {
    #[inline]
    pub fn xs(&self) -> &'a [f64] {
        self.base.xs()
    }
    #[inline]
    pub fn ys(&self) -> &'a [f64] {
        self.base.ys()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.base.len()
    }

    pub fn iter_runs(&self) -> impl Iterator<Item = Range<usize>> + '_ {
        let len = self.len();

        let mut runs = Vec::new();

        let mut i = 0;

        while i < len {
            while i < len && (!self.xs()[i].is_finite() || !self.ys()[i].is_finite()) {
                i += 1;
            }
            let start = i;

            while i < len && self.xs()[i].is_finite() && self.ys()[i].is_finite() {
                i += 1;
            }
            if start < i {
                runs.push(start..i);
            }
        }
        runs.into_iter()
    }
    pub fn bounds(&self) -> PlotBounds {
        let mut b = PlotBounds::NOTHING;
        let (xs, ys) = (self.xs(), self.ys());

        if let Some(mask) = self.valid {
            let m = mask.len().min(xs.len()).min(ys.len());
            for i in 0..m {
                if !mask[i] {
                    continue;
                }
                let (x, y) = (xs[i], ys[i]);
                if x.is_finite() {
                    b.extend_with_x(x);
                }
                if y.is_finite() {
                    b.extend_with_y(y);
                }
            }
        } else {
            let n = xs.len().min(ys.len());
            for i in 0..n {
                let (x, y) = (xs[i], ys[i]);
                if x.is_finite() {
                    b.extend_with_x(x);
                }
                if y.is_finite() {
                    b.extend_with_y(y);
                }
            }
        }
        b
    }
}
/// A zero-copy Series of `(x, y)`.
///
/// This is the canonical way to pass data into plotting items without packing
/// into `PlotPoint` vectors. Multiple series can share the same `xs` slice.
#[derive(Copy, Clone)]
pub struct ColumnarSeries<'a> {
    xs: &'a [f64],
    ys: &'a [f64],
}

impl<'a> ColumnarSeries<'a> {
    /// Construct a columnar series from borrowed `xs` and `ys`.
    ///
    /// # Panics
    /// Panics if `xs.len() != ys.len()`. If you want a version that
    /// *truncates* to the shorter length instead, see [`Self::new_truncating`].
    #[inline]
    pub fn new(xs: &'a [f64], ys: &'a [f64]) -> Self {
        assert!(
            xs.len() == ys.len(),
            "ColumnarSeries::new: xs and ys must have equal length (got {} vs {})",
            xs.len(),
            ys.len()
        );
        Self { xs, ys }
    }

    /// Construct a series by **truncating to the shorter** of `xs` and `ys`.
    ///
    /// This never panics. If the lengths differ, the longer one is sliced down.
    #[inline]
    pub fn new_truncating(xs: &'a [f64], ys: &'a [f64]) -> Self {
        let n = xs.len().min(ys.len());
        Self {
            xs: &xs[..n],
            ys: &ys[..n],
        }
    }

    /// An always-valid empty series.
    pub const EMPTY: ColumnarSeries<'static> = ColumnarSeries { xs: &[], ys: &[] };

    /// Borrow the X slice.
    #[inline]
    pub fn xs(&self) -> &'a [f64] {
        self.xs
    }

    /// Borrow the Y slice.
    #[inline]
    pub fn ys(&self) -> &'a [f64] {
        self.ys
    }

    /// Number of samples.
    ///
    /// `xs.len() == ys.len()` is guaranteed by construction
    #[inline]
    pub fn len(&self) -> usize {
        self.xs.len()
    }

    /// Is the series empty?
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.xs.is_empty()
    }

    /// Get the `(x, y)` at `index`, if in-bounds.
    #[inline]
    pub fn get(&self, index: usize) -> Option<(f64, f64)> {
        if index < self.len() {
            Some((self.xs[index], self.ys[index]))
        } else {
            None
        }
    }

    /// Return an iterator over `(x, y)` pairs (by value).
    #[allow(clippy::iter_without_into_iter)]
    #[inline]
    pub fn iter(&self) -> ColumnarSeriesIter<'a> {
        ColumnarSeriesIter {
            xs: self.xs,
            ys: self.ys,
            i: 0,
        }
    }

    /// Return a **subseries** sliced by element **index** range.
    ///
    /// Accepts any `RangeBounds<usize>`; `Bound::Excluded` and `Bound::Included`
    /// are honored; the result is clamped to `[0, len()]`. Empty ranges return
    /// [`ColumnarSeries::EMPTY`].
    pub fn slice<R>(&self, range: R) -> Self
    where
        R: RangeBounds<usize>,
    {
        let len = self.len();

        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i.saturating_add(1),
        }
        .min(len);

        let end = match range.end_bound() {
            Bound::Unbounded => len,
            Bound::Included(&i) => i.saturating_add(1),
            Bound::Excluded(&i) => i,
        }
        .min(len);

        if end <= start {
            ColumnarSeries::EMPTY
        } else {
            ColumnarSeries {
                xs: &self.xs[start..end],
                ys: &self.ys[start..end],
            }
        }
    }

    /// Estimate numeric bounds over all finite points in the series.
    ///
    /// Non-finite values (`NaN`, `±∞`) are **ignored**. If no finite values
    /// are found, returns `PlotBounds::NOTHING`.
    pub fn bounds(&self) -> PlotBounds {
        let mut b = PlotBounds::NOTHING;

        // Fast path for contiguous slices.
        for i in 0..self.len() {
            let x = self.xs[i];
            let y = self.ys[i];
            if x.is_finite() {
                b.extend_with_x(x);
            }
            if y.is_finite() {
                b.extend_with_y(y);
            }
        }
        b
    }
}

/// Iterator over `(x, y)` pairs in a [`ColumnarSeries`].
pub struct ColumnarSeriesIter<'a> {
    xs: &'a [f64],
    ys: &'a [f64],
    i: usize,
}

impl Iterator for ColumnarSeriesIter<'_> {
    type Item = (f64, f64);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.xs.len() {
            return None;
        }
        let i = self.i;
        self.i += 1;
        Some((self.xs[i], self.ys[i]))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.xs.len().saturating_sub(self.i);
        (n, Some(n))
    }
}

impl ExactSizeIterator for ColumnarSeriesIter<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.xs.len() - self.i
    }
}

impl fmt::Debug for ColumnarSeries<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ColumnarSeries")
            .field("len", &self.len())
            .finish()
    }
}

impl PartialEq for ColumnarSeries<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.xs.len() != other.xs.len() || self.ys.len() != other.ys.len() {
            return false;
        }

        if self.ys != other.ys {
            return false;
        }

        self.xs == other.xs
    }
}

impl Eq for ColumnarSeries<'_> {}

impl<'a> From<(&'a [f64], &'a [f64])> for ColumnarSeries<'a> {
    #[inline]
    fn from(tup: (&'a [f64], &'a [f64])) -> Self {
        Self::new(tup.0, tup.1)
    }
}
