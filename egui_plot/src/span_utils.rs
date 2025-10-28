use crate::{Interval, PlotPoint, PlotTransform};
use egui::Rect;

/// Convert a Y-interval in data space to screen-space vertical span.
pub fn interval_to_screen_y(interval: &Interval, tf: &PlotTransform) -> (f32, f32) {
    let frame: Rect = *tf.frame();

    let y0 = if interval.start.is_finite() {
        tf.position_from_point(&PlotPoint::new(0.0, interval.start))
            .y
    } else {
        frame.bottom()
    };

    let y1 = if interval.end.is_finite() {
        tf.position_from_point(&PlotPoint::new(0.0, interval.end)).y
    } else {
        frame.top()
    };

    (y0.min(y1), y0.max(y1))
}

/// Convert an X-interval in data space to screen-space horizontal span.
pub fn interval_to_screen_x(interval: &Interval, tf: &PlotTransform) -> (f32, f32) {
    let frame: Rect = *tf.frame();

    let x0 = if interval.start.is_finite() {
        tf.position_from_point(&PlotPoint::new(interval.start, 0.0))
            .x
    } else {
        frame.left()
    };

    let x1 = if interval.end.is_finite() {
        tf.position_from_point(&PlotPoint::new(interval.end, 0.0)).x
    } else {
        frame.right()
    };

    (x0.min(x1), x0.max(x1))
}
