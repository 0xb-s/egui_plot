use crate::Interval;

/// Declarative layout for a "broken" X axis:
/// - `segments` are the visible data ranges, in order.
/// - `gap_px` is the visual gap (in screen points) drawn between them.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BrokenXAxis {
    pub segments: Vec<Interval>,
    pub gap_px: f32,
}

impl BrokenXAxis {
    /// Create and sanitize (sort, drop empties, merge overlaps).
    pub fn new(mut segments: Vec<Interval>, gap_px: f32) -> Self {
        // 1. drop bad/empty/non-finite segments
        segments.retain(|iv| !iv.is_empty());

        // 2. sort by start
        segments.sort_by(|a, b| {
            a.start
                .partial_cmp(&b.start)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // 3. merge overlaps / touching ones so we don't get nonsense
        let mut merged: Vec<Interval> = Vec::new();
        for iv in segments {
            if let Some(last) = merged.last_mut() {
                if iv.start <= last.end {
                    last.end = last.end.max(iv.end);
                    continue;
                }
            }
            merged.push(iv);
        }

        Self {
            segments: merged,
            gap_px,
        }
    }

    /// Return true if we effectively have a broken axis (2+ segments).
    #[inline]
    pub fn is_multi_segment(&self) -> bool {
        self.segments.len() > 1
    }
    pub fn segment_ticks(&self, step_hint: f64) -> Vec<Vec<f64>> {
        let mut out: Vec<Vec<f64>> = Vec::with_capacity(self.segments.len());

        for seg in &self.segments {
            let lo = seg.start;
            let hi = seg.end;
            if !lo.is_finite() || !hi.is_finite() || hi <= lo {
                out.push(Vec::new());
                continue;
            }

            let span = hi - lo;
            let approx_steps = (span / step_hint.max(f64::EPSILON)).max(1.0);
            let raw_step = span / approx_steps;

            let nice = nice_step(raw_step);

            let start_tick = (lo / nice).ceil() * nice;
            let mut ticks = Vec::new();
            let mut t = start_tick;
            while t <= hi + f64::EPSILON {
                ticks.push(t);
                t += nice;
            }

            if !ticks.contains(&lo) {
                ticks.insert(0, lo);
            }
            if !ticks.contains(&hi) {
                ticks.push(hi);
            }

            out.push(ticks);
        }

        out
    }
}

fn nice_step(step: f64) -> f64 {
    let pow10 = 10.0_f64.powf(step.log10().floor());
    let mant = step / pow10;
    let nice_mant = if mant < 1.5 {
        1.0
    } else if mant < 3.5 {
        2.0
    } else if mant < 7.5 {
        5.0
    } else {
        10.0
    };
    nice_mant * pow10
}
