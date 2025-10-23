use std::ops::RangeInclusive;

use egui::{Color32, Rect, Shape, Stroke, Ui, pos2};

use crate::{PlotBounds, PlotGeometry, PlotItem, PlotItemBase, PlotPoint, PlotTransform};

/// A horizontal shaded region y ∈ (`y_min`, `y_max`), spanning full plot width.
/// Semi-open ends are supported by passing `None` for either side.
#[derive(Clone, Debug, PartialEq)]
pub struct HSpan {
    base: PlotItemBase,
    /// Lower Y.
    y_min: Option<f64>,
    /// Upper Y.
    y_max: Option<f64>,
    /// Fill color of the band.
    fill: Color32,
    /// Optional outline stroke around the band. `None` = no outline.
    stroke: Option<Stroke>,
    /// Toggle visibility via code.
    visible: bool,
}

impl HSpan {
    /// Create a horizontal span with an optional name and optional bounds.
    /// Use `None` to make the span open-ended on that side.
    pub fn new(name: impl Into<String>, y_min: Option<f64>, y_max: Option<f64>) -> Self {
        let default = Color32::from_rgba_unmultiplied(128, 128, 128, 40);
        Self {
            base: PlotItemBase::new(name.into()),
            y_min,
            y_max,
            fill: default,
            stroke: None,
            visible: true,
        }
    }

    /// Set the fill color.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.fill = color.into();
        self
    }

    /// Optional outline stroke around the span.
    #[inline]
    pub fn outline(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = Some(stroke.into());
        self
    }

    /// Toggle visibility.
    #[inline]
    pub fn visible(mut self, yes: bool) -> Self {
        self.visible = yes;
        self
    }
}

impl PlotItem for HSpan {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        if !self.visible {
            return;
        }

        let frame = transform.frame();

        let map_y = |y_opt: Option<f64>, fallback_edge: f32| -> f32 {
            match y_opt {
                Some(y) if y.is_finite() => {
                    transform.position_from_point(&PlotPoint::new(0.0, y)).y
                }
                _ => fallback_edge,
            }
        };

        let y0 = map_y(self.y_min, frame.top());
        let y1 = map_y(self.y_max, frame.bottom());

        // If the interval collapses or is inverted, do nothing.
        let (top, bottom) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        if (bottom - top).abs() <= f32::EPSILON {
            return;
        }

        let rect = Rect::from_min_max(pos2(frame.left(), top), pos2(frame.right(), bottom));

        shapes.push(Shape::rect_filled(rect, 0.0, self.fill));

        if let Some(stroke) = self.stroke {
            shapes.push(Shape::rect_stroke(
                rect,
                0.0,
                stroke,
                egui::StrokeKind::Outside,
            ));
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn color(&self) -> Color32 {
        self.fill
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut b = PlotBounds::NOTHING;
        if let Some(y0) = self.y_min {
            if y0.is_finite() {
                b.extend_with_y(y0);
            }
        }
        if let Some(y1) = self.y_max {
            if y1.is_finite() {
                b.extend_with_y(y1);
            }
        }
        b
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}

/// A vertical shaded region x ∈ (`x_min`, `x_max`), spanning full plot height.
/// Semi-open ends are supported by passing `None` for either side.
#[derive(Clone, Debug, PartialEq)]
pub struct VSpan {
    base: PlotItemBase,
    /// Left X
    x_min: Option<f64>,
    /// Right X
    x_max: Option<f64>,
    /// Fill color of the band (recommended to be translucent).
    fill: Color32,
    /// Optional outline stroke around the band. `None` = no outline.
    stroke: Option<Stroke>,
    /// Toggle visibility via code.
    visible: bool,
}

impl VSpan {
    /// Create a vertical span with an optional name and optional bounds.
    /// Use `None` to make the span open-ended on that side.
    pub fn new(name: impl Into<String>, x_min: Option<f64>, x_max: Option<f64>) -> Self {
        let default = Color32::from_rgba_unmultiplied(128, 128, 128, 40);
        Self {
            base: PlotItemBase::new(name.into()),
            x_min,
            x_max,
            fill: default,
            stroke: None,
            visible: true,
        }
    }

    /// Set the fill color (include transparency).
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.fill = color.into();
        self
    }

    /// Optional outline stroke around the span.
    #[inline]
    pub fn outline(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = Some(stroke.into());
        self
    }

    /// Toggle visibility (code-controlled show/hide).
    #[inline]
    pub fn visible(mut self, yes: bool) -> Self {
        self.visible = yes;
        self
    }
}

impl PlotItem for VSpan {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        if !self.visible {
            return;
        }

        let frame = transform.frame();

        let map_x = |x_opt: Option<f64>, fallback_edge: f32| -> f32 {
            match x_opt {
                Some(x) if x.is_finite() => {
                    transform.position_from_point(&PlotPoint::new(x, 0.0)).x
                }
                _ => fallback_edge,
            }
        };

        let x0 = map_x(self.x_min, frame.left());
        let x1 = map_x(self.x_max, frame.right());

        let (left, right) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        if (right - left).abs() <= f32::EPSILON {
            return;
        }

        let rect = Rect::from_min_max(pos2(left, frame.top()), pos2(right, frame.bottom()));

        shapes.push(Shape::rect_filled(rect, 0.0, self.fill));

        if let Some(stroke) = self.stroke {
            shapes.push(Shape::rect_stroke(
                rect,
                0.0,
                stroke,
                egui::StrokeKind::Outside,
            ));
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn color(&self) -> Color32 {
        self.fill
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut b = PlotBounds::NOTHING;
        if let Some(x0) = self.x_min {
            if x0.is_finite() {
                b.extend_with_x(x0);
            }
        }
        if let Some(x1) = self.x_max {
            if x1.is_finite() {
                b.extend_with_x(x1);
            }
        }
        b
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}
