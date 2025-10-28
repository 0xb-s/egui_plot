use std::ops::RangeInclusive;

use egui::{Color32, Rect, Shape, Stroke, Ui, pos2};

use crate::{
    Interval, PlotBounds, PlotGeometry, PlotItem, PlotItemBase, PlotTransform,
    interval_to_screen_y, span_utils::interval_to_screen_x,
};

/// Horizontal shaded band for a Y interval across full plot width.
#[derive(Clone, Debug, PartialEq)]
pub struct HSpan {
    base: PlotItemBase,

    /// Vertical interval in data space.
    y: Interval,

    /// Fill color of the band
    fill: Color32,

    /// Optional outline stroke around the band. `None` = no outline.
    stroke: Option<Stroke>,

    /// Toggle visibility via code.
    visible: bool,
}

impl HSpan {
    /// Create a horizontal span from an explicit `Interval` in Y.
    pub fn new(name: impl Into<String>, y: Interval) -> Self {
        let default = Color32::from_rgba_unmultiplied(128, 128, 128, 40);
        Self {
            base: PlotItemBase::new(name.into()),
            y,
            fill: default,
            stroke: None,
            visible: true,
        }
    }

    /// Set the fill color
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

impl PlotItem for HSpan {
    fn shapes(&self, _ui: &Ui, transform: &PlotTransform, shapes: &mut Vec<Shape>) {
        if !self.visible {
            return;
        }
        if self.y.is_empty() {
            return;
        }

        let (top, bottom) = interval_to_screen_y(&self.y, transform);

        if (bottom - top).abs() <= f32::EPSILON {
            return;
        }

        let frame = transform.frame();
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

        if self.y.start.is_finite() {
            b.extend_with_y(self.y.start);
        }
        if self.y.end.is_finite() {
            b.extend_with_y(self.y.end);
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
/// Vertical shaded band for an X interval across full plot height.
#[derive(Clone, Debug, PartialEq)]
pub struct VSpan {
    base: PlotItemBase,

    /// Horizontal interval in data space.
    x: Interval,

    /// Fill color of the band (should usually be translucent).
    fill: Color32,

    /// Optional outline stroke around the band. `None` = no outline.
    stroke: Option<Stroke>,

    /// Toggle visibility via code.
    visible: bool,
}

impl VSpan {
    /// Create a vertical span from an explicit `Interval` in X.
    pub fn new(name: impl Into<String>, x: Interval) -> Self {
        let default = Color32::from_rgba_unmultiplied(128, 128, 128, 40);
        Self {
            base: PlotItemBase::new(name.into()),
            x,
            fill: default,
            stroke: None,
            visible: true,
        }
    }
    /// Set the fill color .
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
        if self.x.is_empty() {
            return;
        }

        let (left, right) = interval_to_screen_x(&self.x, transform);

        if (right - left).abs() <= f32::EPSILON {
            return;
        }

        let frame = transform.frame();
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

        if self.x.start.is_finite() {
            b.extend_with_x(self.x.start);
        }
        if self.x.end.is_finite() {
            b.extend_with_x(self.x.end);
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
