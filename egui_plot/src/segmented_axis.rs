use crate::Interval;

/// Declarative layout for a segmented axis:
/// - `segments` are the visible data ranges, in order.
/// - `gap_px` is the visual gap (in screen points) drawn between them.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SegmentedAxis {
    pub segments: Vec<Interval>,
    pub gap_px: f32,
}

impl SegmentedAxis {
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

    /// Return true if we effectively have a segmented axis (2+ segments).
    #[inline]
    pub fn is_multi_segment(&self) -> bool {
        self.segments.len() > 1
    }
    pub fn segment_ticks(&self, step_hint: f64) -> Vec<Vec<f64>> {
        let mut max_raw_step = 0.0;

        for seg in &self.segments {
            let lo = seg.start;
            let hi = seg.end;

            if !lo.is_finite() || !hi.is_finite() || hi <= lo {
                continue;
            }

            let span = hi - lo;

            let approx_steps = (span / step_hint.max(f64::EPSILON)).max(1.0);
            let raw_step = span / approx_steps;

            if raw_step > max_raw_step {
                max_raw_step = raw_step;
            }
        }

        if max_raw_step == 0.0 {
            return vec![Vec::new(); self.segments.len()];
        }

        let nice = nice_step(max_raw_step);

        let mut out: Vec<Vec<f64>> = Vec::with_capacity(self.segments.len());

        for seg in &self.segments {
            let lo = seg.start;
            let hi = seg.end;

            if !lo.is_finite() || !hi.is_finite() || hi <= lo {
                out.push(Vec::new());
                continue;
            }

            let start_tick = (lo / nice).ceil() * nice;

            let end_tick = (hi / nice).floor() * nice;

            let steps = (((end_tick - start_tick) / nice).round() as i64).max(0);
            let mut ticks = Vec::with_capacity((steps + 3) as usize);

            let mut i = 0i64;
            loop {
                let t = start_tick + (i as f64) * nice;
                if t > hi + f64::EPSILON {
                    break;
                }
                ticks.push(t);
                i += 1;
            }

            if ticks.first().copied() != Some(lo) {
                ticks.insert(0, lo);
            }
            if ticks.last().copied() != Some(hi) {
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
